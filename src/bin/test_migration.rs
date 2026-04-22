use std::env;
use tracing::{Level, info};
use tracing_subscriber;

use sea_orm::Database;

use crate::infrastructure::database::migration::DatabaseMigration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("开始测试迁移系统...");

    // 使用您的 PostgreSQL 数据库
    let database_url = "postgres://postgres:password@localhost:5432/";

    info!("使用数据库: {}", database_url);

    // 创建数据库连接
    let db = Database::connect(database_url).await?;

    info!("数据库连接成功");

    // 测试迁移状态
    info!("检查迁移状态...");
    match DatabaseMigration::get_migration_status(&db).await {
        Ok(status) => {
            info!(
                "迁移状态: 总迁移数={}, 已应用={}, 待处理={}",
                status.total_migrations, status.applied_migrations, status.pending_migrations
            );
        }
        Err(e) => {
            info!("获取迁移状态失败: {}", e);
        }
    }

    // 应用迁移
    info!("开始应用迁移...");
    match DatabaseMigration::migrate_up(&db).await {
        Ok(applied) => {
            if applied.is_empty() {
                info!("没有需要应用的迁移");
            } else {
                info!("成功应用了 {} 个迁移: {:?}", applied.len(), applied);
            }
        }
        Err(e) => {
            info!("应用迁移失败: {}", e);
            return Err(e);
        }
    }

    // 再次检查迁移状态
    info!("再次检查迁移状态...");
    match DatabaseMigration::get_migration_status(&db).await {
        Ok(status) => {
            info!(
                "迁移状态: 总迁移数={}, 已应用={}, 待处理={}",
                status.total_migrations, status.applied_migrations, status.pending_migrations
            );
        }
        Err(e) => {
            info!("获取迁移状态失败: {}", e);
        }
    }

    // 测试回滚最后一个迁移
    info!("测试回滚最后一个迁移...");
    match DatabaseMigration::rollback_last(&db).await {
        Ok(rolled_back) => {
            if rolled_back.is_empty() {
                info!("没有需要回滚的迁移");
            } else {
                info!("成功回滚了 {} 个迁移: {:?}", rolled_back.len(), rolled_back);
            }
        }
        Err(e) => {
            info!("回滚迁移失败: {}", e);
        }
    }

    // 最终迁移状态
    info!("最终迁移状态...");
    match DatabaseMigration::get_migration_status(&db).await {
        Ok(status) => {
            info!(
                "迁移状态: 总迁移数={}, 已应用={}, 待处理={}",
                status.total_migrations, status.applied_migrations, status.pending_migrations
            );
        }
        Err(e) => {
            info!("获取迁移状态失败: {}", e);
        }
    }

    info!("迁移系统测试完成");
    Ok(())
}
