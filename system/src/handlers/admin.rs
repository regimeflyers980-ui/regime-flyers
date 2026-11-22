use crate::auth::{hash_password, require_admin, AppError, AuthUser};
use crate::models::{
    ConfirmOcrForm, FloorForm, FloorOption, FullPropertyView, House, HouseForm, MeterForm,
    MeterOption, ReadingForm, SettingsForm, Tenant, TenantForm,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Redirect},
    Form,
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "admin_dashboard.html")]
pub struct DashboardTemplate {
    pub houses: Vec<House>,
    pub floor_options: Vec<FloorOption>,
    pub tenants: Vec<Tenant>,
    pub meter_options: Vec<MeterOption>,
    pub master_table: Vec<FullPropertyView>,
    pub water_rate: String, // NEW
}

pub async fn dashboard(
    user: AuthUser,
    State(pool): State<PgPool>,
) -> Result<Html<String>, AppError> {
    require_admin(&user)?;

    let houses: Vec<House> = sqlx::query_as("SELECT id, house_number, address FROM houses ORDER BY house_number")
        .fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let floor_options: Vec<FloorOption> = sqlx::query_as(
        "SELECT f.id, h.house_number, f.floor_no FROM floors f JOIN houses h ON f.house_id = h.id ORDER BY h.house_number, f.floor_no"
    ).fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tenants: Vec<Tenant> = sqlx::query_as(
        "SELECT id, floor_id, full_name, phone FROM tenants ORDER BY full_name"
    ).fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let meter_options: Vec<MeterOption> = sqlx::query_as(
        "SELECT m.id, m.meter_number, t.full_name as tenant_name
         FROM water_meters m
         LEFT JOIN tenants t ON m.tenant_id = t.id
         ORDER BY m.meter_number"
    ).fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let master_table: Vec<FullPropertyView> = sqlx::query_as(
        "SELECT 
            t.id as tenant_id,
            t.user_id,
            h.house_number, 
            h.address, 
            f.floor_no, 
            f.capacity,
            t.full_name as tenant_name, 
            t.phone, 
            m.meter_number,
            r.previous_units::float8 as previous_units,
            r.current_units::float8 as current_units,
            r.units_used::float8 as units_used
         FROM houses h
         LEFT JOIN floors f ON h.id = f.house_id
         LEFT JOIN tenants t ON f.id = t.floor_id
         LEFT JOIN water_meters m ON t.id = m.tenant_id
         LEFT JOIN LATERAL (
             SELECT previous_units, current_units, units_used
             FROM meter_readings
             WHERE meter_id = m.id
             ORDER BY reading_date DESC LIMIT 1
         ) r ON true
         ORDER BY h.house_number, f.floor_no, t.full_name"
    ).fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // NEW: Fetch the water rate
    let water_rate: String = sqlx::query_scalar("SELECT value FROM settings WHERE key = 'water_rate'")
        .fetch_one(&pool)
        .await
        .unwrap_or_else(|_| "1.50".to_string());

    let tpl = DashboardTemplate { 
        houses, floor_options, tenants, meter_options, master_table, water_rate 
    };
    Ok(Html(tpl.render().unwrap()))
}

pub async fn update_settings(
    user: AuthUser, State(pool): State<PgPool>, Form(form): Form<SettingsForm>,
) -> Result<Redirect, AppError> {
    require_admin(&user)?;
    
    sqlx::query("UPDATE settings SET value = $1 WHERE key = 'water_rate'")
        .bind(&form.water_rate)
        .execute(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok(Redirect::to("/admin"))
}

pub async fn add_house(
    user: AuthUser, State(pool): State<PgPool>, Form(form): Form<HouseForm>,
) -> Result<Redirect, AppError> {
    require_admin(&user)?;
    sqlx::query("INSERT INTO houses (house_number, address) VALUES ($1, $2)")
        .bind(&form.house_number).bind(&form.address).execute(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/admin"))
}

pub async fn add_floor(
    user: AuthUser, State(pool): State<PgPool>, Form(form): Form<FloorForm>,
) -> Result<Redirect, AppError> {
    require_admin(&user)?;
    sqlx::query("INSERT INTO floors (house_id, floor_no, capacity) VALUES ($1, $2, $3)")
        .bind(form.house_id).bind(form.floor_no).bind(form.capacity).execute(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/admin"))
}

pub async fn add_tenant(
    user: AuthUser, State(pool): State<PgPool>, Form(form): Form<TenantForm>,
) -> Result<Redirect, AppError> {
    require_admin(&user)?;
    sqlx::query("INSERT INTO tenants (floor_id, full_name, phone) VALUES ($1, $2, $3)")
        .bind(form.floor_id).bind(&form.full_name).bind(&form.phone).execute(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/admin"))
}

pub async fn add_meter(
    user: AuthUser, State(pool): State<PgPool>, Form(form): Form<MeterForm>,
) -> Result<Redirect, AppError> {
    require_admin(&user)?;
    sqlx::query("INSERT INTO water_meters (meter_number, tenant_id) VALUES ($1, $2)")
        .bind(&form.meter_number).bind(form.tenant_id).execute(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/admin"))
}

pub async fn add_reading(
    user: AuthUser, State(pool): State<PgPool>, Form(form): Form<ReadingForm>,
) -> Result<Redirect, AppError> {
    require_admin(&user)?;
    
    let result = sqlx::query(
        "INSERT INTO meter_readings (meter_id, previous_units, current_units, source, created_by) VALUES ($1, $2, $3, 'MANUAL', $4)"
    )
    .bind(form.meter_id).bind(form.previous_units).bind(form.current_units).bind(user.id)
    .execute(&pool).await;

    if let Err(e) = result {
        tracing::error!("Failed to add reading: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
    }
    
    Ok(Redirect::to("/admin"))
}

pub async fn confirm_ocr_reading(
    user: AuthUser, State(pool): State<PgPool>, Form(form): Form<ConfirmOcrForm>,
) -> Result<Redirect, AppError> {
    require_admin(&user)?;
    
    let result = sqlx::query(
        "INSERT INTO meter_readings (meter_id, previous_units, current_units, source, image_path, created_by) 
         VALUES ($1, $2, $3, 'OCR_SCAN', $4, $5)"
    )
    .bind(form.meter_id)
    .bind(form.previous_units)
    .bind(form.current_units)
    .bind(&form.image_path)
    .bind(user.id)
    .execute(&pool).await;

    if let Err(e) = result {
        tracing::error!("Failed to save OCR reading: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
    }
    
    Ok(Redirect::to("/admin"))
}

pub async fn delete_tenant(
    user: AuthUser, State(pool): State<PgPool>, Path(tenant_id): Path<Uuid>,
) -> Result<Redirect, AppError> {
    require_admin(&user)?;
    sqlx::query("DELETE FROM tenants WHERE id = $1")
        .bind(tenant_id).execute(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/admin"))
}

pub async fn create_tenant_account(
    user: AuthUser, State(pool): State<PgPool>, Path(tenant_id): Path<Uuid>,
) -> Result<Redirect, AppError> {
    require_admin(&user)?;
    let tenant: (String, Option<String>) = sqlx::query_as("SELECT full_name, phone FROM tenants WHERE id = $1")
        .bind(tenant_id).fetch_one(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let email = format!("{}@tenant.regime", tenant.0.to_lowercase().replace(" ", ""));
    let default_password = "Tenant123!";
    let password_hash = hash_password(default_password);

    let user_id: Uuid = sqlx::query_scalar(
        "INSERT INTO users (email, password_hash, role, full_name) VALUES ($1, $2, 'TENANT', $3) RETURNING id"
    ).bind(&email).bind(&password_hash).bind(&tenant.0).fetch_one(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("UPDATE tenants SET user_id = $1 WHERE id = $2")
        .bind(user_id).bind(tenant_id).execute(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/admin"))
}

pub async fn notify_tenant(
    user: AuthUser, State(pool): State<PgPool>, Path(tenant_id): Path<Uuid>,
) -> Result<Redirect, AppError> {
    require_admin(&user)?;

    let data: Option<(String, Option<String>, Option<String>, Option<f64>, Option<f64>, Option<f64>)> = sqlx::query_as(
        "SELECT 
            t.full_name, 
            t.phone, 
            m.meter_number,
            r.previous_units::float8, 
            r.current_units::float8, 
            r.units_used::float8
         FROM tenants t
         LEFT JOIN water_meters m ON t.id = m.tenant_id
         LEFT JOIN LATERAL (
             SELECT previous_units, current_units, units_used
             FROM meter_readings
             WHERE meter_id = m.id
             ORDER BY reading_date DESC LIMIT 1
         ) r ON true
         WHERE t.id = $1"
    )
    .bind(tenant_id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((name, phone, meter, prev, curr, used)) = data {
        if let (Some(m), Some(p), Some(c), Some(u)) = (meter, prev, curr, used) {
            tracing::info!("==================================================");
            tracing::info!("📧 SENDING NOTIFICATION TO CLIENT");
            tracing::info!("To: {} (Phone: {})", name, phone.unwrap_or("N/A".into()));
            tracing::info!("Meter: {}", m);
            tracing::info!("Previous: {} | Current: {} | Units Used: {}", p, c, u);
            tracing::info!("Message: Hello {}, your water consumption for meter {} is {} units.", name, m, u);
            tracing::info!("==================================================");
        } else {
            tracing::warn!("Cannot notify {}: No meter or reading data available.", name);
        }
    }

    Ok(Redirect::to("/admin"))
}
