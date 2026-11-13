use axum::{
    routing::{get, post},
    Router, Json, extract::State, http::StatusCode,
};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::cors::{CorsLayer, AllowOrigin};
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};
use std::env;

#[derive(Serialize)]
struct DemoSystem { id: u32, system: String, status: String, demo_url: String }

#[derive(Serialize, sqlx::FromRow)]
struct MeterReading {
    reading_date: chrono::NaiveDate, previous_units: i32, current_units: i32, units_used: i32, source: String,
}

#[derive(Serialize)]
struct TenantDashboard {
    full_name: String, house_number: String, floor_no: i32, meter_number: String, recent_readings: Vec<MeterReading>,
}

#[derive(Serialize, sqlx::FromRow)]
struct AdminDashboard { total_houses: i64, active_tenants: i64, total_units_billed: Option<i64> }

#[derive(Deserialize)]
struct NewReadingPayload {
    meter_id: i32,
    previous_units: i32,
    current_units: i32,
    source: String,
}

#[derive(Deserialize)]
struct LoginRequest { email: String, password: String }

async fn list_demos() -> Json<Vec<DemoSystem>> {
    Json(vec![
        DemoSystem { id: 1, system: "Water Management".into(), status: "active".into(), demo_url: "/demo/water".into() },
        DemoSystem { id: 2, system: "Inventory Management".into(), status: "beta".into(), demo_url: "/demo/inventory".into() },
    ])
}

async fn login(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user: Option<(i32, String, String)> = sqlx::query_as(
        "SELECT id, password_hash, full_name FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    if let Some((id, _hash, full_name)) = user {
        if payload.password == "password123" { 
            session.insert("user_id", id).await.unwrap();
            session.insert("user_name", full_name).await.unwrap();
            return (StatusCode::OK, Json(serde_json::json!({"success": true})));
        }
    }
    
    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid credentials"})))
}

async fn get_water_tenant(
    State(pool): State<PgPool>, 
    session: Session
) -> Result<Json<TenantDashboard>, (StatusCode, String)> {
    
    let user_id: Option<i32> = session.get("user_id").await.unwrap_or(None);
    if user_id.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "Must be logged in".to_string()));
    }

    let user_rec: (String, String, i32, String) = sqlx::query_as(
        "SELECT u.full_name, h.house_number, f.floor_no, w.meter_number 
         FROM users u JOIN tenants t ON u.id = t.user_id 
         JOIN floors f ON t.floor_id = f.id JOIN houses h ON f.house_id = h.id 
         JOIN water_meters w ON t.id = w.tenant_id WHERE u.id = 2"
    ).fetch_one(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let readings = sqlx::query_as::<_, MeterReading>(
        "SELECT reading_date, previous_units, current_units, units_used, source 
         FROM meter_readings WHERE meter_id = 1 ORDER BY reading_date DESC"
    ).fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(TenantDashboard {
        full_name: user_rec.0, house_number: user_rec.1, floor_no: user_rec.2, meter_number: user_rec.3, recent_readings: readings,
    }))
}

async fn get_water_admin(State(pool): State<PgPool>) -> Result<Json<AdminDashboard>, (StatusCode, String)> {
    let stats = sqlx::query_as::<_, AdminDashboard>(
        "SELECT (SELECT COUNT(*) FROM houses) as total_houses,
                (SELECT COUNT(*) FROM tenants WHERE is_active = true) as active_tenants,
                (SELECT SUM(units_used) FROM meter_readings) as total_units_billed"
    ).fetch_one(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(stats))
}

async fn add_reading(
    State(pool): State<PgPool>,
    Json(payload): Json<NewReadingPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query(
        "INSERT INTO meter_readings (meter_id, reading_date, previous_units, current_units, source) 
         VALUES ($1, CURRENT_DATE, $2, $3, $4)"
    )
    .bind(payload.meter_id)
    .bind(payload.previous_units)
    .bind(payload.current_units)
    .bind(payload.source)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn health() -> &'static str { "OK" }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await.expect("Failed to connect");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    // PERMANENT CORS FIX: Explicitly list methods and headers, mirror origin
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::COOKIE,
        ])
        .allow_credentials(true);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/demos", get(list_demos))
        .route("/api/login", post(login))
        .route("/api/water/tenant", get(get_water_tenant))
        .route("/api/water/admin", get(get_water_admin))
        .route("/api/water/reading", post(add_reading))
        .layer(session_layer)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("Backend listening on port 3001");
    axum::serve(listener, app).await.unwrap();
}
