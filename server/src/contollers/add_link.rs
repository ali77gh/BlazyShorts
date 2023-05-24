use crate::database::AppState;
use axum::{extract::{State, Json}, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{to_string,json};

#[derive(Deserialize)]
pub struct RequestBody{
    link: String,
}

#[derive(Serialize)]
pub struct ResponseSuccess{
    id: String
} 


pub async fn add_link_handler(
    State(mut state): State<AppState>,
    Json(payload): Json<RequestBody>
) -> (StatusCode, String){

    match state.add_link(payload.link) {
        Ok(id)   => { 
            (StatusCode::CREATED, to_string(&ResponseSuccess{id}).unwrap_or("err".to_string()) )
        }   
        Err(msg) => {
            #[cfg(debug_assertions)]
            dbg!(msg);
            (StatusCode::INTERNAL_SERVER_ERROR, json!({ "err": "INTERNAL_SERVER_ERROR" }).to_string() )
        },
    }
}