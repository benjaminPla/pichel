mod errors;

use crate::domain::{
    auth::{
        ports::{TokenService, TokenServiceError},
        Claims,
    },
    user::value_objects::id::UserId,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct JwtPayload {
    sub: String,
    exp: i64,
}

pub struct JwtTokenService {
    jwt_secret: String,
}

impl JwtTokenService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

impl TokenService for JwtTokenService {
    fn issue(&self, user_id: &UserId) -> Result<String, TokenServiceError> {
        let claims  = Claims::new(user_id.clone());
        let payload = JwtPayload {
            exp: claims.get_exp(),
            sub: claims.get_sub().value().to_string(),
        };
        let token = encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;
        Ok(token)
    }

    fn validate(&self, token: &str) -> Result<Claims, TokenServiceError> {
        let decode = decode::<JwtPayload>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;
        let user_id = Uuid::parse_str(&decode.claims.sub)?;
        let claims  = Claims::reconstitute(
            decode.claims.exp as i64,
            UserId::reconstitute(user_id),
        );
        Ok(claims)
    }
}
