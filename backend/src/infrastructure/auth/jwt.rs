use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

pub struct JwtService {
    secret: String,
    expiration_hours: u64,
}

impl JwtService {
    pub fn new(secret: String, expiration_hours: u64) -> Self {
        Self {
            secret,
            expiration_hours,
        }
    }

    pub fn generate_token(&self, user_id: &str, email: &str, role: &str) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.expiration_hours as i64);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(AppError::Jwt)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}
