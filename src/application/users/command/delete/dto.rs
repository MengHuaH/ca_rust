use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/// 删除用户命令
#[derive(Debug, Clone, Validate, IntoParams, ToSchema, Deserialize)]
pub struct DeleteUserCommand {
    /// 用户ID
    #[validate(length(min = 1, message = "用户ID不能为空"))]
    pub user_id: String,
}