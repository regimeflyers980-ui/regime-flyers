mod models;

use axum::{routing::get, Router};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load Environment Variables
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // 2. Connect to Database
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    let pool = PgPool::connect(&database_url).await?;

    // 3. Run Database Migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 4. Build Router
    let app = Router::new()
        .route("/", get(root))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // 5. Start Server
    let addr_str = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let addr: SocketAddr = addr_str.parse()?;
    tracing::info!("🚀 Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Basic health check route
async fn root() -> &'static str {
    "Regime Flyers System is Online!"
}
