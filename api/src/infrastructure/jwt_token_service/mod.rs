use crate::domain::{
    auth::{
        ports::{TokenService, TokenServiceError},
        Claims,
    },
    user::value_objects::id::UserId,
};

pub struct JwtTokenService {
    secret: String,
}

impl JwtTokenService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl TokenService for JwtTokenService {
    fn issue(&self, user_id: &UserId) -> Result<String, TokenServiceError> {
        todo!()
    }

    fn validate(&self, token: &str) -> Result<Claims, TokenServiceError> {
        todo!()
    }
}
