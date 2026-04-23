use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

lazy_static::lazy_static! {
    static ref PHONE_REGEX: regex::Regex = regex::Regex::new(r"^1[3-9]\d{9}$").unwrap();
}

/// 更新用户命令
#[derive(Debug, Clone, Validate, IntoParams, ToSchema, Deserialize)]
pub struct UpdateUserCommand {
    /// 用户名（可选更新）
    #[validate(length(min = 2, max = 50, message = "用户名长度必须在2-50个字符之间"))]
    pub name: Option<String>,

    /// 手机号（可选更新）
    #[validate(regex(path = "PHONE_REGEX", message = "手机号格式不正确"))]
    pub phone: Option<String>,

    /// 邮箱（可选更新）
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
}
