use crate::application::users::command::update::dto;
use crate::application::users::command::update::dto::UpdateUserCommand;
use crate::domain::entities::user::{Column, Entity as UsersEntity};
use crate::infrastructure::common::FieldError;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use tracing::error;
use validator::{Validate, ValidationErrors};

pub struct UpdateUserValidator {
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

    #[error("用户不存在: {0}")]
    UserNotFound(String),
}

impl UpdateUserValidator {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 完整的更新用户验证 - 收集所有错误
    pub async fn validate(
        &self,
        command: &UpdateUserCommand,
        user_id: &str,
    ) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        // 1. 首先进行DTO字段格式验证
        if let Err(validation_errors) = command.validate() {
            return Err(ValidationError::MultipleErrors(
                validation_errors
                    .field_errors()
                    .into_iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(|error| FieldError {
                            field: field.to_string(),
                            message: error
                                .message
                                .as_ref()
                                .map(|s| s.to_string())
                                .unwrap_or_default(),
                            code: format!("VALIDATION_ERROR_{}", field.to_uppercase()),
                        })
                    })
                    .collect(),
            ));
        }

        // 2. 验证用户是否存在
        if let Err(e) = self.validate_user_exists(&user_id).await {
            return Err(ValidationError::UserNotFound(e.to_string()));
        }

        // 3. 验证手机号唯一性（如果提供）
        if let Some(phone) = &command.phone {
            if let Err(e) = self.validate_phone_unique(&user_id, phone).await {
                errors.push(FieldError {
                    field: "phone".to_string(),
                    message: e.to_string(),
                    code: "INVALID_PHONE".to_string(),
                });
            }
        }

        // 4. 验证邮箱唯一性（如果提供）
        if let Some(email) = &command.email {
            if let Err(e) = self.validate_email_unique(&user_id, email).await {
                errors.push(FieldError {
                    field: "email".to_string(),
                    message: e.to_string(),
                    code: "INVALID_EMAIL".to_string(),
                });
            }
        }

        // 5. 验证用户名唯一性（如果提供）
        if let Some(name) = &command.name {
            if let Err(e) = self.validate_name_unique(&user_id, name).await {
                errors.push(FieldError {
                    field: "name".to_string(),
                    message: e.to_string(),
                    code: "INVALID_NAME".to_string(),
                });
            }
        }

        // 如果有错误，返回所有错误
        if !errors.is_empty() {
            return Err(ValidationError::MultipleErrors(errors));
        }

        Ok(())
    }

    /// 验证用户是否存在
    async fn validate_user_exists(&self, user_id: &str) -> Result<(), DbErr> {
        let existing_user: Option<crate::domain::entities::user::Model> = UsersEntity::find()
            .filter(Column::Id.eq(user_id))
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await?;

        if existing_user.is_none() {
            error!("用户不存在: {}", user_id);
            return Err(DbErr::Custom(format!("用户 {} 不存在", user_id)));
        }

        Ok(())
    }

    /// 验证用户名唯一性（排除当前用户）
    async fn validate_name_unique(&self, user_id: &str, name: &str) -> Result<(), DbErr> {
        let existing_user: Option<crate::domain::entities::user::Model> = UsersEntity::find()
            .filter(Column::Name.eq(name))
            .filter(Column::Id.ne(user_id))
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await?;

        if existing_user.is_some() {
            error!("用户名已存在: {}", name);
            return Err(DbErr::Custom(format!("用户名 {} 已被注册", name)));
        }

        Ok(())
    }

    /// 验证手机号唯一性（排除当前用户）
    async fn validate_phone_unique(&self, user_id: &str, phone: &str) -> Result<(), DbErr> {
        let existing_user: Option<crate::domain::entities::user::Model> = UsersEntity::find()
            .filter(Column::Phone.eq(phone))
            .filter(Column::Id.ne(user_id))
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await?;

        if existing_user.is_some() {
            error!("手机号已存在: {}", phone);
            return Err(DbErr::Custom(format!("手机号 {} 已被注册", phone)));
        }

        Ok(())
    }

    /// 验证邮箱唯一性（排除当前用户）
    async fn validate_email_unique(&self, user_id: &str, email: &str) -> Result<(), DbErr> {
        let existing_user: Option<crate::domain::entities::user::Model> = UsersEntity::find()
            .filter(Column::Email.eq(email))
            .filter(Column::Id.ne(user_id))
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
