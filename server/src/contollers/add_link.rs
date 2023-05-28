use crate::database::AppState;
use axum::{extract::{State, Json}, http::StatusCode,response::{Response, IntoResponse}};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use crate::core::error_to_response as e2r;


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
) -> Response {

    if let Err(err) = validate(&payload) { return err; }

    let id = state.add_link(payload.link);
    (StatusCode::CREATED, to_string(&ResponseSuccess{id}).unwrap_or("err".to_string()).into_response() ).into_response()
}

use url::Url;
fn validate(body: &RequestBody) -> Result<(), Response>{

    if body.link.len() > 100 { return e2r("id len is too long"); }

    match Url::parse(&body.link) {
        Err(err) => e2r(&err.to_string()),
        Ok(url) => {
            let scheme = url.scheme();
            if scheme != "http" && scheme != "https" {
                return e2r("url scheme not supported");
            }
            
            Ok(())
        }
    }

}