use axum::{Json, extract::State, http::{self}};
use serde::Deserialize;
use crate::{AppState, internal::{database::users::UserType, *}};

#[derive(Deserialize)]
pub struct IncomingData {
    username: String,
    password: String
}

#[derive(serde::Serialize)]
pub struct MessageResponse {
    status: String,
    message: String
}

pub async fn register(State(state): State<AppState>, Json(payload): Json<IncomingData>) -> Json<MessageResponse> {
    match database::users::get_id(&state.pool, &payload.username).await {
        Err(_) => {},
        Ok(_) => return Json(MessageResponse {
            status: http::StatusCode::BAD_REQUEST.to_string(),
            message: "Username already being used".to_string()
        })
    }

    let user = UserType::from(payload.username, payload.password);

    match database::users::register_user(&state.pool, &user).await {
        Ok(_) => {},
        Err(_) => return Json(MessageResponse {
            status: http::StatusCode::BAD_REQUEST.to_string(),
            message: "Could not register user".to_string()
        })
    }

    Json(MessageResponse {
        status: http::StatusCode::CREATED.to_string(),
        message: "Account created".to_string()
    })
}