use axum::{
    routing::get,
    Router,
    Json,
    extract::State,
    http::StatusCode,
};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use std::env;

#[derive(Serialize)]
struct DemoSystem {
    id: u32,
    system: String,
    status: String,
    demo_url: String,
}

#[derive(Serialize, sqlx::FromRow)]
struct MeterReading {
    reading_date: chrono::NaiveDate,
    previous_units: i32,
    current_units: i32,
    units_used: i32,
    source: String,
}

#[derive(Serialize)]
struct TenantDashboard {
    full_name: String,
    house_number: String,
    floor_no: i32,
    meter_number: String,
    recent_readings: Vec<MeterReading>,
}

#[derive(Serialize, sqlx::FromRow)]
struct AdminDashboard {
    total_houses: i64,
    active_tenants: i64,
    total_units_billed: Option<i64>,
}

async fn list_demos() -> Json<Vec<DemoSystem>> {
    let demos = vec![
        DemoSystem { id: 1, system: "Water Management".into(), status: "active".into(), demo_url: "/demo/water".into() },
        DemoSystem { id: 2, system: "Inventory Management".into(), status: "beta".into(), demo_url: "/demo/inventory".into() },
    ];
    Json(demos)
}

async fn get_water_tenant(State(pool): State<PgPool>) -> Result<Json<TenantDashboard>, (StatusCode, String)> {
    let user_rec: (String, String, i32, String) = sqlx::query_as(
        "SELECT u.full_name, h.house_number, f.floor_no, w.meter_number 
         FROM users u 
         JOIN tenants t ON u.id = t.user_id 
         JOIN floors f ON t.floor_id = f.id 
         JOIN houses h ON f.house_id = h.id 
         JOIN water_meters w ON t.id = w.tenant_id 
         WHERE u.id = 2"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let readings = sqlx::query_as::<_, MeterReading>(
        "SELECT reading_date, previous_units, current_units, units_used, source 
         FROM meter_readings 
         WHERE meter_id = 1 
         ORDER BY reading_date DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(TenantDashboard {
        full_name: user_rec.0,
        house_number: user_rec.1,
        floor_no: user_rec.2,
        meter_number: user_rec.3,
        recent_readings: readings,
    }))
}

async fn get_water_admin(State(pool): State<PgPool>) -> Result<Json<AdminDashboard>, (StatusCode, String)> {
    let stats = sqlx::query_as::<_, AdminDashboard>(
        "SELECT 
            (SELECT COUNT(*) FROM houses) as total_houses,
            (SELECT COUNT(*) FROM tenants WHERE is_active = true) as active_tenants,
            (SELECT SUM(units_used) FROM meter_readings) as total_units_billed"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(stats))
}

async fn health() -> &'static str { "OK" }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut attempts = 0;
    
    let pool = loop {
        match PgPoolOptions::new().max_connections(5).connect(&db_url).await {
            Ok(pool) => break pool,
            Err(e) => {
                attempts += 1;
                if attempts >= 10 { panic!("Failed to connect: {}", e); }
                tracing::warn!("DB not ready ({}/10). Retrying in 2s...", attempts);
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    };

    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/demos", get(list_demos))
        .route("/api/water/tenant", get(get_water_tenant))
        .route("/api/water/admin", get(get_water_admin))
        .with_state(pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("Backend listening on port 3001");
    axum::serve(listener, app).await.unwrap();
}
