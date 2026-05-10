use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub cookie_secure: bool,
    pub database_url:  String,
    pub jwt_secret:    String,
    pub port:          u16,
}
