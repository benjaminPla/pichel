use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

impl Email {
    const MAX_LEN: usize = 254;

    pub fn new(value: impl Into<String>) -> Result<Self, EmailError> {
        let s: String = value.into();

        if s.trim().is_empty() { return Err(EmailError::Blank) }
        if s.len() > Self::MAX_LEN { return Err(EmailError::TooLong(Self::MAX_LEN)) }

        let re = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap();
        if !re.is_match(&s) { return Err(EmailError::Invalid) }

        Ok(Self(s))
    }

    pub fn value(&self) -> &str { &self.0 }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str { &self.0 }
}

// ── Errors ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("email cannot be blank")]
    Blank,
    #[error("email is invalid")]
    Invalid,
    #[error("email is too long (maximum {0} characters)")]
    TooLong(usize),
}
