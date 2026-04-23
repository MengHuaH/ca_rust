use crate::application::users::command::create::dto::CreateUserCommand;
use crate::domain::entities::user::{Column, Entity as UsersEntity};
use crate::infrastructure::common::FieldError;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use std::collections::HashSet;
use tracing::error;
use utoipa::openapi::Array;
use validator::Validate;

pub struct CreateUserValidator {
    db: DatabaseConnection,
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("验证失败: {0:?}")]
    MultipleErrors(Vec<FieldError>),

    #[error("数据库错误: {0:?}")]
    DatabaseError(#[from] DbErr),

    #[error("数据库验证错误: {0:?}")]
    DatabaseValidationError(String),
}

impl CreateUserValidator {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 完整的创建用户验证 - 收集所有错误
    pub async fn validate(&self, command: &CreateUserCommand) -> Result<(), Vec<FieldError>> {
        let mut errors = Vec::new();

        // 验证手机号唯一性
        if let Err(e) = self.validate_phone_unique(&command.phone).await {
            errors.push(FieldError {
                field: "phone".to_string(),
                message: e.to_string(),
                code: "INVALID_PHONE".to_string(),
            });
        }

        // 验证邮箱唯一性（如果提供）
        if let Some(email) = &command.email {
            if let Err(e) = self.validate_email_unique(email).await {
                errors.push(FieldError {
                    field: "email".to_string(),
                    message: e.to_string(),
                    code: "INVALID_EMAIL".to_string(),
                });
            }
        }

        // 验证用户名唯一性
        if let Err(e) = self.validate_name_unique(&command.name).await {
            errors.push(FieldError {
                field: "name".to_string(),
                message: e.to_string(),
                code: "INVALID_NAME".to_string(),
            });
        }

        // 如果有错误，返回所有错误
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    /// 验证用户名唯一性
    async fn validate_name_unique(&self, name: &str) -> Result<(), DbErr> {
        let existing_user = UsersEntity::find()
            .filter(Column::Name.eq(name))
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await?;

        if existing_user.is_some() {
            error!("用户名已存在: {}", name);
            return Err(DbErr::Custom(format!("用户名 {} 已被注册", name)));
        }

        Ok(())
    }

    /// 验证手机号唯一性
    async fn validate_phone_unique(&self, phone: &str) -> Result<(), DbErr> {
        let existing_user = UsersEntity::find()
            .filter(Column::Phone.eq(phone))
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await?;

        if existing_user.is_some() {
            error!("手机号已存在: {}", phone);
            return Err(DbErr::Custom(format!("手机号 {} 已被注册", phone)));
        }

        Ok(())
    }

    /// 验证邮箱唯一性
    async fn validate_email_unique(&self, email: &str) -> Result<(), DbErr> {
        let existing_user = UsersEntity::find()
            .filter(Column::Email.eq(email))
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await?;

        if existing_user.is_some() {
            error!("邮箱已存在: {}", email);
            return Err(DbErr::Custom(format!("邮箱 {} 已被注册", email)));
        }

        Ok(())
    }
}
