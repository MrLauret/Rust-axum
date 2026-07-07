use std::os::linux::raw::stat;

use axum::{Json, extract::State, http::{self}};
use serde::{Deserialize, Serialize};
use crate::{AppState, internal::services::auth::refresh_session};

#[derive(Deserialize)]
pub struct IncomingData {
    access_token: String,
    refresh_token: String
}

#[derive(Serialize)]
pub struct MessageResponse {
    status: String,
    access_token: String
}

pub async fn refresh_token(State(state): State<AppState>, Json(payload): Json<IncomingData>) -> Json<MessageResponse> {
    let id = match state.autheng.get_id_from_expired_token(&payload.access_token) {
        Ok(id) => id,
        Err(_) => return Json(MessageResponse {
            status: http::StatusCode::BAD_REQUEST.to_string(),
            access_token: String::new()
        })
    };

    let access_token = match refresh_session(&state.pool, &payload.refresh_token, &state.autheng, &id).await {
        Ok(token) => token,
        Err(_) => return Json(MessageResponse {
            status: http::StatusCode::BAD_REQUEST.to_string(),
            access_token: String::new()
        })
    };

    Json(MessageResponse {
        status: http::StatusCode::OK.to_string(),
        access_token: access_token
    })
}