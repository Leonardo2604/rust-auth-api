use axum::response::IntoResponse;
use axum::{Json, extract::State};
use sqlx;

use crate::models::User;

use crate::config::AppState;

pub async fn get_users(State(app): State<AppState>) -> impl IntoResponse {
    let users: Vec<User> = sqlx::query_as::<_, User>(
        "select id, name, email, password, created_at, updated_at, deleted_at from users",
    )
    .fetch_all(&*app.pool)
    .await
    .expect("Falha ao tentar obter os usuários");

    Json(users)
}
