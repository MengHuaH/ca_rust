use sea_orm::Database;
use std::env;
use tracing::info;
use tracing_subscriber;

mod infrastructure;
use infrastructure::database::DatabaseMigration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::init();

    info!("开始测试迁移系统...");

    // 设置数据库连接 URL（使用 SQLite 内存数据库进行测试）
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

    info!("使用数据库: {}", database_url);

    // 创建数据库连接
    let db = Database::connect(&database_url).await?;

    info!("数据库连接成功");

    // 测试迁移状态
    info!("检查迁移状态...");
    match DatabaseMigration::get_migration_status(&db).await {
        Ok(status) => {
            info!(
                "迁移状态: 总迁移数={}, 已应用={}, 待处理={}",
                status.total_migrations, status.applied_migrations, status.pending_migrations
            );
            if let Some(version) = status.current_version {
                info!("当前版本: {}", version);
            }
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
    info!("迁移后检查状态...");
    match DatabaseMigration::get_migration_status(&db).await {
        Ok(status) => {
            info!(
                "迁移后状态: 总迁移数={}, 已应用={}, 待处理={}",
                status.total_migrations, status.applied_migrations, status.pending_migrations
            );
        }
        Err(e) => {
            info!("获取迁移后状态失败: {}", e);
        }
    }

    // 测试回滚
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

    // 最终状态检查
    info!("最终迁移状态...");
    match DatabaseMigration::get_migration_status(&db).await {
        Ok(status) => {
            info!(
                "最终状态: 总迁移数={}, 已应用={}, 待处理={}",
                status.total_migrations, status.applied_migrations, status.pending_migrations
            );
        }
        Err(e) => {
            info!("获取最终状态失败: {}", e);
        }
    }

    info!("迁移系统测试完成");
    Ok(())
}
