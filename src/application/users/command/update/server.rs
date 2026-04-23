use crate::application::users::command::update::dto::UpdateUserCommand;
use crate::application::users::command::update::validator::{UpdateUserValidator, ValidationError};
use crate::domain::entities::user::{Column, Entity as UsersEntity, Model};
use crate::infrastructure::common::FieldError;
use crate::infrastructure::common::PasswordSecurity;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
};
use tracing::info;

pub struct UpdateUserService {
    db: DatabaseConnection,
}

impl UpdateUserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 更新用户应用服务
    pub async fn execute(
        &self,
        command: UpdateUserCommand,
        updated_by: String,
    ) -> Result<String, ValidationError> {
        // 1. 使用验证器进行完整验证
        let validator = UpdateUserValidator::new(self.db.clone());
        validator.validate(&command).await?;

        // 2. 获取现有用户数据
        let existing_user = UsersEntity::find()
            .filter(Column::Id.eq(&command.user_id))
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
            .map_err(ValidationError::DatabaseError)?
            .ok_or_else(|| ValidationError::UserNotFound(command.user_id.clone()))?;

        // 3. 使用领域层方法创建用户实体（传入已哈希的密码）
        let user_active_model = existing_user
            .update_info(
                updated_by,
                command.name,
                command.phone,
                command.email.clone(),
            )
            .map_err(|e| ValidationError::DatabaseValidationError(e.to_string()))?;

        // 5. 保存到数据库
        let updated_user: Model = user_active_model
            .update(&self.db)
            .await
            .map_err(ValidationError::DatabaseError)?;

        info!("用户更新成功: ID={}", updated_user.id);

        Ok(updated_user.id)
    }
}
