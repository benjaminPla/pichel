// ADAPTER — implements the TokenService PORT using the jsonwebtoken crate.
// The domain has zero knowledge of this file exists.

use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::auth::{service::TokenService, AuthDomainError, UserId};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String, // UserId as string
    exp: usize,  // expiry (unix timestamp)
    iat: usize,  // issued at
}

pub struct JwtTokenService {
    secret: String,
    expiry_secs: i64,
}

impl JwtTokenService {
    pub fn new(secret: impl Into<String>) -> Self {
        Self { secret: secret.into(), expiry_secs: 60 * 60 * 24 } // 24h
    }
}

impl TokenService for JwtTokenService {
    fn issue(&self, user_id: &UserId) -> Result<String, AuthDomainError> {
        let now = Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: user_id.0.to_string(),
            exp: now + self.expiry_secs as usize,
            iat: now,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AuthDomainError::Token(e.to_string()))
    }

    fn validate(&self, token: &str) -> Result<UserId, AuthDomainError> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AuthDomainError::Token(e.to_string()))?;

        let uuid = Uuid::parse_str(&data.claims.sub)
            .map_err(|e| AuthDomainError::Token(e.to_string()))?;

        Ok(UserId(uuid))
    }
}
