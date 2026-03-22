use axum::{
    Router,
    routing::{get, post},
};

use super::echo::echo_message;
use super::health::health_check;
use super::hello::hello_world;
use super::system_info::system_info_handler;
use super::users::{get_user_by_id, list_users};

pub fn create_api_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/echo", post(echo_message))
        .route("/hello", get(hello_world))
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user_by_id))
        .route("/system-info", get(system_info_handler))
}
