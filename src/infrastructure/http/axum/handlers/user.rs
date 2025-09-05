use std::sync::Arc;

use axum::response::IntoResponse;
use axum::{Json, extract::State};
use serde_json::json;

use crate::config::AppState;

pub async fn get_users(State(_): State<Arc<AppState>>) -> impl IntoResponse {
    Json(json!([]))
}
