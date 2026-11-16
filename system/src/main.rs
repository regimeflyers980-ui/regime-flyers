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
        .route("/", get(|| async { "Regime Flyers System is Online!" }))
        .route("/login", get(handlers::auth::login_form).post(handlers::auth::login))
        .route("/logout", post(handlers::auth::logout))
        // Admin Routes
        .route("/admin", get(handlers::admin::dashboard))
        .route("/admin/houses", post(handlers::admin::add_house))
        .route("/admin/floors", post(handlers::admin::add_floor))
        .route("/admin/tenants", post(handlers::admin::add_tenant))
        .route("/admin/meters", post(handlers::admin::add_meter))
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let addr_str = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let addr: SocketAddr = addr_str.parse()?;
    tracing::info!("🚀 Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
