use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret:   String,
    pub port:         u16,
}
