use crate::application::users::command::create::validator::ValidationError;
use crate::application::users::command::create::{CreateUserCommand, CreateUserService};
use crate::infrastructure::common::{AuthUtils, ResponseBuilder};
use axum::{Json, extract::State, http::HeaderMap};
use tracing::{error, info};

/// 创建用户
///
/// 创建新用户，需要提供用户名、手机号、密码等信息
#[utoipa::path(
    post,
    path = "/api/users/create",
    tag = "users",
    request_body = CreateUserCommand,
    responses(
        (status = 200, description = "成功创建用户", body = ApiResponse<String>),
        (status = 400, description = "请求参数错误", body = ApiResponse<()>),
        (status = 409, description = "手机号或邮箱已存在", body = ApiResponse<()>),
        (status = 500, description = "服务器内部错误", body = ApiResponse<()>)
    )
)]
pub async fn create_user(
    State(db): State<sea_orm::DatabaseConnection>,
    headers: HeaderMap,
    Json(command): Json<CreateUserCommand>,
) -> (
    axum::http::StatusCode,
    Json<crate::infrastructure::common::ApiResponse<()>>,
) {
    info!("创建用户请求: {:?}", command);

    // 获取调用者信息（从请求头或其他认证信息）
    let created_by =
        AuthUtils::get_caller_from_headers(&headers).unwrap_or_else(|| "system".to_string());

    // 创建服务实例
    let user_service = CreateUserService::new(db);

    // 执行创建操作
    match user_service.execute(command, created_by).await {
        Ok(user_id) => {
            info!("用户创建成功: ID={}", user_id);
            ResponseBuilder::success_message(&format!("用户创建成功，ID: {}", user_id))
        }
        Err(e) => {
            error!("用户创建失败: {}", e);

            // 处理多重验证错误
            if let ValidationError::MultipleErrors(errors) = &e {
                error!("验证失败: {:?}", errors);
                return ResponseBuilder::validation_error(errors.to_vec());
            } else if let ValidationError::DatabaseValidationError(message) = &e {
                error!("数据库验证错误: {}", message);
                return ResponseBuilder::conflict(message);
            } else {
                error!("数据库错误: {}", e.to_string());
                return ResponseBuilder::server_error("服务器内部错误", Some(e.to_string()));
            }
        }
    }
}
