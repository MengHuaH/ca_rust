use super::base_entity::BaseEntity;
use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct User {
    #[serde(flatten)]
    pub base: BaseEntity,

    #[validate(length(min = 2, max = 50, message = "用户名长度必须在2-50个字符之间"))]
    pub name: String,

    #[validate(regex(path = "PHONE_REGEX", message = "手机号格式不正确"))]
    pub phone: String,

    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,

    #[validate(length(min = 6, message = "密码长度至少6位"))]
    pub password_hash: String,
}

impl User {
    /// 创建一个新的用户
    ///
    /// # Arguments
    ///
    /// * `name` - 用户名
    /// * `phone` - 用户手机号
    /// * `email` - 用户邮箱
    /// * `password` - 用户密码（明文）
    /// * `created_by` - 创建用户的ID
    pub fn new(
        created_by: String,
        name: String,
        phone: String,
        email: Option<String>,
        password: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // 验证输入数据
        let user = User {
            base: BaseEntity::new(created_by.clone()),
            name: name.clone(),
            phone: phone.clone(),
            email: email.clone(),
            password_hash: String::new(), // 临时值，将在下面设置
        };

        user.validate()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // 验证密码长度
        if password.len() < 6 {
            return Err("密码长度至少6位".into());
        }

        // 哈希密码
        let password_hash = hash(password, DEFAULT_COST)?;

        Ok(Self {
            base: BaseEntity::new(created_by),
            name,
            phone,
            email,
            password_hash,
        })
    }

    /// 验证密码
    ///
    /// # Arguments
    ///
    /// * `password` - 要验证的密码（明文）
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }

    /// 更新用户信息
    ///
    /// # Arguments
    ///
    /// * `name` - 新用户名
    /// * `phone` - 新手机号
    /// * `email` - 新邮箱
    /// * `updated_by` - 更新用户的ID
    pub fn update_info(
        &mut self,
        name: String,
        phone: String,
        email: Option<String>,
        updated_by: String,
    ) -> Result<(), validator::ValidationErrors> {
        let temp_user = User {
            base: self.base.clone(),
            name: name.clone(),
            phone: phone.clone(),
            email: email.clone(),
            password_hash: self.password_hash.clone(),
        };

        temp_user.validate()?;

        self.name = name;
        self.phone = phone;
        self.email = email;
        self.base.update(updated_by);

        Ok(())
    }

    /// 更新密码
    ///
    /// # Arguments
    ///
    /// * `new_password` - 新密码（明文）
    /// * `updated_by` - 更新用户的ID
    pub fn update_password(
        &mut self,
        new_password: String,
        updated_by: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 验证密码长度
        if new_password.len() < 6 {
            return Err("密码长度至少6位".into());
        }

        let password_hash = hash(new_password, DEFAULT_COST)?;
        self.password_hash = password_hash;
        self.base.update(updated_by);

        Ok(())
    }
}

// 手机号正则验证
lazy_static::lazy_static! {
    static ref PHONE_REGEX: regex::Regex = regex::Regex::new(r"^1[3-9]\d{9}$").unwrap();
}
