use argon2::{self, PasswordVerifier, password_hash};
use sqlx::{PgPool, Row};

pub async fn check_password(pool: &PgPool, id: i32, raw_password: &str) -> Result<(), String> {
    let hash: String = sqlx::query("SELECT password_hash FROM Users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
        .get("password_hash");

    let parsed_hash = password_hash::PasswordHash::new(&hash.as_str())
        .map_err(|e| e.to_string())?;

    match argon2::Argon2::default().verify_password(raw_password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid password".to_string())
    }
}