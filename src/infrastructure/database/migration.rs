use sea_orm::DbConn;
use tracing::info;

use crate::infrastructure::database::migration_manager::{MigrationManager, MigrationStatus};
use crate::infrastructure::database::migrations::{MigrationVersionsTable, UsersTableMigration};

/// 数据库迁移管理器
pub struct DatabaseMigration;

impl DatabaseMigration {
    /// 获取数据库迁移状态
    pub async fn get_migration_status(
        db: &DbConn,
    ) -> Result<MigrationStatus, Box<dyn std::error::Error>> {
        MigrationManager::get_migration_status(db).await
    }

    /// 应用待处理的迁移
    pub async fn migrate_up(db: &DbConn) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        MigrationManager::migrate_up(db).await
    }

    /// 回滚迁移
    pub async fn migrate_down(
        db: &DbConn,
        target_version: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        MigrationManager::migrate_down(db, target_version).await
    }

    /// 回滚到指定版本
    pub async fn rollback_to_version(
        db: &DbConn,
        version: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        MigrationManager::migrate_down(db, Some(version)).await
    }

    /// 回滚最后一个迁移
    pub async fn rollback_last(db: &DbConn) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let status = MigrationManager::get_migration_status(db).await?;

        if let Some(current_version) = status.current_version {
            MigrationManager::migrate_down(db, Some(&current_version)).await
        } else {
            Ok(vec![])
        }
    }

    /// 强制重置迁移（删除所有迁移记录，重新应用）
    pub async fn force_reset_migrations(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
        info!("强制重置迁移...");

        // 删除所有表
        UsersTableMigration::drop_table(db).await?;

        // 重新应用所有迁移
        MigrationManager::migrate_up(db).await?;

        info!("迁移强制重置完成");
        Ok(())
    }
}
