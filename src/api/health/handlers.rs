use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::application::{ResponseService, create_response_service};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthData {
    pub status: String,
    pub version: String,
}

pub async fn health_check() -> Json<crate::domain::ApiResponse<HealthData>> {
    info!("Health check endpoint called");
    
    let response_service = create_response_service();
    let health_data = HealthData {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    };
    
    response_service.success(health_data)
}