use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use tracing::info;

/// 用户表迁移
pub struct UsersTableMigration;

impl UsersTableMigration {
    /// 获取创建用户表的 SQL 语句
    pub fn create_table_sql(backend: DatabaseBackend) -> &'static str {
        match backend {
            DatabaseBackend::Postgres => {
                r#"
                CREATE TABLE IF NOT EXISTS users (
                    id VARCHAR(36) PRIMARY KEY,
                    name VARCHAR(50) NOT NULL UNIQUE,
                    phone VARCHAR(20) NOT NULL UNIQUE,
                    email VARCHAR(100) UNIQUE,
                    password_hash VARCHAR(255) NOT NULL,
                    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                    updated_at TIMESTAMP WITH TIME ZONE,
                    created_by VARCHAR(36),
                    updated_by VARCHAR(36),
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
        }
    }

    /// 获取删除用户表的 SQL 语句
    pub fn drop_table_sql() -> &'static str {
        "DROP TABLE IF EXISTS users"
    }

    /// 创建用户表
    pub async fn create_table<C: ConnectionTrait>(
        conn: &C,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let backend = conn.get_database_backend();
        let sql = Self::create_table_sql(backend);

        info!("创建用户表...");
        conn.execute(Statement::from_string(backend, sql)).await?;
        info!("用户表创建完成");

        Ok(())
    }

    /// 删除用户表
    pub async fn drop_table<C: ConnectionTrait>(
        conn: &C,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let backend = conn.get_database_backend();
        let sql = Self::drop_table_sql();

        info!("删除用户表...");
        conn.execute(Statement::from_string(backend, sql)).await?;
        info!("用户表删除完成");

        Ok(())
    }

    /// 重置用户表（删除后重新创建）
    pub async fn reset_table<C: ConnectionTrait>(
        conn: &C,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Self::drop_table(conn).await?;
        Self::create_table(conn).await?;
        Ok(())
    }
}
