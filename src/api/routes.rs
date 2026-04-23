use axum::{
    Router,
    routing::{get, post},
};

use super::users::create_user;
use crate::infrastructure::database::connection::get_db_connection;

pub fn create_api_routes() -> Router {
    let db = get_db_connection();
    Router::new()
        .route("/users/create", post(create_user))
        .with_state((*db).clone())
}
