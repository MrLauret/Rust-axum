use axum::{Json, extract::State, http::{self, StatusCode}};
use serde::Deserialize;
use crate::{AppState, internal::*};

#[derive(Deserialize)]
pub struct IncomingData {
    username: String,
    password: String
}

#[derive(serde::Serialize)]
pub struct MessageResponse {
    status: String,
    access_token: String,
    refresh_token: String
}

pub async fn login(State(state): State<AppState>, Json(payload): Json<IncomingData>) -> Json<MessageResponse> {
    let tokens = match services::auth::login( &state.pool, &state.autheng, payload.username, payload.password).await {
        Ok(tokens) => tokens,
        Err(_) => {
            return Json(MessageResponse {
                status: http::StatusCode::BAD_REQUEST.to_string(),
                access_token: String::new(),
                refresh_token: String::new()
            });
        }
    };
    
    Json(MessageResponse {
        status: StatusCode::OK.to_string(),
        access_token: tokens.0,
        refresh_token: tokens.1
    })
}