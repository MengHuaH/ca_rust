use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::users::handlers::{User, UserListQuery};
use crate::application::{hello::service::HelloResponse, system_info::service::SystemInfoResponse};
use crate::domain::responses::ApiResponse;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "VisualEngine API",
        description = "VisualEngine 图像处理和分析API",
        version = "0.1.0",
        contact(
            name = "VisualEngine Team",
            email = "team@visualengine.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    tags(
        (name = "users", description = "用户管理"),

    ),
    paths(
        crate::api::users::handlers::list_users,
        crate::api::users::handlers::get_user_by_id,
    ),
    components(
        schemas(User, UserListQuery, SystemInfoResponse)
    ),
    servers(
        (description = "API服务器")
    )
)]
pub struct ApiDoc;

pub fn create_swagger_routes() -> Router {
    // 使用正确的路由格式
    SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
        .into()
}
