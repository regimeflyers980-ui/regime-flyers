mod auth;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(7)));

    let app = Router::new()
        .route("/", get(handlers::public::index))
        .route("/login", get(handlers::auth::login_form).post(handlers::auth::login))
        .route("/logout", post(handlers::auth::logout))
        // Admin Routes
        .route("/admin", get(handlers::admin::dashboard))
        .route("/admin/houses", post(handlers::admin::add_house))
        .route("/admin/floors", post(handlers::admin::add_floor))
        .route("/admin/tenants", post(handlers::admin::add_tenant))
        .route("/admin/meters", post(handlers::admin::add_meter))
        .route("/admin/readings", post(handlers::admin::add_reading))
        .route("/admin/tenants/:id/delete", post(handlers::admin::delete_tenant))
        .route("/admin/tenants/:id/create-account", post(handlers::admin::create_tenant_account))
        // Client Portal
        .route("/portal", get(handlers::portal::dashboard))
  // Add these inside Router::new():
        .route("/admin/ocr", post(handlers::ocr::scan_meter))
        .route("/admin/tenants/:id/notify", post(handlers::admin::notify_tenant))
        .route("/admin/settings", post(handlers::admin::update_settings))
        .route("/admin/ocr/confirm", post(handlers::admin::confirm_ocr_reading))
        // Add this to serve the uploaded images and any CSS:
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let addr_str = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());
    let addr: SocketAddr = addr_str.parse()?;
    tracing::info!("🚀 Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
