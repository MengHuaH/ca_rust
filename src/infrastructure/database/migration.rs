use sea_orm::{ConnectionTrait, DatabaseBackend, DbConn, Statement};
use tracing::info;

pub async fn create_tables(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
    info!("开始创建数据库表...");

    let backend = db.get_database_backend();

    // 创建用户表
    let create_users_table = match backend {
        DatabaseBackend::Postgres => {
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id VARCHAR(36) PRIMARY KEY,
                name VARCHAR(50) NOT NULL UNIQUE,
                phone VARCHAR(20) NOT NULL UNIQUE,
                email VARCHAR(100) UNIQUE,
                password_hash VARCHAR(255) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
                created_by VARCHAR(36) NOT NULL,
                updated_by VARCHAR(36) NOT NULL,
                is_deleted BOOLEAN NOT NULL DEFAULT false,
                deleted_at TIMESTAMP WITH TIME ZONE,
                deleted_by VARCHAR(36)
            )"#
        }
        DatabaseBackend::MySql => {
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id VARCHAR(36) PRIMARY KEY,
                name VARCHAR(50) NOT NULL UNIQUE,
                phone VARCHAR(20) NOT NULL UNIQUE,
                email VARCHAR(100) UNIQUE,
                password_hash VARCHAR(255) NOT NULL,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL,
                created_by VARCHAR(36) NOT NULL,
                updated_by VARCHAR(36) NOT NULL,
                is_deleted BOOLEAN NOT NULL DEFAULT false,
                deleted_at DATETIME,
                deleted_by VARCHAR(36)
            )"#
        }
        DatabaseBackend::Sqlite => {
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                phone TEXT NOT NULL UNIQUE,
                email TEXT UNIQUE,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                created_by TEXT NOT NULL,
                updated_by TEXT NOT NULL,
                is_deleted INTEGER NOT NULL DEFAULT 0,
                deleted_at TEXT,
                deleted_by TEXT
            )"#
        }
    };

    db.execute(Statement::from_string(backend, create_users_table))
        .await?;

    info!("数据库表创建完成");
    Ok(())
}

pub async fn drop_tables(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
    info!("开始删除数据库表...");

    let backend = db.get_database_backend();

    // 删除用户表
    let drop_users_table = "DROP TABLE IF EXISTS users";
    db.execute(Statement::from_string(backend, drop_users_table))
        .await?;

    info!("数据库表删除完成");
    Ok(())
}

pub async fn reset_database(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
    info!("开始重置数据库...");

    drop_tables(db).await?;
    create_tables(db).await?;

    info!("数据库重置完成");
    Ok(())
}
