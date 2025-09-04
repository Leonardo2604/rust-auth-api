use axum::{Router, routing::get};

use crate::config::AppState;
use crate::infrastructure::http::axum::handlers::user;

pub fn router() -> Router<AppState> {
    let router: Router<AppState> = Router::new().route("/", get(user::get_users));

    Router::new().nest("/users", router)
}
