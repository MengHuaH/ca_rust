use std::path::Path;
use tracing::{error, info};
use tracing_subscriber;

use CA::infrastructure::database::connection::{
    get_db_connection, init_database_migration_only, init_database_no_migration,
};
use CA::infrastructure::database::migration::DatabaseMigration;

use clap::{Arg, Command};
use dotenvy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("Warning: Could not load .env file: {}", e);
    }

    // 初始化日志
    tracing_subscriber::fmt::init();

    // 解析命令行参数
    let matches = Command::new("数据库迁移工具")
        .version("1.0")
        .about("数据库迁移管理工具")
        .arg(
            Arg::new("action")
                .short('a')
                .long("action")
                .value_name("ACTION")
                .help("执行的操作: migrate, rollback, create")
                .default_value("migrate"),
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .value_name("VERSION")
                .help("回滚到指定版本"),
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("迁移模板名称"),
        )
        .get_matches();

    let action = matches.get_one::<String>("action").unwrap();
    let target_version = matches.get_one::<String>("target");
    let migration_name = matches.get_one::<String>("name");

    info!("Starting database migration tool with action: {}", action);

    // 加载数据库配置
    let app_config: CA::infrastructure::config::AppConfig =
        CA::infrastructure::config::AppConfig::default();
    info!("App config loaded successfully");
    let database_config = app_config.database;
    info!(
        "Database config extracted: host={}, port={}, database={}",
        database_config.host, database_config.port, database_config.database
    );

    match action.as_str() {
        "migrate" => {
            // 执行迁移
            info!("执行数据库迁移...");
            match init_database_migration_only(&database_config).await {
                Ok(()) => {
                    info!("Database migration completed successfully!");
                    println!("Database migration completed successfully!");
                }
                Err(e) => {
                    error!("Database migration failed: {}", e);
                    eprintln!("Database migration failed: {}", e);
                    return Err(e);
                }
            }
        }
        "rollback" => {
            // 执行回滚
            info!("执行数据库回滚...");
            match init_database_no_migration(&database_config).await {
                Ok(()) => {
                    let db_conn = get_db_connection();
                    let result = if let Some(target) = target_version {
                        info!("回滚到版本: {}", target);
                        DatabaseMigration::rollback_to_version(&db_conn, target).await
                    } else {
                        info!("回滚最后一个迁移");
                        DatabaseMigration::rollback_last(&db_conn).await
                    };

                    match result {
                        Ok(rolled_back) => {
                            if rolled_back.is_empty() {
                                info!("没有可回滚的迁移");
                                println!("没有可回滚的迁移");
                            } else {
                                info!("回滚完成，共回滚了 {} 个迁移", rolled_back.len());
                                println!(
                                    "回滚完成，共回滚了 {} 个迁移: {:?}",
                                    rolled_back.len(),
                                    rolled_back
                                );
                            }
                        }
                        Err(e) => {
                            error!("Database rollback failed: {}", e);
                            eprintln!("Database rollback failed: {}", e);
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    error!("Database connection failed: {}", e);
                    eprintln!("Database connection failed: {}", e);
                    return Err(e);
                }
            }
        }
        "create" => {
            // 创建迁移模板
            if let Some(name) = migration_name {
                create_migration_template(name)?;
            } else {
                error!("创建迁移模板需要指定名称，使用 --name 参数");
                eprintln!("创建迁移模板需要指定名称，使用 --name 参数");
                return Err("Missing migration name".into());
            }
        }
        _ => {
            error!("未知的操作: {}", action);
            eprintln!("未知的操作: {}", action);
            return Err(format!("Unknown action: {}", action).into());
        }
    }

    Ok(())
}

/// 创建迁移模板文件
fn create_migration_template(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
    let version = format!("_{}", timestamp);
    let file_name = format!(
        "src/infrastructure/database/migrations/{}_create_{}.rs",
        version, name
    );

    // 创建migrations目录（如果不存在）
    let migrations_dir = Path::new("src/infrastructure/database/migrations");
    if !migrations_dir.exists() {
        std::fs::create_dir_all(migrations_dir)?;
        info!("创建 migrations 目录");
    }

    // 将表名转换为大驼峰命名法
    let camel_case_name = name
        .chars()
        .next()
        .unwrap()
        .to_uppercase()
        .collect::<String>()
        + &name[1..];

    // 创建迁移模板内容（Rust格式）
    let template = format!(
        "use sea_orm::{{ConnectionTrait, DatabaseBackend, Statement}};\n\
        use tracing::info;\n\n\
        /// {}表迁移\n\
        pub struct {}TableMigration;\n\n\
        impl {}TableMigration {{\n\
            /// 获取创建{}表的 SQL 语句\n\
            pub fn create_table_sql(backend: DatabaseBackend) -> &'static str {{\n\
                match backend {{\n\
                    DatabaseBackend::Postgres => {{\n\
                        r#\"\n\
                        CREATE TABLE IF NOT EXISTS {} (\n\
                            id SERIAL PRIMARY KEY,\n\
                            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,\n\
                            updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP\n\
                        )\"#\n\
                    }}\n\
                    DatabaseBackend::MySql => {{\n\
                        r#\"\n\
                        CREATE TABLE IF NOT EXISTS {} (\n\
                            id INT AUTO_INCREMENT PRIMARY KEY,\n\
                            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,\n\
                            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP\n\
                        )\"#\n\
                    }}\n\
                    DatabaseBackend::Sqlite => {{\n\
                        r#\"\n\
                        CREATE TABLE IF NOT EXISTS {} (\n\
                            id INTEGER PRIMARY KEY AUTOINCREMENT,\n\
                            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,\n\
                            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP\n\
                        )\"#\n\
                    }}\n\
                }}\n\
            }}\n\n\
            /// 获取删除{}表的 SQL 语句\n\
            pub fn drop_table_sql() -> &'static str {{\n\
                \"DROP TABLE IF EXISTS {}\"\n\
            }}\n\n\
            /// 创建{}表\n\
            pub async fn create_table<C: ConnectionTrait>(\n\
                conn: &C,\n\
            ) -> Result<(), Box<dyn std::error::Error>> {{\n\
                let backend = conn.get_database_backend();\n\
                let sql = Self::create_table_sql(backend);\n\n\
                info!(\"创建{}表...\");\n\
                conn.execute(Statement::from_string(backend, sql)).await?;\n\
                info!(\"{}表创建完成\");\n\n\
                Ok(())\n\
            }}\n\n\
            /// 删除{}表\n\
            pub async fn drop_table<C: ConnectionTrait>(\n\
                conn: &C,\n\
            ) -> Result<(), Box<dyn std::error::Error>> {{\n\
                let backend = conn.get_database_backend();\n\
                let sql = Self::drop_table_sql();\n\n\
                info!(\"删除{}表...\");\n\
                conn.execute(Statement::from_string(backend, sql)).await?;\n\
                info!(\"{}表删除完成\");\n\n\
                Ok(())\n\
            }}\n\
        }}\n",
        name, camel_case_name, camel_case_name, name, name, name, name, name, name, name, name, name, name, name, name
    );

    // 写入文件
    std::fs::write(&file_name, template)?;

    // 更新mod.rs文件
    update_migrations_mod(&version, name)?;

    info!("创建迁移模板: {}", file_name);
    println!("创建迁移模板: {}", file_name);

    Ok(())
}

/// 更新migrations模块的mod.rs文件
fn update_migrations_mod(version: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mod_file_path = "src/infrastructure/database/migrations/mod.rs";
    let mod_file_content = std::fs::read_to_string(mod_file_path)?;

    // 将表名转换为大驼峰命名法
    let camel_case_name = name
        .chars()
        .next()
        .unwrap()
        .to_uppercase()
        .collect::<String>()
        + &name[1..];

    // 添加新的模块声明
    let new_module_decl = format!("pub mod {}_create_{};", version, name);
    let new_use_decl = format!(
        "pub use {}_create_{}::{}TableMigration;",
        version, name, camel_case_name
    );

    let mut lines: Vec<String> = mod_file_content.lines().map(|s| s.to_string()).collect();

    // 在现有模块声明后添加新的模块声明
    let mut insert_index = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("pub mod") {
            insert_index = i + 1;
        }
    }

    lines.insert(insert_index, new_module_decl);

    // 在现有use声明后添加新的use声明
    let mut use_insert_index = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("pub use") {
            use_insert_index = i + 1;
        }
    }

    lines.insert(use_insert_index, new_use_decl);

    // 写入更新后的文件
    let new_content = lines.join("\n");
    std::fs::write(mod_file_path, new_content)?;

    info!("更新 migrations mod.rs 文件");

    Ok(())
}
