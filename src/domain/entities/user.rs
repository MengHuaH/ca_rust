use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::base_entity::{self, BaseEntity};

// 使用宏创建 SeaORM 实体，自动包含基础字段，支持字段校验
sea_orm_entity_with_base!(
    User,
    "users",
    name: String => {length(min = 2, max = 50, message = "用户名长度必须在2-50个字符之间")},
    phone: String => {regex(path = "PHONE_REGEX", message = "手机号格式不正确")},
    email: Option<String> => {email(message = "邮箱格式不正确")},
    password_hash: String => {length(min = 6, message = "密码长度至少6位")}
);

impl Model {
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
    ) -> Result<ActiveModel, Box<dyn std::error::Error>> {
        // 创建基础实体
        let base_entity = BaseEntity::new(created_by);

        // 验证密码长度
        if password.len() < 6 {
            return Err("密码长度至少6位".into());
        }

        // 哈希密码
        let password_hash = hash(password, DEFAULT_COST)?;

        // 创建临时模型进行验证
        let temp_model = Model::from_base_and_fields(
            base_entity,
            name.clone(),
            phone.clone(),
            email.clone(),
            password_hash.clone(),
        );

        // 使用宏生成的校验
        temp_model
            .validate()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // 使用宏提供的方法创建 ActiveModel
        let model = Model::from_base_and_fields(
            BaseEntity::new(created_by),
            name,
            phone,
            email,
            password_hash,
        );
        Ok(model.into())
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
        &self,
        name: String,
        phone: String,
        email: Option<String>,
        updated_by: String,
    ) -> Result<ActiveModel, validator::ValidationErrors> {
        // 创建临时模型进行验证
        let temp_model = Model {
            id: self.id.clone(),
            created_at: self.created_at,
            created_by: self.created_by.clone(),
            updated_at: self.updated_at,
            updated_by: self.updated_by.clone(),
            is_deleted: self.is_deleted,
            deleted_at: self.deleted_at,
            deleted_by: self.deleted_by.clone(),
            name: name.clone(),
            phone: phone.clone(),
            email: email.clone(),
            password_hash: self.password_hash.clone(),
        };

        temp_model.validate()?;

        // 更新基础字段和用户特定字段
        let mut active_model: ActiveModel = self.clone().into();
        active_model.name = sea_orm::Set(name);
        active_model.phone = sea_orm::Set(phone);
        active_model.email = sea_orm::Set(email);
        active_model.updated_at = sea_orm::Set(Some(chrono::Utc::now()));
        active_model.updated_by = sea_orm::Set(Some(updated_by));

        Ok(active_model)
    }

    /// 更新密码
    ///
    /// # Arguments
    ///
    /// * `new_password` - 新密码（明文）
    /// * `updated_by` - 更新用户的ID
    pub fn update_password(
        &self,
        new_password: String,
        updated_by: String,
    ) -> Result<ActiveModel, Box<dyn std::error::Error>> {
        // 验证密码长度
        if new_password.len() < 6 {
            return Err("密码长度至少6位".into());
        }

        let password_hash = hash(new_password, DEFAULT_COST)?;

        let mut active_model: ActiveModel = self.clone().into();
        active_model.password_hash = sea_orm::Set(password_hash);
        active_model.updated_at = sea_orm::Set(Some(chrono::Utc::now()));
        active_model.updated_by = sea_orm::Set(Some(updated_by));

        Ok(active_model)
    }

    /// 软删除用户
    ///
    /// # Arguments
    ///
    /// * `deleted_by` - 删除用户的ID
    pub fn soft_delete(&self, deleted_by: String) -> ActiveModel {
        self.soft_delete_base(deleted_by)
    }
}

// 手机号正则验证
lazy_static::lazy_static! {
    static ref PHONE_REGEX: regex::Regex = regex::Regex::new(r"^1[3-9]\d{9}$").unwrap();
}
