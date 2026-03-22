use axum::Json;
use tracing::info;

use crate::application::{HelloService, ResponseService, create_response_service};

pub async fn hello_world() -> Json<crate::domain::ApiResponse<crate::application::hello::service::HelloResponse>> {
    info!("Hello World endpoint called");
    
    let hello_service = HelloService::new();
    let response_service = create_response_service();
    let hello_data = hello_service.get_hello_message();
    
    response_service.success(hello_data)
}