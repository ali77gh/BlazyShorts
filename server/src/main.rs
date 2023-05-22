use axum::{routing::get, routing::post, Router, extract::State};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone)]
struct AppState {
    database: Arc<Mutex<HashMap<String,String>>>,
    counter: Arc<Mutex<u64>>
}

impl AppState{

    fn new() -> Self{
        let database = HashMap::new();
        Self { 
            database: Arc::new(Mutex::new(database.to_owned())), 
            counter: Arc::new(Mutex::new(0u64.to_owned())), 
        }
    }

    fn add_link(&mut self, link: String) -> Result<String, String>{
        let mut database = self.database.lock().expect("mutex was poisoned");
        let counter = self.counter.lock().expect("mutex was poisoned");
        database.insert(counter.to_string(), link);
        Ok(counter.to_string())
    }

    fn get_link_by_id(&mut self, id: String) -> Result<String, String>{
        let database = self.database.lock().expect("mutex was poisoned");
        database.get(&id).ok_or(String::from("Not Found")).cloned()
    }
}


#[tokio::main]
async fn main() {

    let state = AppState::new();
    let app = Router::new()
        .route("/link", get(get_link_handler))
        .route("/link", post(add_link_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn add_link_handler(
    State(mut state): State<AppState>,
){
    todo!();
}

async fn get_link_handler(
    State(state): State<AppState>,
) -> String {
    todo!();
}


#[cfg(test)]
mod tests {

    use super::AppState;
    #[test]
    fn save_and_load() {
        let mut state = AppState::new();
        let test_url = String::from("google.com/something_for_test");
        let id = state.add_link(test_url.clone()).unwrap();
        let url = state.get_link_by_id(id).unwrap();
        println!("{url}");
        println!("{test_url}");
        assert_eq!(url, test_url);
    }
}