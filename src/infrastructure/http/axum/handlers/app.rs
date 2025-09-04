use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn info() -> impl IntoResponse {
    Json(json!({
        "name": "auth-api",
        "version": "1.0.0"
    }))
}
