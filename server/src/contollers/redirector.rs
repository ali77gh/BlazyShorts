use crate::database::AppState;
use axum::extract::State;

pub async fn redirector(
    State(state): State<AppState>,
) -> String {
    todo!();
}