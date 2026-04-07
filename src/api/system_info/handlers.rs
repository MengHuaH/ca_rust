use axum::Json;
use tracing::info;
use utoipa::IntoParams;

use crate::application::{ResponseService, SystemInfoService, create_response_service};

/// 系统信息端点
///
/// 返回当前服务器的硬件和系统信息
#[utoipa::path(
    get,
    path = "/api/system-info",
    tag = "system",
    responses(
        (status = 200, description = "成功获取系统信息", body = ApiResponse<SystemInfoResponse>)
    )
)]
pub async fn system_info_handler()
-> Json<crate::domain::ApiResponse<crate::application::system_info::service::SystemInfoResponse>> {
    info!("System Info endpoint called");

    let system_info_service = SystemInfoService::new();
    let response_service = create_response_service();
    let system_info_data = system_info_service.get_system_info();

    response_service.success(system_info_data)
}
