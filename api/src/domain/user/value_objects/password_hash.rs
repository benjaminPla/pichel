#[derive(Clone)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(value: impl Into<String>) -> Self {
        let s: String = value.into();
        Self(s)
    }

    pub fn value(&self) -> &str { &self.0 }
}

impl std::fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PasswordHash([redacted])")
    }
}
