use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthenticateRequestBody {
    pub email:    String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthenticateResponse {
    pub token: String,
}
