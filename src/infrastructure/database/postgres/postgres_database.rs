use crate::config::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub struct PostgresDatabase {
    env: env::Database,
}

impl PostgresDatabase {
    pub fn new(env: env::Database) -> Self {
        Self { env }
    }

    pub async fn connect(&self) -> Result<Pool<Postgres>, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(self.env.max_connections())
            .connect(&self.url())
            .await
    }

    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.env.username(),
            self.env.password(),
            self.env.host(),
            self.env.port(),
            self.env.name()
        )
    }
}
