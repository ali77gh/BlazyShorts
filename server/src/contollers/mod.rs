
mod add_link;
mod redirector;

use axum::{routing::{get,post}, Router, middleware};
use crate::{database::AppState, middleware::logger::logger};
use redirector::redirector;
use add_link::add_link_handler;

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/link/:id", get(redirector))
        .route("/link", post(add_link_handler))
        .route_layer(middleware::from_fn(logger))
        .with_state(state)
}