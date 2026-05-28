use sqlx::{Pool, Postgres, PgPool};

pub async fn init_db() -> Pool<Postgres>{
    let url = dotenvy::var("DB_URL").unwrap();
    
    println!("Connecting to PostgreSQL...");

    let db_pool = PgPool::connect(&url).await.expect("Failed to connect to database");
    
    sqlx::migrate!("src/internal/database/migrations")
        .run(&db_pool)
        .await.unwrap();
    
    db_pool
}