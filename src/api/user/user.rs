use serde::{Serialize};
use axum::{extract::State, Extension, Json};
use crate::{AppState, internal::database::users::get_username};

#[derive(Serialize)]
pub struct MessageResponse {
    id: i32,
    username: String
}

pub async fn profile_handler(
    Extension(user_id): Extension<i32>,
    State(state): State<AppState>
) -> Json<MessageResponse> {
    Json(MessageResponse {
        id: user_id,
        username: get_username(&state.pool, user_id).await.expect("No user found")
    })
}