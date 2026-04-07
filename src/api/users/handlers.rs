use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::IntoParams;

use crate::application::{ResponseService, create_response_service};
use crate::domain::PaginationRequest;

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct User {
    /// 用户ID
    pub id: u32,
    /// 用户名
    pub name: String,
    /// 用户邮箱
    pub email: String,
    /// 创建时间
    pub created_at: String,
}

#[derive(Debug, Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
pub struct UserListQuery {
    /// 页码，默认为1
    pub page: Option<u32>,
    /// 每页数量，默认为20，最大100
    pub page_size: Option<u32>,
}

/// 获取用户列表
///
/// 分页获取用户列表，支持页码和每页数量参数
#[utoipa::path(
    get,
    path = "/api/users",
    tag = "users",
    params(UserListQuery),
    responses(
        (status = 200, description = "成功获取用户列表", body = PaginatedResponse<User>),
        (status = 400, description = "请求参数错误")
    )
)]
pub async fn list_users(
    Query(query): Query<UserListQuery>,
) -> Json<crate::domain::PaginatedResponse<User>> {
    info!(
        "List users endpoint called with page: {:?}, page_size: {:?}",
        query.page, query.page_size
    );

    let response_service = create_response_service();
    let pagination_request =
        response_service.create_pagination_request(query.page, query.page_size);

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

/// 根据用户ID获取用户信息
///
/// 通过用户ID获取特定用户的详细信息
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = u32, Path, description = "用户ID")
    ),
    responses(
        (status = 200, description = "成功获取用户信息", body = ApiResponse<User>),
        (status = 404, description = "用户不存在")
    )
)]
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
