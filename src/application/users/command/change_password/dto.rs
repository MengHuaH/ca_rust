use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Clone, Validate, Deserialize, ToSchema)]
pub struct ChangePasswordCommand {
    /// 旧密码
    #[validate(length(min = 6, message = "密码长度至少 6 位"))]
    pub old_password: String,

    /// 新密码
    #[validate(length(min = 6, message = "新密码长度至少 6 位"))]
    pub new_password: String,
}
