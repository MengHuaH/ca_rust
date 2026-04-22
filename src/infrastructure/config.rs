use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub features: FeaturesConfig,
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
pub struct FeaturesConfig {
    pub enable_websocket: bool,
    pub enable_image_processing: bool,
    pub max_image_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        let DATABASE_URL: String = env::var("DB_URL")
            .unwrap_or("postgres://postgres:password@localhost:5432/db_default".to_string());
        Self {
            server: ServerConfig {
                host: "localhost".to_string(),
                port: 3000,
                workers: 4,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
            features: FeaturesConfig {
                enable_websocket: true,
                enable_image_processing: true,
                max_image_size: 10 * 1024 * 1024, // 10MB
            },
            database: DatabaseConfig {
                url: DATABASE_URL,
                max_connections: 10,
                min_connections: 2,
                acquire_timeout: 30,
                idle_timeout: 300,
                max_lifetime: 1800,
            },
        }
    }
}
