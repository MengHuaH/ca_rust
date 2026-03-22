use axum::Json;
use tracing::info;

use crate::application::{ResponseService, SystemInfoService, create_response_service};

pub async fn system_info_handler()
-> Json<crate::domain::ApiResponse<crate::application::system_info::service::SystemInfoResponse>> {
    info!("System Info endpoint called");

    let system_info_service = SystemInfoService::new();
    let response_service = create_response_service();
    let system_info_data = system_info_service.get_system_info();

    response_service.success(system_info_data)
}
