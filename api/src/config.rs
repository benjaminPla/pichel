use serde::Deserialize;

fn default_uploads_path() -> String { "./uploads".to_string() }

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub cookie_secure:  bool,
    pub database_url:   String,
    pub jwt_secret:     String,
    pub port:           u16,
    #[serde(default = "default_uploads_path")]
    pub uploads_path:   String,
}
