use crate::data::AppState;
use crate::core::id_generator::ALPHABET;
use axum::{response::{Redirect, IntoResponse, Response, Html},extract::{State,Path}, http::StatusCode};
use crate::core::error_to_response as e2r;


pub async fn redirector(
    State(state): State<AppState>,
    Path(id): Path<String>
) ->  Response {

    if let Err(err) = validate(&id) { return e2r(&err); }

    match state.get_link_by_id(&id).await{
        Some(url) => Redirect::permanent(&url).into_response(),
        None => {
            (StatusCode::NOT_FOUND, Html("<p>Not Found</p>")).into_response()
        }
    }
    
}

fn validate(path: &String) -> Result<(), String>{

    if path.len() > 10 { return Err("id is too long".to_string()); }

    for ch in path.chars(){
        if !ALPHABET.contains(ch){
            return Err("invalid char".to_string());
        }
    }

    Ok(())
}