use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

impl AppState {
    pub fn new(db_pool: Pool<Postgres>) -> AppState {
        AppState { db_pool }
    }
}
