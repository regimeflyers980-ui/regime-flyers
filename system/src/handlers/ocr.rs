use crate::auth::{require_admin, AppError, AuthUser};
use crate::models::OcrResult;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use image::{DynamicImage, ImageBuffer, Luma};
use rusty_tesseract::{Args, Image as TesseractImage};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn scan_meter(
    user: AuthUser,
    State(_pool): State<PgPool>,
    mut multipart: Multipart,
) -> Result<Json<OcrResult>, AppError> {
    require_admin(&user)?;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart error: {:?}", e);
        StatusCode::BAD_REQUEST
    })? {
        if field.name() == Some("meter_image") {
            let data = field.bytes().await.map_err(|e| {
                tracing::error!("Failed to read bytes: {:?}", e);
                StatusCode::BAD_REQUEST
            })?;
            
            // 1. Decode the image
            let img = image::load_from_memory(&data).map_err(|e| {
                tracing::error!("Image decode error: {:?}", e);
                StatusCode::BAD_REQUEST
            })?;
            
            // 2. Preprocess for better OCR accuracy
            let processed = preprocess_image(img);

            // 3. Save to disk for audit trail
            let file_id = Uuid::new_v4();
            let filename = format!("scan_{}.png", file_id);
            let path = format!("static/uploads/{}", filename);
            
            std::fs::create_dir_all("static/uploads").map_err(|e| {
                tracing::error!("Failed to create upload directory: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            
            processed.save(&path).map_err(|e| {
                tracing::error!("Failed to save image: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            // 4. Run Tesseract OCR
            let args = Args {
                lang: "eng".to_string(),
                psm: Some(6), // Assume single uniform block of text
                ..Default::default()
            };

            let tesseract_img = TesseractImage::from_dynamic_image(&processed).map_err(|e| {
                tracing::error!("Tesseract image conversion error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            
            let text = rusty_tesseract::image_to_string(&tesseract_img, &args).map_err(|e| {
                tracing::error!("Tesseract OCR error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            // 5. Extract numbers from raw text
            let detected_number = extract_number(&text);

            // Return to the frontend for confirmation
            return Ok(Json(OcrResult {
                raw_text: text,
                detected_number,
                image_path: path,
                meter_id: 0,
            }));
        }
    }
    Err(StatusCode::BAD_REQUEST.into())
}

fn preprocess_image(img: DynamicImage) -> DynamicImage {
    let gray = img.to_luma8();
    let resized = image::imageops::resize(
        &gray,
        gray.width() / 2,
        gray.height() / 2,
        image::imageops::FilterType::Lanczos3,
    );

    let mut binarized = ImageBuffer::new(resized.width(), resized.height());
    for (x, y, pixel) in resized.enumerate_pixels() {
        let val = if pixel[0] > 128 { 255 } else { 0 };
        binarized.put_pixel(x, y, Luma([val]));
    }

    DynamicImage::ImageLuma8(binarized)
}

fn extract_number(text: &str) -> Option<f64> {
    let re = regex::Regex::new(r"(\d+[.,]?\d*)").ok()?;
    re.find(text)
        .and_then(|m| m.as_str().replace(',', ".").parse::<f64>().ok())
}

