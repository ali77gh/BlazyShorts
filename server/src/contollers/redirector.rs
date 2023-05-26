use crate::database::AppState;
use crate::core::id_generator::ALPHABET;
use axum::{response::{Redirect, IntoResponse, Response, Html},extract::{State,Path}, http::StatusCode};
use crate::core::error_to_response::error_to_response as e2r;


pub async fn redirector(
    State(state): State<AppState>,
    Path(id): Path<String>
) ->  Response {

    if let Err(err) = validate(&id) { return err; }

    match state.get_link_by_id(&id){
        Some(url) => Redirect::permanent(&url).into_response(),
        None => {
            (StatusCode::NOT_FOUND, Html("<p>Not Found</p>")).into_response()
        }
    }
    
}

fn validate(path: &String) -> Result<(), Response>{

    if path.len() > 10 { return e2r("id is too long"); }

    for ch in path.chars(){
        if !ALPHABET.contains(ch){
            return e2r("invalid char");
        }
    }

    Ok(())
}