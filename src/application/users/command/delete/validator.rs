use crate::application::users::command::delete::dto::DeleteUserCommand;
use crate::domain::entities::user::{Column, Entity as UsersEntity};
use crate::infrastructure::common::FieldError;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use tracing::error;
use validator::Validate;

pub struct DeleteUserValidator {
    db: DatabaseConnection,
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("验证失败: {0:?}")]
    MultipleErrors(Vec<FieldError>),

    #[error("数据库错误: {0:?}")]
    DatabaseError(#[from] DbErr),

    #[error("用户不存在: {0}")]
    UserNotFound(String),
}

impl DeleteUserValidator {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 完整的删除用户验证 - 收集所有错误
    pub async fn validate(&self, command: &DeleteUserCommand) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        // 1. 首先进行DTO字段格式验证
        if let Err(validation_errors) = command.validate() {
            errors.extend(
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
                    .collect::<Vec<_>>(),
            );
        }

        // 2. 验证用户是否存在且未被删除
        if let Err(e) = self.validate_user_exists_and_active(&command.user_id).await {
            return Err(ValidationError::UserNotFound(e.to_string()));
        }

        // 如果有错误，返回所有错误
        if !errors.is_empty() {
            return Err(ValidationError::MultipleErrors(errors));
        }

        Ok(())
    }

    /// 验证用户是否存在且未被删除
    async fn validate_user_exists_and_active(&self, user_id: &str) -> Result<(), DbErr> {
        let existing_user: Option<crate::domain::entities::user::Model> = UsersEntity::find()
            .filter(Column::Id.eq(user_id))
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await?;

        if existing_user.is_none() {
            error!("用户不存在或已被删除: {}", user_id);
            return Err(DbErr::Custom(format!("用户 {} 不存在或已被删除", user_id)));
        }

        Ok(())
    }
}
