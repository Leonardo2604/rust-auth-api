use axum::{Router, routing::get};

use crate::config::AppState;
use crate::handlers::app;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(app::info))
}
