use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::application::{ResponseService, create_response_service};

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoData {
    pub echo: String,
}

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
