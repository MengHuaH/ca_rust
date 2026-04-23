use crate::application::users::command::delete::dto::DeleteUserCommand;
use crate::application::users::command::delete::validator::{DeleteUserValidator, ValidationError};
use crate::domain::entities::user::{Column, Entity as UsersEntity, Model};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::info;

pub struct DeleteUserService {
    db: DatabaseConnection,
}

impl DeleteUserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 删除用户应用服务
    pub async fn execute(
        &self,
        command: DeleteUserCommand,
        deleted_by: String,
    ) -> Result<String, ValidationError> {
        // 1. 使用验证器进行完整验证
        let validator = DeleteUserValidator::new(self.db.clone());
        validator.validate(&command).await?;

        // 2. 获取现有用户数据
        let existing_user = UsersEntity::find()
            .filter(Column::Id.eq(&command.user_id))
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
            .map_err(ValidationError::DatabaseError)?
            .ok_or_else(|| ValidationError::UserNotFound(command.user_id.clone()))?;

        // 3. 构建删除模型（软删除）
        let mut user_active_model = existing_user.soft_delete(deleted_by);

        // 4. 保存到数据库
        let deleted_user: Model = user_active_model
            .update(&self.db)
            .await
            .map_err(ValidationError::DatabaseError)?;

        info!("用户删除成功: ID={}", deleted_user.id);

        Ok(deleted_user.id)
    }
}
