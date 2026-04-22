use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 基础实体特征，包含所有实体共有的字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
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
    ///
    /// # Arguments
    ///
    /// * `created_by` - 创建用户的ID
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
    ///
    /// # Arguments
    ///
    /// * `updated_by` - 更新用户的ID
    pub fn update(&mut self, updated_by: String) {
        self.updated_at = Some(Utc::now());
        self.updated_by = Some(updated_by);
    }

    /// 软删除实体
    ///
    /// # Arguments
    ///
    /// * `deleted_by` - 删除用户的ID
    pub fn soft_delete(&mut self, deleted_by: String) {
        self.is_deleted = true;
        self.deleted_at = Some(Utc::now());
        self.deleted_by = Some(deleted_by);
    }
}

/// SeaORM 实体宏，用于为实体添加基础字段，支持字段校验
#[macro_export]
macro_rules! sea_orm_entity_with_base {
    // 基本用法：sea_orm_entity_with_base!(User, "users", name: String, phone: String)
    ($name:ident, $table_name:expr, $($field:ident: $type:ty),*) => {
        sea_orm_entity_with_base!(@inner $name, $table_name, $($field: $type => {}),*);
    };

    // 带校验的用法：sea_orm_entity_with_base!(User, "users", name: String => {length(min = 2, max = 50)}, phone: String => {regex(path = "PHONE_REGEX")})
    ($name:ident, $table_name:expr, $($field:ident: $type:ty => {$($attr:tt)*}),*) => {
        #[derive(Clone, Debug, PartialEq, sea_orm::entity::prelude::DeriveEntityModel, Eq, serde::Serialize, serde::Deserialize, validator::Validate)]
        #[sea_orm(table_name = $table_name)]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: String,

            pub created_at: chrono::DateTime<chrono::Utc>,
            pub created_by: String,

            pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
            pub updated_by: Option<String>,

            pub is_deleted: bool,
            pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
            pub deleted_by: Option<String>,

            $(
                #[validate($($attr)*)]
                pub $field: $type,
            )*
        }

        #[derive(Copy, Clone, Debug, sea_orm::entity::prelude::EnumIter, sea_orm::entity::prelude::DeriveRelation)]
        pub enum Relation {}

        impl sea_orm::entity::prelude::ActiveModelBehavior for ActiveModel {}

        impl Model {
            /// 从基础实体和特定字段创建模型
            pub fn from_base_and_fields(base: BaseEntity, $($field: $type),*) -> Self {
                Self {
                    id: base.id,
                    created_at: base.created_at,
                    created_by: base.created_by,
                    updated_at: base.updated_at,
                    updated_by: base.updated_by,
                    is_deleted: base.is_deleted,
                    deleted_at: base.deleted_at,
                    deleted_by: base.deleted_by,
                    $(
                        $field,
                    )*
                }
            }

            /// 转换为基础实体
            pub fn to_base_entity(&self) -> BaseEntity {
                BaseEntity {
                    id: self.id.clone(),
                    created_at: self.created_at,
                    created_by: self.created_by.clone(),
                    updated_at: self.updated_at,
                    updated_by: self.updated_by.clone(),
                    is_deleted: self.is_deleted,
                    deleted_at: self.deleted_at,
                    deleted_by: self.deleted_by.clone(),
                }
            }

            /// 更新基础字段
            pub fn update_base_fields(&self, updated_by: String) -> ActiveModel {
                let mut active_model: ActiveModel = self.clone().into();
                active_model.updated_at = sea_orm::Set(Some(chrono::Utc::now()));
                active_model.updated_by = sea_orm::Set(Some(updated_by));
                active_model
            }

            /// 软删除
            pub fn soft_delete_base(&self, deleted_by: String) -> ActiveModel {
                let mut active_model: ActiveModel = self.clone().into();
                active_model.is_deleted = sea_orm::Set(true);
                active_model.deleted_at = sea_orm::Set(Some(chrono::Utc::now()));
                active_model.deleted_by = sea_orm::Set(Some(deleted_by));
                active_model
            }
        }

        impl From<Model> for sea_orm::entity::prelude::ActiveModel {
            fn from(model: Model) -> Self {
                ActiveModel {
                    id: sea_orm::Set(model.id),
                    created_at: sea_orm::Set(model.created_at),
                    created_by: sea_orm::Set(model.created_by),
                    updated_at: sea_orm::Set(model.updated_at),
                    updated_by: sea_orm::Set(model.updated_by),
                    is_deleted: sea_orm::Set(model.is_deleted),
                    deleted_at: sea_orm::Set(model.deleted_at),
                    deleted_by: sea_orm::Set(model.deleted_by),
                    $(
                        $field: sea_orm::Set(model.$field),
                    )*
                }
            }
        }
    };

    // 内部实现
    (@inner $name:ident, $table_name:expr, $($field:ident: $type:ty => {$($attr:tt)*}),*) => {
        sea_orm_entity_with_base!($name, $table_name, $($field: $type => {$($attr)*}),*);
    };
}
