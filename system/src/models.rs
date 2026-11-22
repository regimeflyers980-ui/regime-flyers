use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(FromRow, Serialize)]
pub struct House {
    pub id: i32,
    pub house_number: String,
    pub address: Option<String>,
}

#[derive(Deserialize)]
pub struct HouseForm {
    pub house_number: String,
    pub address: String,
}

#[derive(Deserialize)]
pub struct FloorForm {
    pub house_id: i32,
    pub floor_no: i32,
    pub capacity: i32,
}

#[derive(FromRow, Serialize)]
pub struct Tenant {
    pub id: Uuid,
    pub floor_id: i32,
    pub full_name: String,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct TenantForm {
    pub floor_id: i32,
    pub full_name: String,
    pub phone: String,
}

#[derive(FromRow, Serialize)]
pub struct FloorOption {
    pub id: i32,
    pub house_number: String,
    pub floor_no: i32,
}

#[derive(Deserialize)]
pub struct MeterForm {
    pub tenant_id: Uuid,
    pub meter_number: String,
}

#[derive(FromRow, Serialize)]
pub struct MeterOption {
    pub id: i32,
    pub meter_number: String,
    pub tenant_name: Option<String>,
}

#[derive(Deserialize)]
pub struct ReadingForm {
    pub meter_id: i32,
    pub previous_units: f64,
    pub current_units: f64,
}

#[derive(FromRow, Serialize)]
pub struct FullPropertyView {
    pub tenant_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub house_number: String,
    pub address: Option<String>,
    pub floor_no: Option<i32>,
    pub capacity: Option<i32>,
    pub tenant_name: Option<String>,
    pub phone: Option<String>,
    pub meter_number: Option<String>,
    pub previous_units: Option<f64>,
    pub current_units: Option<f64>,
    pub units_used: Option<f64>,
}

#[derive(FromRow, Serialize)]
pub struct PortalReading {
    pub meter_number: String,
    pub reading_date: DateTime<Utc>,
    pub previous_units: f64,
    pub current_units: f64,
    pub units_used: f64,
}

#[derive(Serialize, Deserialize)]
pub struct OcrResult {
    pub raw_text: String,
    pub detected_number: Option<f64>,
    pub image_path: String,
    pub meter_id: i32,
}

#[derive(Deserialize)]
pub struct ConfirmOcrForm {
    pub meter_id: i32,
    pub current_units: f64,
    pub image_path: String,
    pub previous_units: f64,
}

// --- NEW SETTINGS STRUCTS ---
#[allow(dead_code)]
#[derive(FromRow, Serialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct SettingsForm {
    pub water_rate: String,
}
