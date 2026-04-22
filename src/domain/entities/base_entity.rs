use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 基础实体，包含所有实体共有的字段
/// 支持 SeaORM 字段映射
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub created_at: DateTime<Utc>,
    pub created_by: String,

    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,

    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}

impl BaseEntity {
    /// 创建一个新的基础实体
    pub fn new(created_by: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: now,
            created_by,
            updated_at: None,
            updated_by: None,
            is_deleted: false,
            deleted_at: None,
            deleted_by: None,
        }
    }

    /// 更新基础实体
    pub fn update(&mut self, updated_by: String) {
        self.updated_at = Some(Utc::now());
        self.updated_by = Some(updated_by);
    }

    /// 软删除实体
    pub fn soft_delete(&mut self, deleted_by: String) {
        self.is_deleted = true;
        self.deleted_at = Some(Utc::now());
        self.deleted_by = Some(deleted_by);
    }

    /// 转换为 ActiveModel 的基础字段
    pub fn to_active_model(&self) -> BaseActiveModel {
        BaseActiveModel {
            id: Set(self.id.clone()),
            created_at: Set(self.created_at),
            created_by: Set(self.created_by.clone()),
            updated_at: Set(self.updated_at),
            updated_by: Set(self.updated_by.clone()),
            is_deleted: Set(self.is_deleted),
            deleted_at: Set(self.deleted_at),
            deleted_by: Set(self.deleted_by.clone()),
        }
    }
}

/// 基础字段的 ActiveModel 表示
#[derive(Debug, Clone)]
pub struct BaseActiveModel {
    pub id: Set<String>,
    pub created_at: Set<DateTime<Utc>>,
    pub created_by: Set<String>,
    pub updated_at: Set<Option<DateTime<Utc>>>,
    pub updated_by: Set<Option<String>>,
    pub is_deleted: Set<bool>,
    pub deleted_at: Set<Option<DateTime<Utc>>>,
    pub deleted_by: Set<Option<String>>,
}

impl BaseActiveModel {
    /// 应用到目标 ActiveModel
    pub fn apply_to<T: ActiveModelTrait>(&self, target: &mut T) {
        // 这里需要根据具体的 ActiveModel 类型来设置字段
        // 由于 Rust 的类型系统限制，这个实现需要更复杂的类型处理
        // 暂时保留这个接口，实际使用中可能需要其他方式
    }
}
