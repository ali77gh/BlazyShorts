use std::net::SocketAddr;

mod database;
mod contollers;

use crate::database::AppState;
use crate::contollers::get_router;

#[tokio::main]
async fn main() {

    let state = AppState::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(get_router(state).into_make_service())
        .await
        .unwrap();
}
