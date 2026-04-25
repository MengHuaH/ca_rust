use tracing::{error, info};
use tracing_subscriber;

use CA::infrastructure::database::connection::init_database_migration_only;

use dotenvy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("Warning: Could not load .env file: {}", e);
    }

    // 初始化日志
    tracing_subscriber::fmt::init();

    info!("Starting database migration tool...");

    // 加载数据库配置
    let app_config: CA::infrastructure::config::AppConfig =
        CA::infrastructure::config::AppConfig::default();
    info!("App config loaded successfully");
    let database_config = app_config.database;
    info!(
        "Database config extracted: host={}, port={}, database={}",
        database_config.host, database_config.port, database_config.database
    );

    // 初始化数据库连接并执行迁移（严格模式）
    match init_database_migration_only(&database_config).await {
        Ok(()) => {
            info!("Database migration completed successfully!");
            println!("Database migration completed successfully!");
        }
        Err(e) => {
            error!("Database migration failed: {}", e);
            eprintln!("Database migration failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
