use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::application::{ResponseService, create_response_service};
use crate::domain::PaginationRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

pub async fn list_users(
    Query(query): Query<UserListQuery>,
) -> Json<crate::domain::PaginatedResponse<User>> {
    info!("List users endpoint called with page: {:?}, page_size: {:?}", 
          query.page, query.page_size);
    
    let response_service = create_response_service();
    let pagination_request = response_service.create_pagination_request(query.page, query.page_size);
    
    // 模拟数据 - 在实际应用中这里会从数据库获取
    let total_items = 100;
    let page = pagination_request.get_page();
    let page_size = pagination_request.get_page_size();
    let offset = pagination_request.get_offset();
    
    let mut users = Vec::new();
    for i in 0..page_size {
        let user_id = offset + i as u64 + 1;
        users.push(User {
            id: user_id as u32,
            name: format!("User {}", user_id),
            email: format!("user{}@example.com", user_id),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        });
    }
    
    response_service.paginated(users, pagination_request, total_items)
}

pub async fn get_user_by_id(
    axum::extract::Path(user_id): axum::extract::Path<u32>,
) -> Json<crate::domain::ApiResponse<User>> {
    info!("Get user by id endpoint called: {}", user_id);
    
    let response_service = create_response_service();
    
    // 模拟数据 - 在实际应用中这里会从数据库获取
    let user = User {
        id: user_id,
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
        created_at: "2024-01-01T00:00:00Z".to_string(),
    };
    
    response_service.success(user)
}