use sea_orm::DbConn;
use tracing::info;

use crate::infrastructure::database::migration_manager::{MigrationManager, MigrationStatus};
use crate::infrastructure::database::migrations::{MigrationVersionsTable, UsersTableMigration};

/// 数据库迁移管理器
pub struct DatabaseMigration;

impl DatabaseMigration {
    /// 创建所有数据库表（使用迁移版本管理）
    pub async fn create_tables(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
        info!("开始创建数据库表（使用迁移版本管理）...");

        // 使用迁移版本管理系统
        MigrationManager::migrate_up(db).await?;

        info!("数据库表创建完成");
        Ok(())
    }

    /// 删除所有数据库表
    pub async fn drop_tables(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
        info!("开始删除数据库表...");

        // 删除用户表
        UsersTableMigration::drop_table(db).await?;

        // 删除迁移版本表（最后删除）
        MigrationVersionsTable::drop_table(db).await?;

        info!("数据库表删除完成");
        Ok(())
    }

    /// 重置数据库（删除后重新创建所有表）
    pub async fn reset_database(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
        info!("开始重置数据库...");

        Self::drop_tables(db).await?;
        Self::create_tables(db).await?;

        info!("数据库重置完成");
        Ok(())
    }

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
        Self::drop_tables(db).await?;

        // 重新应用所有迁移
        MigrationManager::migrate_up(db).await?;

        info!("迁移强制重置完成");
        Ok(())
    }
}

/// 向后兼容的旧接口（保持现有代码不变）
pub async fn create_tables(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
    DatabaseMigration::create_tables(db).await
}

/// 向后兼容的旧接口（保持现有代码不变）
pub async fn drop_tables(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
    DatabaseMigration::drop_tables(db).await
}

/// 向后兼容的旧接口（保持现有代码不变）
pub async fn reset_database(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
    DatabaseMigration::reset_database(db).await
}
