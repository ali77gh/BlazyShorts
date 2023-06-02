use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::core::seed_to_id;

#[derive(Clone)]
pub struct AppState {
    database: Arc<Mutex<HashMap<String,String>>>,
    counter: Arc<Mutex<usize>>
}

impl AppState{

    pub fn new() -> Self{
        let database = HashMap::new();
        Self { 
            database: Arc::new(Mutex::new(database)), 
            counter: Arc::new(Mutex::new(1usize.to_owned())), 
        }
    }

    pub fn add_link(&mut self, link: String) -> String{
        let mut database = self.database.lock().expect("mutex was poisoned");
        let mut counter = self.counter.lock().expect("mutex was poisoned");

        if let Some(id) = self.get_link_by_id(&link) {
            return id;
        }

        let id = seed_to_id(*counter);
        database.insert(id.clone(), link);
        *counter += 1;

        id
    }

    pub fn get_link_by_id(&self, id: &String) -> Option<String>{
        let database = self.database.lock().expect("mutex was poisoned");
        database.get(id).cloned()
    }
}

#[cfg(test)]
mod tests {

    use super::AppState;
    #[test]
    fn save_and_load() {
        let mut state = AppState::new();
        let test_url = String::from("https://google.com/something_for_test");
        let id = state.add_link(test_url.clone());
        let url = state.get_link_by_id(&id).unwrap();
        println!("{url}");
        println!("{test_url}");
        assert_eq!(url, test_url);
    }
}