use axum::Router;

use super::users::routes::*;

pub fn create_api_routes() -> Router {
    Router::new().nest("/users", users_api_routes())
}
