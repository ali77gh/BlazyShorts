use serde_json::json;
use axum::{response::{Response, IntoResponse}, http::StatusCode};

pub fn error_to_response(msg: &str) -> Result<(),Response> {
    Err((StatusCode::BAD_REQUEST, json!( { "err" : msg } ).to_string()).into_response())
}