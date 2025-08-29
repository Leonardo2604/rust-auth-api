mod handlers;
mod models;
mod config;
mod routes;

use std::{sync::Arc};
use std::error::Error;

use dotenvy;
use tokio::net::TcpListener;

use config::{Database, Env, AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenvy::dotenv().ok();
  
  let env = Env::new();

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
