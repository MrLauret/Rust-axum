use dotenvy;
use tokio::{self, main};
use axum::{Router, routing::{get, post}};

mod internal;
use internal::handlers::handler::{*};
use internal::database::database;

#[main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let port = dotenvy::var("PORT").unwrap();

    let db_pool = database::init_db().await;

    let handler = Router::new()
        .route("/home", get(get_handler))
        .route("/home", post(post_handler))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();


    println!("Server running at 127.0.0.1:{port}");
    axum::serve(listener, handler).await.unwrap();
}