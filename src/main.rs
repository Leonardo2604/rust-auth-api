mod config;
mod handlers;
mod models;
mod routes;

use std::error::Error;
use std::sync::Arc;

use tokio::net::TcpListener;

use config::{AppState, Database, Env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = Env::load();

    let pool = Database::new(env.database().clone()).connect().await?;

    let arc_pool = Arc::new(pool);

    let app_state = AppState::new(arc_pool);

    let router = routes::router(app_state);

    let addr = env.server().addr();
    let listener = TcpListener::bind(addr).await?;

    println!("Servidor rodando em http://{}", addr);

    axum::serve(listener, router).await?;

    Ok(())
}
