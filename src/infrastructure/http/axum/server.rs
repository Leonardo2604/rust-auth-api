use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::oneshot;
use axum::Router;

use crate::config::AppState;
use crate::infrastructure::http::axum::routes;

pub struct AxumServer {
    addr: SocketAddr,
    app_state: Arc<AppState>,
}

impl AxumServer {
    pub fn new(addr: SocketAddr, app_state: AppState) -> Self {
        Self { addr, app_state: Arc::new(app_state) }
    }

    pub async fn start(self) -> Result<oneshot::Sender<()>, Box<dyn std::error::Error>> {
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let app = routes::router(self.app_state);

        let listener = TcpListener::bind(self.addr).await?;
        let local_addr = listener.local_addr()?;

        println!("Server running at {}", local_addr);

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)?
                .serve(app.into_make_service())
                .with_graceful_shutdown(async {
                    shutdown_rx.await.ok();
                })
                .with_timeout(std::time::Duration::from_secs(30))
                .await?;
        });

        Ok(shutdown_tx)
    }
}
