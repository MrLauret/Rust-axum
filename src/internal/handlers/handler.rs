use axum::{Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IncomingData {
    name: String
}

#[derive(serde::Serialize)]
pub struct MessageResponse {
    msg: String
}

pub async fn post_handler(Json(payload): Json<IncomingData>) -> Json<MessageResponse> {
    let response = MessageResponse {
        msg: format!("hello {}!", payload.name).to_string()
    };

    Json(response)
}

pub async fn get_handler() -> Json<MessageResponse> {
    let response = MessageResponse {
        msg: "Hello!".to_string()
    };

    Json(response)
}