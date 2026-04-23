use axum::{Json, extract::State, http::HeaderMap};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::ToSchema;

use crate::application::users::command::create::{CreateUserCommand, CreateUserService};
use crate::domain::entities::user::Model;
use crate::infrastructure::common::{AuthUtils, PasswordSecurity};

/// 创建用户
///
/// 创建新用户，需要提供用户名、手机号、密码等信息
#[utoipa::path(
    post,
    path = "/api/users/create",
    tag = "users",
    request_body = CreateUserCommand,
    responses(
        (status = 201, description = "成功创建用户", body = String),
        (status = 400, description = "请求参数错误"),
        (status = 409, description = "手机号或邮箱已存在"),
        (status = 500, description = "服务器内部错误")
    )
)]
pub async fn create_user(
    State(db): State<sea_orm::DatabaseConnection>,
    headers: HeaderMap,
    Json(command): Json<CreateUserCommand>,
) -> Result<Json<String>, axum::http::StatusCode> {
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
            Ok(Json(user_id))
        }
        Err(e) => {
            error!("用户创建失败: {}", e);

            // 根据错误类型返回不同的状态码
            if e.to_string().contains("已被注册") {
                return Err(axum::http::StatusCode::CONFLICT);
            }

            if e.to_string().contains("验证失败") || e.to_string().contains("格式不正确") {
                return Err(axum::http::StatusCode::BAD_REQUEST);
            }

            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
