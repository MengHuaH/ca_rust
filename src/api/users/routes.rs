use axum::{
    Router,
    routing::{get, post},
};

use super::handlers::create_user;
use crate::infrastructure::database::connection::get_db_connection;

pub fn users_api_routes() -> Router {
    let db = get_db_connection();
    Router::new()
        .route("/create", post(create_user))
        .with_state((*db).clone())
}
