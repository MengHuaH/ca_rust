use sea_orm::DbConn;
use tracing::info;

use crate::infrastructure::database::migrations::UsersTableMigration;

/// 数据库迁移管理器
pub struct DatabaseMigration;

impl DatabaseMigration {
    /// 创建所有数据库表
    pub async fn create_tables(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
        info!("开始创建数据库表...");

        // 创建用户表
        UsersTableMigration::create_table(db).await?;

        // 未来可以添加其他表的创建
        // OtherTableMigration::create_table(db).await?;

        info!("数据库表创建完成");
        Ok(())
    }

    /// 删除所有数据库表
    pub async fn drop_tables(db: &DbConn) -> Result<(), Box<dyn std::error::Error>> {
        info!("开始删除数据库表...");

        // 删除用户表
        UsersTableMigration::drop_table(db).await?;

        // 未来可以添加其他表的删除
        // OtherTableMigration::drop_table(db).await?;

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
        // 这里可以添加检查表是否存在的逻辑
        // 暂时返回一个默认状态
        Ok(MigrationStatus {
            users_table_exists: true, // 需要实现实际的检查逻辑
        })
    }
}

/// 迁移状态
#[derive(Debug, Clone)]
pub struct MigrationStatus {
    pub users_table_exists: bool,
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
