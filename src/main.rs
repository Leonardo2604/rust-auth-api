mod domain;
mod infrastructure;
mod application;
mod config;

use std::error::Error;
use std::sync::Arc;

use tokio::{sync::oneshot, signal};

use config::{AppState, Env};
use infrastructure::http::axum::AxumServer;
use infrastructure::database::postgres::PostgresDatabase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = Env::load();

    let db_pool = PostgresDatabase::new(env.database().clone()).connect().await?;

    let app_state = AppState { db_pool };

    let server = AxumServer::new(env.server().addr(), app_state);

    let shutdown_tx = server.start().await?;

    signal::ctrl_c().await?;

    println!("Shutting down server...");

    shutdown_tx.send(()).unwrap();

    Ok(())
}
