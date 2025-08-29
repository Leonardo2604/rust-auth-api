use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool<Postgres>>,
}

impl AppState {
    pub fn new(pool: Arc<Pool<Postgres>>) -> AppState {
        AppState { pool }
    }
}
