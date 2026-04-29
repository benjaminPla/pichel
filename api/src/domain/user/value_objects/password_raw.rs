pub struct PasswordRaw(String); 

impl PasswordRaw {
    const MAX_LEN: usize = 30;
    const MIN_LEN: usize = 10;

    pub fn value(&self) -> &str { &self.0 }

    pub fn new(value: impl Into<String>) -> Result<Self, PasswordRawError> {
        let s: String = value.into();
        if s.trim().is_empty()     { return Err(PasswordRawError::Blank) }
        if s.len() > Self::MAX_LEN { return Err(PasswordRawError::TooLong(Self::MAX_LEN)) }
        if s.len() < Self::MIN_LEN { return Err(PasswordRawError::TooShort(Self::MIN_LEN)) }
        Ok(Self(s))
    }
}

impl std::fmt::Debug for PasswordRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PasswordRaw([redacted])")
    }
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum PasswordRawError {
    #[error("password cannot be blank")]
    Blank,
    #[error("invalid password")]
    Invalid,
    #[error("password is too long (maximum {0} characters")]
    TooLong(usize),
    #[error("password is too short (minimum {0} characters")]
    TooShort(usize),
}

