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
    // 使用简单的日志配置避免编码问题
    tracing_subscriber::fmt().init();

    dotenvy::dotenv().ok();

    let config = AppConfig::default();

    info!(
        "Starting VisualEngine server on {}:{}",
        config.server.host, config.server.port
    );

    // 初始化数据库（基础设施层负责连接和迁移）
    match infrastructure::database::connection::init_database(&config.database).await {
        Ok(_) => {
            info!("Database initialization completed successfully");
        }
        Err(e) => {
            info!("Database initialization failed: {}", e);
        }
    }

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
