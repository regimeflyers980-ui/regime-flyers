use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Serialize)]
struct DemoResponse {
    system: String,
    status: String,
    demo_url: String,
}

async fn list_demos() -> Json<Vec<DemoResponse>> {
    let demos = vec![
        DemoResponse {
            system: "Inventory Management".into(),
            status: "active".into(),
            demo_url: "/demos/inventory".into(),
        },
        DemoResponse {
            system: "Client Portal".into(),
            status: "beta".into(),
            demo_url: "/demos/client-portal".into(),
        },
    ];
    Json(demos)
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // For development, we allow all origins. We will lock this down for production later.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/demos", get(list_demos))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();
    tracing::info!("Backend listening on port 3001");
    axum::serve(listener, app).await.unwrap();
}
