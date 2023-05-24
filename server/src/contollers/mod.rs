use axum::{routing::{get,post}, Router};

mod add_link;
mod redirector;

use crate::database::AppState;
use redirector::redirector;
use add_link::add_link_handler;

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/link/:id", get(redirector))
        .route("/link", post(add_link_handler))
        .with_state(state)
}