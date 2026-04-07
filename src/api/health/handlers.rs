use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::IntoParams;

use crate::application::{ResponseService, create_response_service};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct HealthResponse {
    pub success: bool,
    pub code: u32,
    pub message: String,
    pub data: HealthData,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct HealthData {
    /// 服务状态
    pub status: String,
    /// API版本
    pub version: String,
}

/// 健康检查端点
///
/// 返回服务器的健康状态和版本信息
#[utoipa::path(
    get,
    path = "/api/health",
    tag = "health",
    responses(
        (status = 200, description = "服务正常运行", body = ApiResponse<HealthData>)
    )
)]
pub async fn health_check() -> Json<crate::domain::ApiResponse<HealthData>> {
    info!("Health check endpoint called");

    let response_service = create_response_service();
    let health_data = HealthData {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    };

    response_service.success(health_data)
}
