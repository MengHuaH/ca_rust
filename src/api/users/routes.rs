use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

use super::create::create_user;
use super::delete::delete_user;
use super::update::update_user;
use crate::infrastructure::database::connection::get_db_connection;

pub fn users_api_routes() -> Router {
    let db = get_db_connection();
    Router::new()
        .route("/create", post(create_user))
        .route("/update/:user_id", put(update_user))
        .route("/delete/:user_id", delete(delete_user))
        .with_state((*db).clone())
}
