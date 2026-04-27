use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Dimension {
    Count,
    Volume,
    Weight,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitOfMeasure {
    Gram,
    Kilogram,
    Liter,
    Milliliter,
    Unit,
}

impl UnitOfMeasure {
    pub fn dimension(&self) -> Dimension {
        match self {
            Self::Gram | Self::Kilogram    => Dimension::Weight,
            Self::Milliliter | Self::Liter => Dimension::Volume,
            Self::Unit                     => Dimension::Count,
        }
    }

    pub fn to_base_factor(&self) -> f64 {
        match self {
            Self::Gram | Self::Milliliter | Self::Unit => 1.0,
            Self::Kilogram | Self::Liter               => 1000.0,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Gram       => "g",
            Self::Kilogram   => "kg",
            Self::Liter      => "l",
            Self::Milliliter => "ml",
            Self::Unit       => "unit",
        }
    }
}

impl FromStr for UnitOfMeasure {
    type Err = UnitOfMeasureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "g"    => Ok(Self::Gram),
            "kg"   => Ok(Self::Kilogram),
            "l"    => Ok(Self::Liter),
            "ml"   => Ok(Self::Milliliter),
            "unit" => Ok(Self::Unit),
            other  => Err(UnitOfMeasureError::Invalid(other.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UnitOfMeasureError {
    #[error("invalid unit of measure: {0}")]
    Invalid(String),
}
