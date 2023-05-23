use crate::database::AppState;
use axum::extract::State;

pub async fn add_link_handler(
    State(mut state): State<AppState>,
){
    todo!();
}