use std::sync::Arc;
use futures::lock::Mutex;

use crate::core::seed_to_id;

use super::DB;

#[derive(Clone)]
pub struct AppState {
    database: Arc<Mutex<DB>>,
    counter: Arc<Mutex<usize>>
}

impl AppState{

    pub async fn new() -> Self{
        let db = DB::new().await;
        let currernt_counter_value = db.load_counter().await;
        Self { 
            database: Arc::new(Mutex::new(db)), 
            counter: Arc::new(Mutex::new(currernt_counter_value)), 
        }
    }

    pub async fn add_link(&mut self, link: String) -> String{


        let database = self.database.lock().await;
        let mut counter = self.counter.lock().await;

        // TODO check if link exists in database

        let id = seed_to_id(*counter);
        database.set(id.clone(), link).await;
        *counter += 1;
        database.set_counter(*counter).await;

        id
    }

    pub async fn get_link_by_id(&self, id: &String) -> Option<String>{
        let database = self.database.lock().await;
        database.get(id).await
    }

}

#[cfg(test)]
mod tests {

    use super::AppState;
    #[tokio::test]
    async fn save_and_load() {
        let mut state = AppState::new().await;
        let test_url = String::from("https://google.com/something_for_test");
        let id = state.add_link(test_url.clone()).await;
        let url = state.get_link_by_id(&id).await;
        assert_eq!(url.unwrap(), test_url);
    }
}