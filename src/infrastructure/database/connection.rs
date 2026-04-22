use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::Arc;
use tokio::sync::OnceCell;
use tracing::info;

use crate::infrastructure::config::DatabaseConfig;

static DB_CONNECTION: OnceCell<Arc<DatabaseConnection>> = OnceCell::const_new();

pub async fn init_database(config: &DatabaseConfig) -> Result<(), DbErr> {
    info!("正在初始化数据库连接...");
    
    let db = Database::connect(&config.url).await?;
    
    DB_CONNECTION
        .set(Arc::new(db))
        .map_err(|_| DbErr::Custom("数据库连接已初始化".to_string()))?;
    
    info!("数据库连接初始化成功");
    Ok(())
}

pub fn get_db_connection() -> Arc<DatabaseConnection> {
    DB_CONNECTION
        .get()
        .expect("数据库连接未初始化")
        .clone()
}

pub async fn test_connection(config: &DatabaseConfig) -> Result<(), DbErr> {
    let db = Database::connect(&config.url).await?;
    
    db.ping().await?;
    
    drop(db);
    Ok(())
}