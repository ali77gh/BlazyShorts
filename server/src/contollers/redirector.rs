use crate::database::AppState;
use axum::{response::{Redirect, IntoResponse, Response, Html},extract::{State,Path}, http::StatusCode};


// TODO: input validator on Path (should starts with http:// or https://) 

pub async fn redirector(
    State(state): State<AppState>,
    Path(id): Path<String>
) ->  Response {

    match state.get_link_by_id(id){
        Some(url) => Redirect::permanent(&url).into_response(),
        None => {
            (StatusCode::NOT_FOUND, Html("<p>Not Found</p>")).into_response()
        }
    }
    
}