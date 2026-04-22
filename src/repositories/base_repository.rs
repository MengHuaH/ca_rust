use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr, Statement};
use std::marker::PhantomData;

#[async_trait]
pub trait BaseRepository<T>: Send + Sync {
    async fn create(&self, entity: T) -> Result<T, DbErr>;
    async fn find_by_id(&self, id: &str) -> Result<Option<T>, DbErr>;
    async fn update(&self, entity: T) -> Result<T, DbErr>;
    async fn delete(&self, id: &str, deleted_by: &str) -> Result<(), DbErr>;
    async fn list(&self, page: u64, page_size: u64) -> Result<Vec<T>, DbErr>;
}

pub trait EntityMapper<T> {
    fn table_name() -> &'static str;
    fn id_column() -> &'static str;
    fn to_insert_params(entity: &T) -> Vec<(String, String)>;
    fn to_update_params(entity: &T) -> Vec<(String, String)>;
    fn from_row(row: sea_orm::prelude::QueryResult) -> Result<T, DbErr>;
}

pub struct BaseRepositoryImpl<T, M> {
    db: DatabaseConnection,
    _phantom: PhantomData<(T, M)>,
}

impl<T, M> BaseRepositoryImpl<T, M> 
where 
    T: Clone + Send + Sync,
    M: EntityMapper<T> + Send + Sync,
{
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            _phantom: PhantomData,
        }
    }

    async fn execute_insert(&self, entity: &T) -> Result<T, DbErr> {
        let backend = self.db.get_database_backend();
        let table_name = M::table_name();
        let params = M::to_insert_params(entity);
        
        let columns: Vec<String> = params.iter().map(|(k, _)| k.clone()).collect();
        let values: Vec<String> = params.iter().map(|(_, v)| v.clone()).collect();
        
        let sql = match backend {
            sea_orm::DatabaseBackend::Postgres => {
                format!(
                    "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
                    table_name,
                    columns.join(", "),
                    values.join(", ")
                )
            }
            _ => {
                format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    table_name,
                    columns.join(", "),
                    values.join(", ")
                )
            }
        };

        let result = self.db.execute(Statement::from_string(backend, sql)).await?;
        // 这里需要处理返回结果并映射到实体
        // 暂时返回克隆的实体
        Ok(entity.clone())
    }

    async fn execute_find_by_id(&self, id: &str) -> Result<Option<T>, DbErr> {
        let backend = self.db.get_database_backend();
        let table_name = M::table_name();
        let id_column = M::id_column();
        
        let sql = format!(
            "SELECT * FROM {} WHERE {} = '{}' AND is_deleted = false",
            table_name, id_column, id
        );

        // 这里需要实现查询结果到实体的转换
        // 暂时返回 None，表示需要实现结果映射
        Ok(None)
    }

    async fn execute_update(&self, entity: &T) -> Result<T, DbErr> {
        let backend = self.db.get_database_backend();
        let table_name = M::table_name();
        let id_column = M::id_column();
        let params = M::to_update_params(entity);
        
        let set_clause: Vec<String> = params.iter().map(|(k, v)| format!("{} = {}", k, v)).collect();
        
        let sql = format!(
            "UPDATE {} SET {} WHERE {} = '{}' AND is_deleted = false",
            table_name,
            set_clause.join(", "),
            id_column,
            // 这里需要获取实体的 ID
            "" // 暂时为空，需要实现 ID 获取
        );

        self.db.execute(Statement::from_string(backend, sql)).await?;
        Ok(entity.clone())
    }

    async fn execute_soft_delete(&self, id: &str, deleted_by: &str) -> Result<(), DbErr> {
        let backend = self.db.get_database_backend();
        let table_name = M::table_name();
        let id_column = M::id_column();
        let now = chrono::Utc::now();
        
        let sql = format!(
            "UPDATE {} SET is_deleted = true, deleted_at = '{}', deleted_by = '{}' WHERE {} = '{}'",
            table_name, now, deleted_by, id_column, id
        );

        self.db.execute(Statement::from_string(backend, sql)).await?;
        Ok(())
    }

    async fn execute_list(&self, page: u64, page_size: u64) -> Result<Vec<T>, DbErr> {
        let backend = self.db.get_database_backend();
        let table_name = M::table_name();
        let offset = page * page_size;
        
        let sql = format!(
            "SELECT * FROM {} WHERE is_deleted = false ORDER BY created_at LIMIT {} OFFSET {}",
            table_name, page_size, offset
        );

        // 这里需要实现查询结果到实体列表的转换
        // 暂时返回空列表，表示需要实现结果映射
        Ok(vec![])
    }
}

#[async_trait]
impl<T, M> BaseRepository<T> for BaseRepositoryImpl<T, M> 
where 
    T: Clone + Send + Sync + 'static,
    M: EntityMapper<T> + Send + Sync + 'static,
{
    async fn create(&self, entity: T) -> Result<T, DbErr> {
        self.execute_insert(&entity).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<T>, DbErr> {
        self.execute_find_by_id(id).await
    }

    async fn update(&self, entity: T) -> Result<T, DbErr> {
        self.execute_update(&entity).await
    }

    async fn delete(&self, id: &str, deleted_by: &str) -> Result<(), DbErr> {
        self.execute_soft_delete(id, deleted_by).await
    }

    async fn list(&self, page: u64, page_size: u64) -> Result<Vec<T>, DbErr> {
        self.execute_list(page, page_size).await
    }
}