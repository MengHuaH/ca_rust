use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct HelloResponse {
    /// 欢迎消息
    pub message: String,
}

pub struct HelloService;

impl HelloService {
    pub fn new() -> Self {
        HelloService
    }

    pub fn get_hello_message(&self) -> HelloResponse {
        info!("HelloService: Generating hello message");
        HelloResponse {
            message: "Hello, World!".to_string(),
        }
    }
}
