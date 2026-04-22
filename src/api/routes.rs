use axum::{
    Router,
    routing::{get, post},
};

use super::users::{get_user_by_id, list_users};

pub fn create_api_routes() -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user_by_id))
}
