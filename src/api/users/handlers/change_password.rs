use crate::application::users::command::change_password::{
    ChangePasswordCommand, ChangePasswordService,
};
use crate::infrastructure::common::{ApiResponse, AuthUtils, ResponseBuilder};
use axum::{extract::State, http::HeaderMap, Json};
use serde::Deserialize;
use tracing::{error, info};
use utoipa::ToSchema;

/// 修改密码请求体
#[derive(Debug, Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    /// 旧密码
    pub old_password: String,
    /// 新密码
    pub new_password: String,
}

/// 修改密码
///
/// 用户修改自己的密码，需要提供旧密码进行验证
#[utoipa::path(
    put,
    path = "/api/users/password/{user_id}",
    tag = "users",
    params(
        ("user_id" = String, Path, description = "用户 ID")
    ),
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "密码修改成功", body = ApiResponse),
        (status = 400, description = "请求参数错误", body = ApiResponse),
        (status = 401, description = "旧密码错误", body = ApiResponse),
        (status = 404, description = "用户不存在", body = ApiResponse),
        (status = 500, description = "服务器内部错误", body = ApiResponse)
    )
)]
pub async fn change_password(
    State(db): State<sea_orm::DatabaseConnection>,
    headers: HeaderMap,
    axum::extract::Path(user_id): axum::extract::Path<String>,
    Json(command): Json<ChangePasswordRequest>,
) -> (axum::http::StatusCode, Json<ApiResponse<()>>) {
    info!("修改密码请求：user_id={}", user_id);

    // 获取调用者信息
    let updated_by =
        AuthUtils::get_caller_from_headers(&headers).unwrap_or_else(|| "system".to_string());

    // 转换为应用层命令
    let change_password_command = ChangePasswordCommand {
        old_password: command.old_password,
        new_password: command.new_password,
    };

    // 创建服务实例
    let password_service = ChangePasswordService::new(db);

    // 执行修改密码操作
    match password_service
        .execute(&user_id, change_password_command, updated_by)
        .await
    {
        Ok(_) => {
            info!("用户密码修改成功：user_id={}", user_id);
            ResponseBuilder::success_message("密码修改成功")
        }
        Err(e) => {
            error!("用户密码修改失败：user_id={}, error={}", user_id, e);

            // 处理验证错误
            if let crate::application::users::command::change_password::validator::ValidationError::MultipleErrors(
                errors,
            ) = &e
            {
                // 检查是否是旧密码错误
                for error in errors {
                    if error.code == "INVALID_OLD_PASSWORD" {
                        return ResponseBuilder::unauthorized("旧密码不正确");
                    }
                    if error.code == "USER_NOT_FOUND" || error.code == "USER_DELETED" {
                        return ResponseBuilder::not_found("用户");
                    }
                }

                return ResponseBuilder::validation_error(errors.clone());
            }

            ResponseBuilder::server_error("密码修改失败", Some(e.to_string()))
        }
    }
}
