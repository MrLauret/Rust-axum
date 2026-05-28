use sqlx::{self, PgPool, Row};
use argon2::{password_hash};
use password_hash::PasswordHasher;

pub struct UserType {
    pub username: String,
    pub password: String
}

impl UserType {
    pub fn from(username: String, password: String) -> Self {
        let salt = password_hash::SaltString::generate(&mut password_hash::rand_core::OsRng);
        let password_hash = argon2::Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap();

        Self { username: username, password: password_hash.to_string() }
    }
}

pub async fn register_user(pool: &PgPool, user: &UserType) -> Result<(), String> {
    if get_id(pool, user.username.clone()).await.is_ok() {
        return Err("Username already exists".to_string());
    }

    match sqlx::query("INSERT INTO Users (username, password_hash) VALUES ($1, $2)")
        .bind(user.username.clone())
        .bind(user.password.clone())
        .execute(pool).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

pub async fn get_id(pool: &PgPool, username: String) -> Result<i32, String> {
    let id: i32 = sqlx::query("SELECT id FROM Users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
        .get("id");

    Ok(id)
}

pub async fn get_username(pool: &PgPool, id: i32) -> Result<String, String> {
    let username = sqlx::query("SELECT username FROM Users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
        .get("username");

    Ok(username)
}