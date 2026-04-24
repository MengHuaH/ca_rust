use crate::application::users::command::update::validator::ValidationError;
use crate::application::users::command::update::{UpdateUserCommand, UpdateUserService};
use crate::infrastructure::common::{AuthUtils, ResponseBuilder};
use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};
use tracing::{error, info};
use utoipa::ToSchema;

/// 更新用户
///
/// 更新用户信息，支持部分更新
#[utoipa::path(
    put,
    path = "/api/users/update/{user_id}",
    tag = "users",
    request_body = UpdateUserCommand,
    params(
        ("user_id" = String, Path, description = "用户ID")
    ),
    responses(
        (status = 200, description = "成功更新用户", body = ApiResponseString),
        (status = 400, description = "请求参数错误", body = ApiResponseString),
        (status = 404, description = "用户不存在", body = ApiResponseString),
        (status = 409, description = "数据冲突", body = ApiResponseString),
        (status = 500, description = "服务器内部错误", body = ApiResponseString)
    )
)]
pub async fn update_user(
    State(db): State<sea_orm::DatabaseConnection>,
    Path(user_id): Path<String>,
    headers: HeaderMap,
    Json(command): Json<UpdateUserCommand>,
) -> (
    axum::http::StatusCode,
    Json<crate::infrastructure::common::ApiResponse<()>>,
) {
    info!("更新用户请求: ID={}, 数据={:?}", user_id, command);

    // 获取调用者信息
    let updated_by =
        AuthUtils::get_caller_from_headers(&headers).unwrap_or_else(|| "system".to_string());

    // 创建服务实例
    let user_service = UpdateUserService::new(db);

    // 执行更新操作
    match user_service.execute(command, user_id, updated_by).await {
        Ok(user_id) => {
            info!("用户更新成功: ID={}", user_id);
            ResponseBuilder::success_message(&format!("用户更新成功，ID: {}", user_id))
        }
        Err(e) => {
            error!("用户更新失败: {}", e);

            // 根据错误类型返回不同的友好错误响应
            match e {
                ValidationError::MultipleErrors(errors) => {
                    error!("验证失败: {:?}", errors);
                    ResponseBuilder::validation_error(errors.to_vec())
                }
                ValidationError::UserNotFound(message) => {
                    error!("用户不存在: {}", message);
                    ResponseBuilder::not_found(&message)
                }
                ValidationError::DatabaseValidationError(message) => {
                    error!("数据库验证错误: {}", message);
                    let message = format!("数据库验证错误: {}", message);
                    ResponseBuilder::conflict(&message)
                }
                ValidationError::DatabaseError(e) => {
                    error!("数据库错误: {}", e);
                    ResponseBuilder::server_error("服务器内部错误", Some(e.to_string()))
                }
            }
        }
    }
}
