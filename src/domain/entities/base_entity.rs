use chrono::{DateTime, Utc}; // 时间库
use serde::{Deserialize, Serialize};
use uuid::Uuid; // UUID 库

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
}

impl BaseEntity {
    /// 创建一个新的基础实体
    ///
    /// # Arguments
    ///
    /// * `created_by` - 创建用户的ID
    pub fn new(created_by: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(), // 使用 v4 替代 v7，更稳定
            created_at: now,
            created_by,
            updated_at: None,
            updated_by: None,
        }
    }

    /// 更新基础实体
    ///
    /// # Arguments
    ///
    /// * `updated_by` - 更新用户的ID
    pub fn update(&mut self, updated_by: String) {
        self.updated_at = Some(Utc::now());
        self.updated_by = Some(updated_by);
    }
}
