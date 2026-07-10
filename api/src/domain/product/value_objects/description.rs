#[derive(Clone, Debug)]
pub struct Description(String);

impl Description {
    const MAX_CHARS: usize = 255;
    const MIN_CHARS: usize = 3;

    pub fn new(value: impl Into<String>) -> Result<Self, DescriptionError> {
        let s = value.into();
        if s.trim().is_empty()       { return Err(DescriptionError::Empty) }
        if s.len() > Self::MAX_CHARS { return Err(DescriptionError::TooLong(Self::MAX_CHARS)) }
        if s.len() < Self::MIN_CHARS { return Err(DescriptionError::TooShort(Self::MIN_CHARS)) }
        Ok(Self(s))
    }

    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, thiserror::Error)]
pub enum DescriptionError {
    #[error("description cannot be empty")]
    Empty,
    #[error("description cannot be longer than {0} characters")]
    TooLong(usize),
    #[error("description cannot be shorter than {0} characters")]
    TooShort(usize),
}
