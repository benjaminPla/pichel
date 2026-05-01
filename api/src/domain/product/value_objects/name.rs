#[derive(Debug, Clone)]
pub struct Name(String);

impl Name {
    const MAX_CHARS: usize = 100;
    const MIN_CHARS: usize = 3;

    pub fn new(value: impl Into<String>) -> Result<Self, NameError> {
        let s = value.into();
        if s.trim().is_empty()       { return Err(NameError::Empty) }
        if s.len() > Self::MAX_CHARS { return Err(NameError::TooLong(Self::MAX_CHARS)) }
        if s.len() < Self::MIN_CHARS { return Err(NameError::TooShort(Self::MIN_CHARS)) }
        Ok(Self(s))
    }

    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, thiserror::Error)]
pub enum NameError {
    #[error("name cannot be empty")]
    Empty,
    #[error("name cannot be longer than {0} characters")]
    TooLong(usize),
    #[error("name cannot be shorter than {0} characters")]
    TooShort(usize),
}
