use crate::application::users::command::change_password::dto::ChangePasswordCommand;
use crate::application::users::command::change_password::validator::{
    ChangePasswordValidator, ValidationError,
};
use crate::domain::entities::user::Entity as UsersEntity;
use crate::infrastructure::common::PasswordSecurity;
use sea_orm::ActiveModelTrait;
use sea_orm::{EntityTrait, ModelTrait};
use sea_orm::{IntoActiveModel, Set};
use tracing::info;

pub struct ChangePasswordService {
    db: sea_orm::DatabaseConnection,
}

impl ChangePasswordService {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }

    /// 修改密码
    pub async fn execute(
        &self,
        user_id: &str,
        command: ChangePasswordCommand,
        updated_by: String,
    ) -> Result<(), ValidationError> {
        // 1. 验证输入
        let validator = ChangePasswordValidator::new(self.db.clone());
        validator
            .validate(&command)
            .await
            .map_err(|e| ValidationError::MultipleErrors(e))?;

        // 2. 查找用户
        let user = UsersEntity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                ValidationError::MultipleErrors(vec![crate::infrastructure::common::FieldError {
                    field: "user_id".to_string(),
                    message: format!("查询用户失败：{}", e),
                    code: "DATABASE_ERROR".to_string(),
                }])
            })?
            .ok_or_else(|| {
                ValidationError::MultipleErrors(vec![crate::infrastructure::common::FieldError {
                    field: "user_id".to_string(),
                    message: "用户不存在".to_string(),
                    code: "USER_NOT_FOUND".to_string(),
                }])
            })?;

        // 3. 检查用户是否被删除
        if user.is_deleted {
            return Err(ValidationError::MultipleErrors(vec![
                crate::infrastructure::common::FieldError {
                    field: "user_id".to_string(),
                    message: "用户已被删除".to_string(),
                    code: "USER_DELETED".to_string(),
                },
            ]));
        }

        // 4. 在领域层验证旧密码并生成新的密码哈希
        let active_model = user
            .change_password(command.old_password, command.new_password, updated_by)
            .map_err(|e| {
                ValidationError::MultipleErrors(vec![crate::infrastructure::common::FieldError {
                    field: "old_password".to_string(),
                    message: e.to_string(),
                    code: "INVALID_OLD_PASSWORD".to_string(),
                }])
            })?;

        // 5. 保存到数据库
        active_model.update(&self.db).await.map_err(|e| {
            ValidationError::MultipleErrors(vec![crate::infrastructure::common::FieldError {
                field: "password".to_string(),
                message: format!("更新密码失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }])
        })?;

        info!("用户密码修改成功：user_id={}", user_id);

        Ok(())
    }
}
