use std::sync::Arc;

use axum::{Router, routing::get};

use crate::config::AppState;
use crate::infrastructure::http::axum::handlers::app;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(app::info))
}
