use crate::database::AppState;
use axum::{extract::{State, Json}, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

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
    let id = state.add_link(payload.link);
    (StatusCode::CREATED, to_string(&ResponseSuccess{id}).unwrap_or("err".to_string()) )
}