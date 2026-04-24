use crate::application::users::command::delete::validator::ValidationError;
use crate::application::users::command::delete::{DeleteUserCommand, DeleteUserService};
use crate::infrastructure::common::{AuthUtils, ResponseBuilder};
use axum::{Json, extract::State, http::HeaderMap};
use tracing::{error, info};

/// 删除用户
///
/// 软删除用户，将用户标记为已删除状态
#[utoipa::path(
    delete,
    path = "/api/users/delete/{user_id}",
    tag = "users",
    params(
        ("user_id" = String, Path, description = "用户ID")
    ),
    responses(
        (status = 200, description = "成功删除用户", body = ApiResponseString),
        (status = 400, description = "请求参数错误", body = ApiResponseString),
        (status = 404, description = "用户不存在", body = ApiResponseString),
        (status = 500, description = "服务器内部错误", body = ApiResponseString)
    )
)]
pub async fn delete_user(
    State(db): State<sea_orm::DatabaseConnection>,
    headers: HeaderMap,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> (
    axum::http::StatusCode,
    Json<crate::infrastructure::common::ApiResponse<()>>,
) {
    info!("删除用户请求: user_id={}", user_id);

    // 获取调用者信息（从请求头或其他认证信息）
    let deleted_by =
        AuthUtils::get_caller_from_headers(&headers).unwrap_or_else(|| "system".to_string());

    // 创建删除命令
    let command = DeleteUserCommand { user_id };

    // 创建服务实例
    let user_service = DeleteUserService::new(db);

    // 执行删除操作
    match user_service.execute(command, deleted_by).await {
        Ok(user_id) => {
            info!("用户删除成功: ID={}", user_id);
            ResponseBuilder::success_message(&format!("用户删除成功，ID: {}", user_id))
        }
        Err(e) => {
            error!("用户删除失败: {}", e);

            // 处理多重验证错误
            if let ValidationError::MultipleErrors(errors) = &e {
                error!("验证失败: {:?}", errors);
                ResponseBuilder::validation_error(errors.clone())
            } else {
                match e {
                    ValidationError::UserNotFound(msg) => ResponseBuilder::not_found(&msg),
                    ValidationError::DatabaseError(db_err) => {
                        error!("数据库错误: {}", db_err);
                        ResponseBuilder::server_error("删除用户时发生数据库错误", None)
                    }
                    _ => ResponseBuilder::server_error(&e.to_string(), None),
                }
            }
        }
    }
}
