use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Serialize)]
struct DemoSystem {
    id: u32,
    system: String,
    status: String,
    demo_url: String,
}

#[derive(Serialize)]
struct MeterReading {
    reading_date: String,
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

#[derive(Serialize)]
struct AdminDashboard {
    total_houses: u32,
    active_tenants: u32,
    total_units_billed: i32,
    recent_scans: Vec<String>,
}

async fn list_demos() -> Json<Vec<DemoSystem>> {
    let demos = vec![
        DemoSystem {
            id: 1,
            system: "Water Management".into(),
            status: "active".into(),
            demo_url: "/demo/water".into(),
        },
        DemoSystem {
            id: 2,
            system: "Inventory Management".into(),
            status: "beta".into(),
            demo_url: "/demo/inventory".into(),
        },
    ];
    Json(demos)
}

async fn get_water_tenant() -> Json<TenantDashboard> {
    let dashboard = TenantDashboard {
        full_name: "John Doe".into(),
        house_number: "A-12".into(),
        floor_no: 2,
        meter_number: "WM-99821".into(),
        recent_readings: vec![
            MeterReading {
                reading_date: "2026-10-01".into(),
                previous_units: 120,
                current_units: 145,
                units_used: 25,
                source: "OCR_SCAN".into(),
            },
            MeterReading {
                reading_date: "2026-09-01".into(),
                previous_units: 100,
                current_units: 120,
                units_used: 20,
                source: "MANUAL".into(),
            },
        ],
    };
    Json(dashboard)
}

async fn get_water_admin() -> Json<AdminDashboard> {
    let dashboard = AdminDashboard {
        total_houses: 15,
        active_tenants: 42,
        total_units_billed: 3450,
        recent_scans: vec!["WM-99821 (John Doe)".into(), "WM-99822 (Jane Smith)".into()],
    };
    Json(dashboard)
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/demos", get(list_demos))
        .route("/api/water/tenant", get(get_water_tenant))
        .route("/api/water/admin", get(get_water_admin))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();
    tracing::info!("Backend listening on port 3001");
    axum::serve(listener, app).await.unwrap();
}
