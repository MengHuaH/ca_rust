use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::domain::entities::user::{ActiveModel, Column, Entity as UsersEntity, Model};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: ActiveModel) -> Result<Model, DbErr>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Model>, DbErr>;
    async fn find_by_phone(&self, phone: &str) -> Result<Option<Model>, DbErr>;
    async fn find_by_email(&self, email: &str) -> Result<Option<Model>, DbErr>;
    async fn update(&self, user: ActiveModel) -> Result<Model, DbErr>;
    async fn delete(&self, id: &str, deleted_by: &str) -> Result<(), DbErr>;
    async fn list(&self, page: u64, page_size: u64) -> Result<Vec<Model>, DbErr>;
}

pub struct UserRepositoryImpl {
    db: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: ActiveModel) -> Result<Model, DbErr> {
        user.insert(&self.db).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Model>, DbErr> {
        UsersEntity::find_by_id(id)
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
    }

    async fn find_by_phone(&self, phone: &str) -> Result<Option<Model>, DbErr> {
        UsersEntity::find()
            .filter(Column::Phone.eq(phone))
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Model>, DbErr> {
        UsersEntity::find()
            .filter(Column::Email.eq(email))
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
    }

    async fn update(&self, user: ActiveModel) -> Result<Model, DbErr> {
        user.update(&self.db).await
    }

    async fn delete(&self, id: &str, deleted_by: &str) -> Result<(), DbErr> {
        let user = UsersEntity::find_by_id(id)
            .filter(Column::IsDeleted.eq(false))
            .one(&self.db)
            .await?;

        if let Some(user) = user {
            let active_model = user.soft_delete(deleted_by.to_string());
            active_model.update(&self.db).await?;
        }

        Ok(())
    }

    async fn list(&self, page: u64, page_size: u64) -> Result<Vec<Model>, DbErr> {
        UsersEntity::find()
            .filter(Column::IsDeleted.eq(false))
            .order_by_asc(Column::CreatedAt)
            .paginate(&self.db, page_size)
            .fetch_page(page)
            .await
    }
}
