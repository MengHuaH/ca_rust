use crate::application::users::command::change_password::dto::ChangePasswordCommand;
use crate::infrastructure::common::FieldError;
use sea_orm::DatabaseConnection;
use validator::Validate;

pub struct ChangePasswordValidator {
    _db: DatabaseConnection,
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("验证失败：{0:?}")]
    MultipleErrors(Vec<FieldError>),
}

impl ChangePasswordValidator {
    pub fn new(_db: DatabaseConnection) -> Self {
        Self { _db }
    }

    /// 验证修改密码命令
    pub async fn validate(
        &self,
        command: &ChangePasswordCommand,
    ) -> Result<(), Vec<FieldError>> {
        let mut errors = Vec::new();

        // DTO 字段格式验证
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

        // 检查新旧密码是否相同（额外验证）
        if command.old_password == command.new_password {
            errors.push(FieldError {
                field: "new_password".to_string(),
                message: "新密码不能与旧密码相同".to_string(),
                code: "SAME_PASSWORD".to_string(),
            });
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}
