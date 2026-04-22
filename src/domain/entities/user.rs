use bcrypt::{DEFAULT_COST, hash, verify};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::base_entity::BaseEntity;

/// 用户实体 - 使用组合模式包含基础实体
/// 通过组合 BaseEntity 来避免字段重复定义
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Validate)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[serde(flatten)]
    pub base: BaseEntity,

    // 用户特有字段
    #[validate(length(min = 2, max = 50, message = "用户名长度必须在2-50个字符之间"))]
    pub name: String,

    #[validate(regex(path = "PHONE_REGEX", message = "手机号格式不正确"))]
    pub phone: String,

    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,

    #[validate(length(min = 6, message = "密码长度至少6位"))]
    pub password_hash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

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
        // 使用 BaseEntity 创建基础字段
        let base_entity = BaseEntity::new(created_by);

        // 验证密码长度
        if password.len() < 6 {
            return Err("密码长度至少6位".into());
        }

        // 哈希密码
        let password_hash = hash(password, DEFAULT_COST)?;

        // 创建临时模型进行验证
        let temp_model = Self::from_base_and_fields(
            &base_entity,
            name.clone(),
            phone.clone(),
            email.clone(),
            password_hash.clone(),
        );

        // 使用宏生成的校验
        temp_model
            .validate()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // 创建 ActiveModel
        let mut active_model = ActiveModel::new();
        active_model.name = Set(name);
        active_model.phone = Set(phone);
        active_model.email = Set(email);
        active_model.password_hash = Set(password_hash);

        Ok(active_model)
    }

    /// 从基础实体和字段创建模型
    fn from_base_and_fields(
        base: &BaseEntity,
        name: String,
        phone: String,
        email: Option<String>,
        password_hash: String,
    ) -> Self {
        Self {
            base: base.clone(),
            name,
            phone,
            email,
            password_hash,
        }
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
        let temp_model = Self {
            base: base.clone(),
            name: name.clone(),
            phone: phone.clone(),
            email: email.clone(),
            password_hash: self.password_hash.clone(),
        };

        temp_model.validate()?;

        // 更新基础字段和用户特定字段
        let mut active_model: ActiveModel = self.clone().into();
        active_model.name = Set(name);
        active_model.phone = Set(phone);
        active_model.email = Set(email);

        // 更新基础实体
        active_model.base = self.base.update(updated_by);

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
        active_model.password_hash = Set(password_hash);
        active_model.base = self.base.update(updated_by);

        Ok(active_model)
    }

    /// 软删除用户
    ///
    /// # Arguments
    ///
    /// * `deleted_by` - 删除用户的ID
    pub fn soft_delete(&self, deleted_by: String) -> ActiveModel {
        self.base.soft_delete(deleted_by).into()
    }

    /// 获取基础实体（组合模式的核心）
    pub fn base_entity(&self) -> &BaseEntity {
        &self.base
    }

    /// 使用基础实体更新用户
    pub fn update_with_base(&mut self, base: BaseEntity) {
        self.base = base;
    }

    /// 创建新的用户模型（使用基础实体）
    pub fn new_with_base(
        name: String,
        phone: String,
        email: Option<String>,
        password_hash: String,
        created_by: String,
    ) -> Self {
        Self {
            base: BaseEntity::new(created_by),
            name,
            phone,
            email,
            password_hash,
        }
    }
}

// 手机号正则验证
lazy_static::lazy_static! {
    static ref PHONE_REGEX: regex::Regex = regex::Regex::new(r"^1[3-9]\d{9}$").unwrap();
}
