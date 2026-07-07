use std::{time::{SystemTime, UNIX_EPOCH}};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use rand::{TryRng, rngs::{SysRng}};

#[derive(Serialize, Debug, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize
}

#[derive(Clone)]
pub struct AuthEngine {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey
}

impl AuthEngine {
    pub fn new() -> Result<Self, String> {
        dotenvy::dotenv().unwrap();
        let secret = dotenvy::var("JWT_Secret").unwrap().to_string();
        
        Ok(Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()) })
    }

    pub fn generate_access_token(&self, id: &i32) -> Result<String, String> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs() + (15 * 60);

        let claims = Claims {
            sub: *id,
            exp: expiration as usize
        };

        jsonwebtoken::encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| e.to_string())
    }

    pub fn generate_refresh_token(&self) -> String {
        let mut bytes = [0u8; 32];
        SysRng::try_fill_bytes(&mut SysRng::default(), &mut bytes).unwrap();
        hex::encode(bytes)
    }

    pub fn check_access_token(&self, token: &str) -> Result<i32, String> {
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

        let token_data = jsonwebtoken::decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expired".to_string(),
                _ => "Token invalid for another reason".to_string(),
            })?;
        
        Ok(token_data.claims.sub)
    }

    pub fn get_id_from_expired_token(&self, token: &str) -> Result<i32, String>{
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.validate_exp = false;

        let token_data = jsonwebtoken::decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|_| "Token invalid for another reason")?;

        Ok(token_data.claims.sub)
    }
}