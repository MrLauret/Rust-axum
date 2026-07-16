use dotenvy;
use sqlx::PgPool;
use tokio::{self, main};
use axum::{Router, routing::{post, get}};

mod internal;
mod api;

use api::auth::{*};
use api::user::{*};
use internal::database::general;

use crate::internal::{middleware::protected::require_auth, services::jwt::AuthEngine};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    autheng: AuthEngine
}

#[main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let port = dotenvy::var("PORT").unwrap();

    let db_pool = general::init_db().await;

    let state = AppState {
        pool: db_pool,
        autheng: AuthEngine::new().unwrap()
    };

    let protected_routes = Router::new()
        .route("/api/user", get(user::profile_handler))
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), require_auth));

    let auth_routes = Router::new()
        .route("/api/login", post(login::login))
        .route("/api/refresh", post(refresh::refresh_token))
        .route("/api/register", post(register::register));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    let app = Router::new()
        .merge(protected_routes)
        .merge(auth_routes)
        .with_state(state);

    println!("Server running at 127.0.0.1:{port}");
    axum::serve(listener, app).await.unwrap();
}