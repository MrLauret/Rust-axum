use std::ptr::hash;

use crate::internal::services::jwt::{AuthEngine};
use chrono;
use sqlx::{PgPool};

fn hash_token(token: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

impl AuthEngine {
    pub async fn save_refresh_token(pool: &PgPool, id: i32, token: String) -> Result<(), String> {
        let token_hash = hash_token(&token);
        
        sqlx::query(
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id) 
            DO UPDATE SET 
                token_hash = EXCLUDED.token_hash, 
                expires_at = EXCLUDED.expires_at, 
                created_at = CURRENT_TIMESTAMP
            "#
        ).bind(id)
        .bind(token_hash)
        .bind(chrono::Utc::now() + chrono::Duration::days(7))
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn verify_refresh_token(&self, pool: &PgPool, id: i32, token: String) -> Result<(), String> {
        let current_hash = hash_token(&token);
        let row: (String, chrono::DateTime<Utc>) = sqlx::query_as(
            "SELECT token_hash, expires_at FROM refresh_tokens WHERE user_id = $1"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => "Refresh token not found or session revoked".to_string(),
            _ => e.to_string(),
        })?;

        Ok(())
    }
}