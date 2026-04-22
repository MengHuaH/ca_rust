use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr, Statement};

use crate::domain::entities::User;

use super::base_repository::{BaseRepository, BaseRepositoryImpl, EntityMapper};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User, DbErr>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, DbErr>;
    async fn find_by_phone(&self, phone: &str) -> Result<Option<User>, DbErr>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DbErr>;
    async fn update(&self, user: User) -> Result<User, DbErr>;
    async fn delete(&self, id: &str, deleted_by: &str) -> Result<(), DbErr>;
    async fn list(&self, page: u64, page_size: u64) -> Result<Vec<User>, DbErr>;
}

pub struct UserMapper;

impl EntityMapper<User> for UserMapper {
    fn table_name() -> &'static str {
        "users"
    }

    fn id_column() -> &'static str {
        "id"
    }

    fn to_insert_params(user: &User) -> Vec<(String, String)> {
        vec![
            ("id".to_string(), format!("'{}'", user.base.id)),
            ("name".to_string(), format!("'{}'", user.name)),
            ("phone".to_string(), format!("'{}'", user.phone)),
            (
                "email".to_string(),
                user.email
                    .as_ref()
                    .map_or("NULL".to_string(), |e| format!("'{}'", e)),
            ),
            (
                "password_hash".to_string(),
                format!("'{}'", user.password_hash),
            ),
            (
                "created_at".to_string(),
                format!("'{}'", user.base.created_at),
            ),
            (
                "updated_at".to_string(),
                format!("'{}'", user.base.updated_at),
            ),
            (
                "created_by".to_string(),
                format!("'{}'", user.base.created_by),
            ),
            (
                "updated_by".to_string(),
                format!("'{}'", user.base.updated_by),
            ),
        ]
    }

    fn to_update_params(user: &User) -> Vec<(String, String)> {
        vec![
            ("name".to_string(), format!("'{}'", user.name)),
            ("phone".to_string(), format!("'{}'", user.phone)),
            (
                "email".to_string(),
                user.email
                    .as_ref()
                    .map_or("NULL".to_string(), |e| format!("'{}'", e)),
            ),
            (
                "password_hash".to_string(),
                format!("'{}'", user.password_hash),
            ),
            (
                "updated_at".to_string(),
                format!("'{}'", user.base.updated_at),
            ),
            (
                "updated_by".to_string(),
                format!("'{}'", user.base.updated_by),
            ),
        ]
    }

    fn from_row(_row: sea_orm::prelude::QueryResult) -> Result<User, DbErr> {
        // 这里需要实现从数据库行到 User 实体的转换
        // 暂时返回错误，表示需要实现
        Err(DbErr::Custom("需要实现查询结果映射".to_string()))
    }
}

pub struct UserRepositoryImpl {
    base_repo: BaseRepositoryImpl<User, UserMapper>,
    db: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            base_repo: BaseRepositoryImpl::new(db.clone()),
            db,
        }
    }

    async fn execute_find_by_field(&self, field: &str, value: &str) -> Result<Option<User>, DbErr> {
        let backend = self.db.get_database_backend();

        let sql = format!(
            "SELECT * FROM users WHERE {} = '{}' AND is_deleted = false",
            field, value
        );

        // 这里需要实现查询结果到 User 的转换
        // 暂时返回 None，表示需要实现结果映射
        Ok(None)
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<User, DbErr> {
        self.base_repo.create(user).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, DbErr> {
        self.base_repo.find_by_id(id).await
    }

    async fn find_by_phone(&self, phone: &str) -> Result<Option<User>, DbErr> {
        self.execute_find_by_field("phone", phone).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DbErr> {
        self.execute_find_by_field("email", email).await
    }

    async fn update(&self, user: User) -> Result<User, DbErr> {
        self.base_repo.update(user).await
    }

    async fn delete(&self, id: &str, deleted_by: &str) -> Result<(), DbErr> {
        self.base_repo.delete(id, deleted_by).await
    }

    async fn list(&self, page: u64, page_size: u64) -> Result<Vec<User>, DbErr> {
        self.base_repo.list(page, page_size).await
    }
}
