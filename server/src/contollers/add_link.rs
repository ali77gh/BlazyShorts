use crate::data::AppState;
use axum::{extract::State, http::StatusCode,response::{Response, IntoResponse}};
use crate::core::error_to_response as e2r;
use common::api::{add_link_api::{AddLinkApi, ResponseSuccess}, BaseApi};

pub async fn add_link_handler(
    State(mut state): State<AppState>,
    body: String
) -> Response {

    match AddLinkApi::parse_and_validate(&body){
        Err(e)=> e2r(&e),
        Ok(req_body) => {
            let id = state.add_link(req_body.link).await;
            (
                StatusCode::CREATED,
                AddLinkApi::create_res(ResponseSuccess{id})
            ).into_response()
        }
    }
    
}
