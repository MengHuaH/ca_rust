use crate::application::users::command::create::dto::CreateUserCommand;
use crate::application::users::command::create::validatorCommand::{
    CreateUserValidator, ValidationError,
};
use crate::domain::entities::user::Model;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use tracing::info;
use uuid::Uuid;

pub struct CreateUserService {
    db: DatabaseConnection,
}

impl CreateUserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建用户应用服务
    pub async fn execute(
        &self,
        command: CreateUserCommand,
        created_by: String,
    ) -> Result<String, ValidationError> {
        // 1. 使用验证器进行完整验证
        let validator = CreateUserValidator::new(self.db.clone());
        validator.validate(&command).await?;

        // 2. 使用领域层方法创建用户实体
        let user_id = Uuid::new_v4().to_string();
        let user_active_model = Model::new(
            created_by,
            command.name,
            command.phone,
            command.email,
            command.password_hash,
        )
        .map_err(|e| ValidationError::MultipleErrors(vec![e.to_string()]))?;

        // 3. 保存到数据库
        let user_model = user_active_model
            .insert(&self.db)
            .await
            .map_err(ValidationError::DatabaseError)?;

        info!(
            "用户创建成功: ID={}, 手机号={}",
            user_model.id, user_model.phone
        );

        Ok(user_model.id)
    }
}
