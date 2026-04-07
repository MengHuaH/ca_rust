use axum::Json;
use tracing::info;
use utoipa::IntoParams;

use crate::application::{HelloService, ResponseService, create_response_service};

/// 欢迎消息端点
/// 
/// 返回一个简单的欢迎消息，用于测试API基本功能
#[utoipa::path(
    get,
    path = "/api/hello",
    tag = "hello",
    responses(
        (status = 200, description = "成功返回欢迎消息", body = ApiResponse<HelloResponse>)
    )
)]
pub async fn hello_world() -> Json<crate::domain::ApiResponse<crate::application::hello::service::HelloResponse>> {
    info!("Hello World endpoint called");
    
    let hello_service = HelloService::new();
    let response_service = create_response_service();
    let hello_data = hello_service.get_hello_message();
    
    response_service.success(hello_data)
}