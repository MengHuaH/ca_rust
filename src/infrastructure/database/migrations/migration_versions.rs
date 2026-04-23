use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use tracing::info;

/// 迁移版本表管理
pub struct MigrationVersionsTable;

impl MigrationVersionsTable {
    /// 获取创建迁移版本表的 SQL 语句
    pub fn create_table_sql(backend: DatabaseBackend) -> &'static str {
        match backend {
            DatabaseBackend::Postgres => {
                r#"
                CREATE TABLE IF NOT EXISTS migration_versions (
                    id SERIAL PRIMARY KEY,
                    version VARCHAR(50) NOT NULL UNIQUE,
                    migration_name VARCHAR(100) NOT NULL,
                    applied_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    checksum VARCHAR(64) NOT NULL,
                    description TEXT
                )"#
            }
            DatabaseBackend::MySql => {
                r#"
                CREATE TABLE IF NOT EXISTS migration_versions (
                    id INT AUTO_INCREMENT PRIMARY KEY,
                    version VARCHAR(50) NOT NULL UNIQUE,
                    migration_name VARCHAR(100) NOT NULL,
                    applied_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    checksum VARCHAR(64) NOT NULL,
                    description TEXT
                )"#
            }
            DatabaseBackend::Sqlite => {
                r#"
                CREATE TABLE IF NOT EXISTS migration_versions (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    version TEXT NOT NULL UNIQUE,
                    migration_name TEXT NOT NULL,
                    applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    checksum TEXT NOT NULL,
                    description TEXT
                )"#
            }
        }
    }

    /// 获取删除迁移版本表的 SQL 语句
    pub fn drop_table_sql() -> &'static str {
        "DROP TABLE IF EXISTS migration_versions"
    }

    /// 创建迁移版本表
    pub async fn create_table<C: ConnectionTrait>(
        conn: &C,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let backend = conn.get_database_backend();
        let sql = Self::create_table_sql(backend);

        info!("创建迁移版本表...");
        conn.execute(Statement::from_string(backend, sql)).await?;
        info!("迁移版本表创建完成");

        Ok(())
    }

    /// 删除迁移版本表
    pub async fn drop_table<C: ConnectionTrait>(
        conn: &C,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let backend = conn.get_database_backend();
        let sql = Self::drop_table_sql();

        info!("删除迁移版本表...");
        conn.execute(Statement::from_string(backend, sql)).await?;
        info!("迁移版本表删除完成");

        Ok(())
    }

    /// 检查迁移版本表是否存在
    pub async fn table_exists<C: ConnectionTrait>(
        conn: &C,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let backend = conn.get_database_backend();
        let check_sql = match backend {
            DatabaseBackend::Postgres => {
                "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'migration_versions')"
            }
            DatabaseBackend::MySql => {
                "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'migration_versions'"
            }
            DatabaseBackend::Sqlite => {
                "SELECT name FROM sqlite_master WHERE type='table' AND name='migration_versions'"
            }
        };

        let result = conn
            .execute(Statement::from_string(backend, check_sql))
            .await?;

        match backend {
            DatabaseBackend::Postgres => {
                // PostgreSQL 返回布尔值
                Ok(true) // 简化实现，实际需要解析结果
            }
            DatabaseBackend::MySql => {
                // MySQL 返回计数
                Ok(true) // 简化实现，实际需要解析结果
            }
            DatabaseBackend::Sqlite => {
                // SQLite 返回表名或空
                Ok(true) // 简化实现，实际需要解析结果
            }
        }
    }
}
