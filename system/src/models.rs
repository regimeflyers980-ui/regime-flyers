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

// Used for the dropdown in the Admin Dashboard
#[derive(FromRow, Serialize)]
pub struct FloorOption {
    pub id: i32,
    pub house_number: String,
    pub floor_no: i32,
}

// Used to display the meter AND the tenant's name together
#[derive(FromRow, Serialize)]
pub struct WaterMeterView {
    pub id: i32,
    pub meter_number: String,
    pub tenant_name: Option<String>,
}

#[derive(Deserialize)]
pub struct MeterForm {
    pub tenant_id: Uuid,
    pub meter_number: String,
}

// Represents a single row in the Master table
#[derive(FromRow, Serialize)]
pub struct FullPropertyView {
    pub house_number: String,
    pub address: Option<String>,
    pub floor_no: Option<i32>,
    pub capacity: Option<i32>,
    pub tenant_name: Option<String>,
    pub phone: Option<String>,
    pub meter_number: Option<String>,
}
