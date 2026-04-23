use axum::Router;

use super::users::routes::*;

pub fn api_routes() -> Router {
    Router::new().nest("/users", users_api_routes())
}
