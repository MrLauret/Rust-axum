use dotenvy;
use sqlx::PgPool;
use tokio::{self, main};
use axum::{Router, extract::State, routing::{self, get, post, }};

mod internal;
mod api;

use api::auth::{*};
use internal::database::general;

use crate::internal::services::jwt::AuthEngine;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    autheng: AuthEngine
}

#[main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let port = dotenvy::var("DB_PORT").unwrap();

    let db_pool = general::init_db().await;

    let state = AppState {
        pool: db_pool,
        autheng: AuthEngine::new().unwrap()
    };

    let handler = Router::new()
        .route("/api/login", post(login::login))
        .route("/api/refresh", post(refresh::refresh_token))
        .route("/api/register", post(register::register))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();


    println!("Server running at 127.0.0.1:{port}");
    axum::serve(listener, handler).await.unwrap();
}