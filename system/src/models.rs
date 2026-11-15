use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct MeterReading {
    pub id: i32,
    pub meter_id: i32,
    pub reading_date: DateTime<Utc>,
    pub previous_units: f64,
    pub current_units: f64,
    pub units_used: f64,
    pub source: String,
    pub image_path: Option<String>,
    pub is_billed: bool,
    pub created_by: Option<Uuid>,
}
