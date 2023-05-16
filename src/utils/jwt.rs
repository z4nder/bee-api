use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::{ApiError, AppError};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: u64,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: u64) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign(id: u64) -> Result<std::string::String, ApiError> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT SECRET ENV NOT SET");
    jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| ApiError::new(AppError::TokenError, None))
}

pub fn verify(token: &str) -> Result<Claims, ApiError> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT SECRET ENV NOT SET");
    let decoded_token = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::new(AppError::TokenError, None))?;

    Ok(decoded_token.claims)
}
