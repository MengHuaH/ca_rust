use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::IntoParams;

use crate::application::{ResponseService, create_response_service};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EchoRequest {
    /// 需要回显的消息
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EchoData {
    /// 回显的消息内容
    pub echo: String,
}

/// 消息回显端点
///
/// 接收一个消息并返回相同的消息内容，用于测试API连通性
#[utoipa::path(
    post,
    path = "/api/echo",
    tag = "echo",
    request_body = EchoRequest,
    responses(
        (status = 200, description = "成功回显消息"),
        (status = 400, description = "请求参数错误")
    )
)]
pub async fn echo_message(
    Json(payload): Json<EchoRequest>,
) -> Json<crate::domain::ApiResponse<EchoData>> {
    info!("Echo endpoint called with message: {}", payload.message);

    let response_service = create_response_service();
    let echo_data = EchoData {
        echo: payload.message,
    };
    response_service.success(echo_data)
}
