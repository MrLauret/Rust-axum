use axum::{Json, http::{HeaderMap, StatusCode}};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IncomingData {
    name: String
}

#[derive(serde::Serialize)]
pub struct MessageResponse {
    status: String,
    msg: String
}

pub async fn post_handler(_headers: HeaderMap, Json(payload): Json<IncomingData>) -> Json<MessageResponse> {
    Json(MessageResponse {
        status: StatusCode::OK.to_string(),
        msg: format!("hello {}!", payload.name).to_string(),
    })
}

pub async fn get_handler() -> Json<MessageResponse> {
    let response = MessageResponse {
        status: StatusCode::OK.to_string(),
        msg: "Hello!".to_string()
    };

    Json(response)
}