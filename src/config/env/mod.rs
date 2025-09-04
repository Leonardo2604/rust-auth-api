mod database;
mod server;

use dotenvy;

pub use database::Database;
pub use server::Server;

#[derive(Clone, Debug)]
pub struct Env {
    database: Database,
    server: Server,
}

impl Env {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database: Database::new(),
            server: Server::new(),
        }
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn server(&self) -> &Server {
        &self.server
    }
}
