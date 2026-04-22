use serde::{Deserialize, Serialize};
use std::env;

// 加载 .env 文件
pub fn load_env() {
    dotenvy::dotenv().ok();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub retry_max_attempts: u32,
    pub retry_base_delay: u64,
    pub retry_delay_multiplier: u64,
}

impl DatabaseConfig {
    /// 构建完整的数据库连接URL
    pub fn build_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        // 确保环境变量已加载
        load_env();

        Self {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or("localhost".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or("3000".to_string())
                    .parse()
                    .unwrap_or(3000),
                workers: env::var("SERVER_WORKERS")
                    .unwrap_or("4".to_string())
                    .parse()
                    .unwrap_or(4),
            },
            logging: LoggingConfig {
                level: env::var("LOG_LEVEL").unwrap_or("info".to_string()),
                format: env::var("LOG_FORMAT").unwrap_or("json".to_string()),
            },
            database: DatabaseConfig {
                host: env::var("DB_HOST").unwrap_or("localhost".to_string()),
                port: env::var("DB_PORT")
                    .unwrap_or("5432".to_string())
                    .parse()
                    .unwrap_or(5432),
                username: env::var("DB_USERNAME").unwrap_or("postgres".to_string()),
                password: env::var("DB_PASSWORD").unwrap_or("password".to_string()),
                database: env::var("DB_NAME").unwrap_or("db_default".to_string()),
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or("10".to_string())
                    .parse()
                    .unwrap_or(10),
                min_connections: env::var("DB_MIN_CONNECTIONS")
                    .unwrap_or("2".to_string())
                    .parse()
                    .unwrap_or(2),
                acquire_timeout: env::var("DB_ACQUIRE_TIMEOUT")
                    .unwrap_or("30".to_string())
                    .parse()
                    .unwrap_or(30),
                idle_timeout: env::var("DB_IDLE_TIMEOUT")
                    .unwrap_or("300".to_string())
                    .parse()
                    .unwrap_or(300),
                max_lifetime: env::var("DB_MAX_LIFETIME")
                    .unwrap_or("1800".to_string())
                    .parse()
                    .unwrap_or(1800),
                retry_max_attempts: env::var("DB_RETRY_MAX_ATTEMPTS")
                    .unwrap_or("5".to_string())
                    .parse()
                    .unwrap_or(5),
                retry_base_delay: env::var("DB_RETRY_BASE_DELAY")
                    .unwrap_or("5".to_string())
                    .parse()
                    .unwrap_or(5),
                retry_delay_multiplier: env::var("DB_RETRY_DELAY_MULTIPLIER")
                    .unwrap_or("5".to_string())
                    .parse()
                    .unwrap_or(5),
            },
        }
    }
}
