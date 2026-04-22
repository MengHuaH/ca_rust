use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell;
use tracing::{info, warn};

use crate::infrastructure::config::DatabaseConfig;
use crate::infrastructure::database::migration::DatabaseMigration;

static DB_CONNECTION: OnceCell<Arc<DatabaseConnection>> = OnceCell::const_new();

/// 提取数据库名称
fn extract_database_name(url: &str) -> Option<String> {
    // 从URL中提取数据库名称
    // 格式：postgres://user:password@host:port/database_name
    url.split('/').last().map(|s| {
        // 移除可能的查询参数并转换为小写
        // PostgreSQL对数据库名称大小写敏感，默认转换为小写
        s.split('?').next().unwrap_or(s).to_lowercase()
    })
}

/// 尝试连接到默认数据库（通常是 postgres）来创建目标数据库
async fn create_database_if_not_exists(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let database_name = extract_database_name(url);

    if let Some(db_name) = database_name {
        // 构建连接到默认数据库的 URL
        let default_db_url = url.replace(&format!("/{}", db_name), "/postgres");

        // 尝试连接到默认数据库
        match Database::connect(&default_db_url).await {
            Ok(db) => {
                // 检查目标数据库是否存在
                let check_sql = format!("SELECT 1 FROM pg_database WHERE datname = '{}'", db_name);

                match db
                    .execute(sea_orm::Statement::from_string(
                        db.get_database_backend(),
                        &check_sql,
                    ))
                    .await
                {
                    Ok(result) => {
                        if result.rows_affected() == 0 {
                            // 数据库不存在，创建它
                            info!("Database '{}' does not exist, creating it...", db_name);
                            let create_sql = format!("CREATE DATABASE {}", db_name);

                            match db
                                .execute(sea_orm::Statement::from_string(
                                    db.get_database_backend(),
                                    &create_sql,
                                ))
                                .await
                            {
                                Ok(_) => {
                                    info!("Database '{}' created successfully", db_name);
                                }
                                Err(e) => {
                                    // 如果数据库已经存在，忽略这个错误
                                    if e.to_string().contains("already exists") {
                                        info!("Database '{}' already exists", db_name);
                                    } else {
                                        return Err(Box::new(e));
                                    }
                                }
                            }
                        } else {
                            info!("Database '{}' already exists", db_name);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to check database existence: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to connect to default database: {}", e);
            }
        }
    }

    Ok(())
}

/// 初始化数据库连接并执行迁移
pub async fn init_database(config: &DatabaseConfig) -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing database connection...");
    info!("Database URL: {}", &config.url);

    // 首先尝试创建数据库（如果不存在）
    info!("Checking if database exists...");

    if let Some(db_name) = extract_database_name(&config.url) {
        info!("Extracted database name: {}", db_name);
    }

    create_database_if_not_exists(&config.url).await?;

    // 使用配置的连接池参数
    let mut opt = ConnectOptions::new(&config.url);

    // 设置连接池参数
    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout))
        .idle_timeout(Duration::from_secs(config.idle_timeout))
        .max_lifetime(Duration::from_secs(config.max_lifetime))
        .sqlx_logging(true);

    // 尝试连接到目标数据库（带重试机制）
    let mut retry_count = 0;
    let max_retries = 5; // 增加重试次数
    let db = loop {
        match Database::connect(opt.clone()).await {
            Ok(db) => break db,
            Err(e) => {
                if retry_count >= max_retries {
                    warn!("Failed to connect after {} attempts: {}", max_retries, e);
                    return Err(Box::new(e));
                }

                warn!(
                    "Failed to connect to target database (attempt {}): {}",
                    retry_count + 1,
                    e
                );
                info!("Retrying connection after database creation...");

                // 等待一段时间让数据库创建完成（每次等待时间递增）
                let wait_time = 5 + retry_count * 5; // 增加等待时间
                info!("Waiting {} seconds for database to be ready...", wait_time);
                tokio::time::sleep(Duration::from_secs(wait_time)).await;

                retry_count += 1;
            }
        }
    };

    DB_CONNECTION
        .set(Arc::new(db))
        .map_err(|_| DbErr::Custom("Database connection already initialized".to_string()))?;

    info!(
        "Database connection initialized successfully with pool settings: max_connections={}, min_connections={}",
        config.max_connections, config.min_connections
    );

    // 执行数据库迁移
    info!("Executing database migrations...");

    let db_conn = get_db_connection();

    match DatabaseMigration::get_migration_status(&db_conn).await {
        Ok(status) => {
            info!(
                "Migration status: total={}, applied={}, pending={}",
                status.total_migrations, status.applied_migrations, status.pending_migrations
            );
        }
        Err(e) => {
            info!("Failed to get migration status: {}", e);
        }
    }

    // 应用迁移
    match DatabaseMigration::migrate_up(&db_conn).await {
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

    info!("Database initialization completed");
    Ok(())
}

pub fn get_db_connection() -> Arc<DatabaseConnection> {
    DB_CONNECTION
        .get()
        .expect("Database connection not initialized")
        .clone()
}

pub async fn test_connection(config: &DatabaseConfig) -> Result<(), DbErr> {
    let db = Database::connect(&config.url).await?;

    db.ping().await?;

    drop(db);
    Ok(())
}
