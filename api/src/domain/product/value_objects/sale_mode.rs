use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum SaleMode {
    Bulk,
    Unit,
}

impl SaleMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bulk => "bulk",
            Self::Unit => "unit",
        }
    }
}

impl FromStr for SaleMode {
    type Err = SaleModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bulk" => Ok(Self::Bulk),
            "unit" => Ok(Self::Unit),
            other  => Err(SaleModeError::Invalid(other.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SaleModeError {
    #[error("invalid sale mode: {0}")]
    Invalid(String),
}
