use axum::{Router, routing::get};
use tracing::info;

use std::sync::Arc;

mod api;
mod application;
mod domain;
mod infrastructure;
mod repositories;

use api::{create_api_routes, create_swagger_routes};
use infrastructure::config::AppConfig;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("VisualEngine=info")
        .init();

    let config = AppConfig::default();

    info!(
        "Starting VisualEngine server on {}:{}",
        config.server.host, config.server.port
    );

    let app = Router::new()
        .nest("/api", create_api_routes())
        .merge(create_swagger_routes())
        .route("/", get(|| async { "VisualEngine API Server" }));

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
            .await
            .expect("Failed to bind to address");

    info!(
        "Server running at http://{}:{}",
        config.server.host, config.server.port
    );

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
