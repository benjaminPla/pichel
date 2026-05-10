use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum UnitOfMeasure {
    Kilogram,
    Unit,
}

impl UnitOfMeasure {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Kilogram => "kg",
            Self::Unit     => "unit",
        }
    }
}

impl FromStr for UnitOfMeasure {
    type Err = UnitOfMeasureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kg"   => Ok(Self::Kilogram),
            "unit" => Ok(Self::Unit),
            other  => Err(UnitOfMeasureError::Invalid(other.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UnitOfMeasureError {
    #[error("unidad de medida inválida: {0}")]
    Invalid(String),
}
