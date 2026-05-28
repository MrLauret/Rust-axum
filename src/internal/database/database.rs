use sqlx::{self, PgPool, Pool, Postgres, Row};

pub async fn init_db() -> Pool<Postgres>{
    let url = dotenvy::var("DB_URL").unwrap();
    
    println!("Connecting to PostgreSQL...");

    let db_pool = PgPool::connect(&url).await.expect("Failed to connect to database");
    
    sqlx::migrate!("src/internal/database/migrations")
        .run(&db_pool)
        .await.unwrap();
    
    db_pool
}

pub async fn get_user_id(pool: PgPool, username: String) -> Result<i32, String> {
    let id: i32 = sqlx::query("SELECT id FROM Users WHERE username = $1")
        .bind(username)
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?
        .get("id");

    Ok(id)
}

pub async fn get_username(pool: PgPool, id: i32) -> Result<String, String> {
    let username = sqlx::query("SELECT username FROM Users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?
        .get("username");

    Ok(username)
}