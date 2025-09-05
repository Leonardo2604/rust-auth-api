mod app;
mod user;

use std::sync::Arc;

use axum::Router;

use crate::config::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    let v1_router = Router::new().merge(user::router());

    Router::new()
        .merge(app::router())
        .nest("/v1", v1_router)
        .with_state(state)
}
