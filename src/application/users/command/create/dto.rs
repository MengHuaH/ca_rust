use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

lazy_static::lazy_static! {
    static ref PHONE_REGEX: regex::Regex = regex::Regex::new(r"^1[3-9]\d{9}$").unwrap();
}

#[derive(Debug, Clone, Validate, IntoParams, ToSchema, Deserialize)]
pub struct CreateUserCommand {
    #[validate(length(min = 2, max = 50, message = "用户名长度必须在2-50个字符之间"))]
    pub name: String,

    #[validate(regex(path = "*PHONE_REGEX", message = "手机号格式不正确"))]
    pub phone: String,

    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,

    #[validate(length(min = 6, message = "密码长度至少6位"))]
    pub password_hash: String,
}
