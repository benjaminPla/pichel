use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
pub enum Symbol {
    ContainsNuts,
    GlutenFree,
    HighProtein,
    LactoseFree,
    NoAddedSalt,
    Organic,
    SugarFree,
    Vegan,
    Vegetarian,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Symbol::ContainsNuts => "contains_nuts",
            Symbol::GlutenFree   => "gluten_free",
            Symbol::HighProtein  => "high_protein",
            Symbol::LactoseFree  => "lactose_free",
            Symbol::NoAddedSalt  => "no_added_salt",
            Symbol::Organic      => "organic",
            Symbol::SugarFree    => "sugar_free",
            Symbol::Vegan        => "vegan",
            Symbol::Vegetarian   => "vegetarian",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Symbol {
    type Err = SymbolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vegan"         => Ok(Self::Vegan),
            "vegetarian"    => Ok(Self::Vegetarian),
            "gluten_free"   => Ok(Self::GlutenFree),
            "lactose_free"  => Ok(Self::LactoseFree),
            "organic"       => Ok(Self::Organic),
            "sugar_free"    => Ok(Self::SugarFree),
            "contains_nuts" => Ok(Self::ContainsNuts),
            "high_protein"  => Ok(Self::HighProtein),
            "no_added_salt" => Ok(Self::NoAddedSalt),
            other           => Err(SymbolError::Invalid(other.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SymbolError {
    #[error("invalid symbol: {0}")]
    Invalid(String),
}
