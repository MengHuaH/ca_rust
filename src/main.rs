use axum::{Router, routing::get};
use sea_orm::Database;
use std::env;
use std::sync::Arc;
use tracing::info;

mod api;
mod application;
mod domain;
mod infrastructure;
mod repositories;

use api::{create_api_routes, create_swagger_routes};
use infrastructure::config::AppConfig;
use infrastructure::database::migration::DatabaseMigration;

#[tokio::main]
async fn main() {
    // 使用简单的日志配置避免编码问题
    tracing_subscriber::fmt().init();

    dotenvy::dotenv().ok();

    let config = AppConfig::default();

    info!(
        "Starting CA server on {}:{}",
        config.server.host, config.server.port
    );

    // 测试数据库迁移
    info!("Testing database migration...");

    let database_url = env::var("DB_URL")
        .unwrap_or("postgres://postgres:password@localhost:5432/DB_default".to_string());

    info!("Using database URL: {}", database_url);

    match Database::connect(database_url).await {
        Ok(db) => {
            info!("Database connection successful");

            // 测试迁移状态
            match DatabaseMigration::get_migration_status(&db).await {
                Ok(status) => {
                    info!(
                        "Migration status: total={}, applied={}, pending={}",
                        status.total_migrations,
                        status.applied_migrations,
                        status.pending_migrations
                    );
                }
                Err(e) => {
                    info!("Failed to get migration status: {}", e);
                }
            }

            // 应用迁移
            match DatabaseMigration::migrate_up(&db).await {
                Ok(applied) => {
                    if applied.is_empty() {
                        info!("No migrations to apply");
                    } else {
                        info!(
                            "Successfully applied {} migrations: {:?}",
                            applied.len(),
                            applied
                        );
                    }
                }
                Err(e) => {
                    info!("Migration failed: {}", e);
                }
            }
        }
        Err(e) => {
            info!("Database connection failed: {}", e);
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
