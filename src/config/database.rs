use crate::config::env::DatabaseEnv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub struct Database {
    env: DatabaseEnv,
}

impl Database {
    pub fn new(env: DatabaseEnv) -> Database {
        Database { env }
    }

    pub async fn connect(&self) -> Result<Pool<Postgres>, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(self.env.url())
            .await
    }
}
