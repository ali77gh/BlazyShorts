use axum::{routing::get, Router,extract::State};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<u32>>
}

impl AppState{

    fn new() -> Self{
        Self { counter: Arc::new(Mutex::new(0u32.to_owned())), }
    }

    fn get(&mut self) -> u32{
        let mut data = self.counter.lock().expect("mutex was poisoned");
        *data += 1;
        *data
    }
}


#[tokio::main]
async fn main() {

    let state = AppState::new();
    let app = Router::new()
        .route("/", get(handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}



async fn handler(
    State(mut state): State<AppState>,
) -> String {
    format!("hello for {}th time", state.get())
}