use axum::{Router, routing::get};
use tracing::{error, info};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use std::sync::Arc;

mod api;
mod application;
mod domain;
mod infrastructure;

use api::{api_routes, create_swagger_routes};
use infrastructure::config::{AppConfig, LoggingConfig};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = AppConfig::default();

    // 根据配置初始化日志
    init_logging(&config.logging);

    // 初始化数据库（基础设施层负责连接和迁移）
    match infrastructure::database::connection::init_database(&config.database).await {
        Ok(_) => {
            info!("Database initialization completed successfully");
        }
        Err(e) => {
            error!("Database initialization failed: {}", e);
            return;
        }
    }

    info!(
        "Starting CA server on {}:{}",
        config.server.host, config.server.port
    );

    let app = Router::new()
        .nest("/api", api_routes())
        .merge(create_swagger_routes())
        .route("/", get(|| async { "CA API Server" }));

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

/// 根据配置初始化日志系统
fn init_logging(logging_config: &LoggingConfig) {
    // 根据配置选择轮转策略
    let rotation = match logging_config.rotation.to_lowercase().as_str() {
        "hourly" => Rotation::HOURLY,
        "minutely" => Rotation::MINUTELY,
        "never" => Rotation::NEVER,
        _ => Rotation::DAILY, // 默认每天轮转
    };

    // 配置文件日志
    let file_appender = RollingFileAppender::new(
        rotation,
        &logging_config.directory,
        &logging_config.filename,
    );

    // 根据配置选择日志格式
    let file_format = if logging_config.format.to_lowercase() == "json" {
        // 对于JSON格式，使用不同的配置方式
        fmt::layer()
            .with_writer(file_appender)
            .with_ansi(false)
            .with_target(false)
            .with_thread_names(true)
            .with_thread_ids(true)
    } else {
        fmt::layer()
            .with_writer(file_appender)
            .with_ansi(false)
            .with_target(false)
            .with_thread_names(true)
            .with_thread_ids(true)
    };

    // 创建控制台日志层
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_target(true)
        .pretty();

    // 设置日志级别过滤器
    let env_filter = tracing_subscriber::EnvFilter::new(&logging_config.level);

    // 注册日志订阅者
    Registry::default()
        .with(console_layer)
        .with(file_format)
        .with(env_filter)
        .init();
}
