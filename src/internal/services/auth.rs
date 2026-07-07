use argon2::{self, PasswordVerifier, password_hash};
use sqlx::{PgPool, Row};

use crate::internal::database;
use crate::internal::services::jwt::AuthEngine;

pub async fn check_password(pool: &PgPool, id: &i32, raw_password: &str) -> Result<(), String> {
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

pub async fn login(
    pool: &PgPool,
    autheng: &AuthEngine,
    username:String,
    raw_password:String
) -> Result<(String, String), String> {

    // Check if the user is in the database \\
    let user_id = database::users::get_id(pool, &username).await?;

    // Check if the password is valid \\
    if check_password(pool, &user_id, &raw_password).await.is_err() {
        return Err("Invalid credentials".to_string());
    }

    // Make the tokens \\
    let access_token = autheng.generate_access_token(&user_id)?;
    let refresh_token = autheng.generate_refresh_token();
    
    AuthEngine::save_refresh_token(pool, user_id, &refresh_token).await?;

    Ok((access_token, refresh_token))
}

pub async fn refresh_session(pool: &PgPool, refresh_token: &String, autheng: &AuthEngine, id: &i32) -> Result<String, String> {
    autheng.verify_refresh_token(pool, id, refresh_token).await?;

    let new_access_token = autheng.generate_access_token(id)?;

    Ok(new_access_token)
}