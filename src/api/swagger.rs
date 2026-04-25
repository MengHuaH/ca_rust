use axum::Router;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::swaggerApiResponseString::ApiResponseString;

use crate::application::ChangePasswordCommand;
use crate::application::CreateUserCommand;
use crate::application::DeleteUserCommand;
use crate::application::UpdateUserCommand;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "CA API",
        description = "CA 用户管理API",
        version = "0.1.0",
        contact(
            name = "CA Team",
            email = "team@ca.com"
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
        crate::api::users::create::create_user,
        crate::api::users::update::update_user,
        crate::api::users::delete::delete_user,
        crate::api::users::change_password::change_password,
    ),
    components(
        schemas(ApiResponseString),
        schemas(UpdateUserCommand, CreateUserCommand, DeleteUserCommand, ChangePasswordCommand),
    ),
    servers(
        (description = "API服务器")
    )
)]
pub struct ApiDoc;

use std::env;

pub fn create_swagger_routes() -> Router {
    let server_url = env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let swagger_ui_route = env::var("SWAGGER_UI_URL").unwrap_or_else(|_| "/swagger-ui".to_string());
    let swagger_ui_openapi_url =
        env::var("SWAGGER_UI_OPENAPI_URL").unwrap_or_else(|_| "/api-docs/openapi.json".to_string());
    let swagger_ui_router = SwaggerUi::new(swagger_ui_route.clone())
        .url(swagger_ui_openapi_url.clone(), ApiDoc::openapi())
        .into();
    info!(
        "swagger-ui路由: http://{}:{}{}",
        server_url, server_port, swagger_ui_route
    );
    info!(
        "swagger-ui-openapi-url: http://{}:{}{}",
        server_url, server_port, swagger_ui_openapi_url
    );
    swagger_ui_router
}
