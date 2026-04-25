use chrono::{DateTime, Utc};
use sea_orm::{ConnectionTrait, DbConn, Statement};
use sha2::{Digest, Sha256};
use tracing::{error, info, warn};

use crate::infrastructure::database::migrations::{MigrationVersionsTable, UsersTableMigration};

/// 迁移定义
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: String,
    pub name: String,
    pub description: String,
    pub up_sql: String,
    pub down_sql: String,
}

impl Migration {
    /// 计算迁移的校验和
    pub fn checksum(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.up_sql.as_bytes());
        hasher.update(self.down_sql.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// 迁移版本记录
#[derive(Debug, Clone)]
pub struct MigrationRecord {
    pub id: i32,
    pub version: String,
    pub migration_name: String,
    pub applied_at: DateTime<Utc>,
    pub checksum: String,
    pub description: String,
}

/// 迁移管理器
pub struct MigrationManager;

impl MigrationManager {
    /// 获取所有迁移定义
    pub fn get_migrations() -> Vec<Migration> {
        vec![
            Migration {
                version: "001".to_string(),
                name: "create_migration_versions_table".to_string(),
                description: "Create migration versions table".to_string(),
                up_sql: MigrationVersionsTable::create_table_sql(
                    sea_orm::DatabaseBackend::Postgres,
                )
                .to_string(),
                down_sql: MigrationVersionsTable::drop_table_sql().to_string(),
            },
            Migration {
                version: "002".to_string(),
                name: "create_users_table".to_string(),
                description: "Create users table".to_string(),
                up_sql: UsersTableMigration::create_table_sql(sea_orm::DatabaseBackend::Postgres)
                    .to_string(),
                down_sql: UsersTableMigration::drop_table_sql().to_string(),
            },
        ]
    }

    /// 获取已应用的迁移
    pub async fn get_applied_migrations<C: ConnectionTrait>(
        conn: &C,
    ) -> Result<Vec<MigrationRecord>, Box<dyn std::error::Error>> {
        // 检查迁移版本表是否存在
        if !MigrationVersionsTable::table_exists(conn).await? {
            return Ok(vec![]);
        }

        let backend = conn.get_database_backend();
        let select_sql = "SELECT id, version, migration_name, applied_at, checksum, description FROM migration_versions ORDER BY version";

        // 使用 query_all 而不是 execute 来获取查询结果
        let rows = conn
            .query_all(Statement::from_string(backend, select_sql))
            .await?;

        let mut applied_migrations = Vec::new();
        for row in rows {
            applied_migrations.push(MigrationRecord {
                id: row
                    .try_get("", "id")
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?,
                version: row
                    .try_get("", "version")
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?,
                migration_name: row
                    .try_get("", "migration_name")
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?,
                applied_at: row
                    .try_get("", "applied_at")
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?,
                checksum: row
                    .try_get("", "checksum")
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?,
                description: row
                    .try_get("", "description")
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?,
            });
        }

        Ok(applied_migrations)
    }

    /// 获取待应用的迁移
    pub async fn get_pending_migrations<C: ConnectionTrait>(
        conn: &C,
    ) -> Result<Vec<Migration>, Box<dyn std::error::Error>> {
        let applied_migrations = Self::get_applied_migrations(conn).await?;
        let all_migrations = Self::get_migrations();

        let pending_migrations: Vec<Migration> = all_migrations
            .into_iter()
            .filter(|migration| {
                !applied_migrations
                    .iter()
                    .any(|applied| applied.version == migration.version)
            })
            .collect();

        Ok(pending_migrations)
    }

    /// 应用迁移
    pub async fn migrate_up<C: ConnectionTrait>(
        conn: &C,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        info!("Starting migration...");

        let pending_migrations = Self::get_pending_migrations(conn).await?;
        let mut applied_versions = Vec::new();

        // 确保迁移版本表存在
        if !MigrationVersionsTable::table_exists(conn).await? {
            MigrationVersionsTable::create_table(conn).await?;
            info!("Migration versions table created");
        }

        for migration in pending_migrations {
            info!(
                "Applying migration: {} - {}",
                migration.version, migration.name
            );

            // 执行迁移 SQL
            let backend = conn.get_database_backend();
            conn.execute(Statement::from_string(backend, &migration.up_sql))
                .await?;

            // 记录迁移版本
            Self::record_migration(conn, &migration).await?;

            let version = migration.version.clone();
            applied_versions.push(version.clone());
            info!("Migration {} applied successfully", version);
        }

        if applied_versions.is_empty() {
            info!("No pending migrations");
        } else {
            info!("All migrations applied, total: {}", applied_versions.len());
        }

        Ok(applied_versions)
    }

    /// 回滚迁移
    pub async fn migrate_down<C: ConnectionTrait>(
        conn: &C,
        target_version: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        info!("开始回滚迁移...");

        let applied_migrations = Self::get_applied_migrations(conn).await?;
        let mut rolled_back_versions = Vec::new();

        // 按版本号降序排列（最新的先回滚）
        let mut migrations_to_rollback: Vec<MigrationRecord> = applied_migrations
            .into_iter()
            .filter(|record| {
                target_version.map_or(true, |target| record.version >= target.to_string())
            })
            .collect();

        migrations_to_rollback.sort_by(|a, b| b.version.cmp(&a.version));

        for record in migrations_to_rollback {
            info!("回滚迁移: {} - {}", record.version, record.migration_name);

            // 查找对应的迁移定义
            let migration = Self::get_migrations()
                .into_iter()
                .find(|m| m.version == record.version)
                .ok_or_else(|| format!("找不到迁移定义: {}", record.version))?;

            // 执行回滚 SQL
            let backend = conn.get_database_backend();
            conn.execute(Statement::from_string(backend, &migration.down_sql))
                .await?;

            // 删除迁移记录
            Self::remove_migration_record(conn, &record.version).await?;

            let version = record.version.clone();
            rolled_back_versions.push(version.clone());
            info!("迁移 {} 回滚完成", version);

            // 如果指定了目标版本，回滚到该版本后停止
            if let Some(target) = target_version {
                if record.version == target {
                    break;
                }
            }
        }

        if rolled_back_versions.is_empty() {
            info!("没有可回滚的迁移");
        } else {
            info!(
                "迁移回滚完成，共回滚了 {} 个迁移",
                rolled_back_versions.len()
            );
        }

        Ok(rolled_back_versions)
    }

    /// 记录迁移版本
    async fn record_migration<C: ConnectionTrait>(
        conn: &C,
        migration: &Migration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let backend = conn.get_database_backend();
        let sql = format!(
            "INSERT INTO migration_versions (version, migration_name, checksum, description) VALUES ('{}', '{}', '{}', '{}')",
            migration.version,
            migration.name,
            migration.checksum(),
            migration.description
        );

        conn.execute(Statement::from_string(backend, &sql)).await?;
        Ok(())
    }

    /// 删除迁移记录
    async fn remove_migration_record<C: ConnectionTrait>(
        conn: &C,
        version: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let backend = conn.get_database_backend();
        let sql = format!(
            "DELETE FROM migration_versions WHERE version = '{}'",
            version
        );

        conn.execute(Statement::from_string(backend, &sql)).await?;
        Ok(())
    }

    /// 获取迁移状态
    pub async fn get_migration_status<C: ConnectionTrait>(
        conn: &C,
    ) -> Result<MigrationStatus, Box<dyn std::error::Error>> {
        let applied_migrations = Self::get_applied_migrations(conn).await?;
        let pending_migrations = Self::get_pending_migrations(conn).await?;
        let all_migrations = Self::get_migrations();

        Ok(MigrationStatus {
            total_migrations: all_migrations.len(),
            applied_migrations: applied_migrations.len(),
            pending_migrations: pending_migrations.len(),
            current_version: applied_migrations.last().map(|m| m.version.clone()),
            applied: applied_migrations,
            pending: pending_migrations,
        })
    }
}

/// 迁移状态
#[derive(Debug, Clone)]
pub struct MigrationStatus {
    pub total_migrations: usize,
    pub applied_migrations: usize,
    pub pending_migrations: usize,
    pub current_version: Option<String>,
    pub applied: Vec<MigrationRecord>,
    pub pending: Vec<Migration>,
}
