use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Phone(String);

impl Phone {
    const MAX_LEN: usize = 16; // E.164: '+' + 15 digits

    pub fn new(value: impl Into<String>) -> Result<Self, PhoneError> {
        let s: String = value.into();

        if s.trim().is_empty() { return Err(PhoneError::Blank) }
        if s.len() > Self::MAX_LEN { return Err(PhoneError::TooLong(Self::MAX_LEN)) }

        let re = Regex::new(r"^\+?[0-9]{7,15}$").unwrap();
        if !re.is_match(&s) { return Err(PhoneError::Invalid) }

        Ok(Self(s))
    }

    pub fn value(&self) -> &str { &self.0 }
}

impl std::fmt::Display for Phone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for Phone {
    fn as_ref(&self) -> &str { &self.0 }
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum PhoneError {
    #[error("phone cannot be blank")]
    Blank,
    #[error("phone is invalid (only digits and an optional leading +)")]
    Invalid,
    #[error("phone is too long (maximum {0} characters)")]
    TooLong(usize),
}
