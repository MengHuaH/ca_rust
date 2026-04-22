use bcrypt::{DEFAULT_COST, hash, verify};
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 用户实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Validate)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,

    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_by: Option<String>,

    pub is_deleted: bool,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_by: Option<String>,

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
    pub fn new(
        created_by: String,
        name: String,
        phone: String,
        email: Option<String>,
        password: String,
    ) -> Result<ActiveModel, Box<dyn std::error::Error>> {
        use uuid::Uuid;

        // 验证密码长度
        if password.len() < 6 {
            return Err("密码长度至少6位".into());
        }

        // 哈希密码
        let password_hash = hash(password, DEFAULT_COST)?;

        // 创建临时模型进行验证
        let temp_model = Model {
            id: Uuid::new_v4().to_string(),
            created_at: chrono::Utc::now(),
            created_by: created_by.clone(),
            updated_at: None,
            updated_by: None,
            is_deleted: false,
            deleted_at: None,
            deleted_by: None,
            name: name.clone(),
            phone: phone.clone(),
            email: email.clone(),
            password_hash: password_hash.clone(),
        };

        temp_model
            .validate()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // 创建 ActiveModel
        let mut active_model = ActiveModel::new();
        active_model.id = Set(Uuid::new_v4().to_string());
        active_model.created_at = Set(chrono::Utc::now());
        active_model.created_by = Set(created_by);
        active_model.updated_at = Set(None);
        active_model.updated_by = Set(None);
        active_model.is_deleted = Set(false);
        active_model.deleted_at = Set(None);
        active_model.deleted_by = Set(None);
        active_model.name = Set(name);
        active_model.phone = Set(phone);
        active_model.email = Set(email);
        active_model.password_hash = Set(password_hash);

        Ok(active_model)
    }

    /// 验证密码
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }

    /// 更新用户信息
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

        // 更新字段
        let mut active_model: ActiveModel = self.clone().into();
        active_model.name = Set(name);
        active_model.phone = Set(phone);
        active_model.email = Set(email);
        active_model.updated_at = Set(Some(chrono::Utc::now()));
        active_model.updated_by = Set(Some(updated_by));

        Ok(active_model)
    }

    /// 更新密码
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
        active_model.updated_at = Set(Some(chrono::Utc::now()));
        active_model.updated_by = Set(Some(updated_by));

        Ok(active_model)
    }

    /// 软删除用户
    pub fn soft_delete(&self, deleted_by: String) -> ActiveModel {
        let mut active_model: ActiveModel = self.clone().into();
        active_model.is_deleted = Set(true);
        active_model.deleted_at = Set(Some(chrono::Utc::now()));
        active_model.deleted_by = Set(Some(deleted_by));
        active_model
    }
}

// 手机号正则验证
lazy_static::lazy_static! {
    static ref PHONE_REGEX: regex::Regex = regex::Regex::new(r"^1[3-9]\d{9}$").unwrap();
}
