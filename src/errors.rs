use std::{error::Error, fmt};

#[derive(Debug)]
pub enum InvalidExchangeRateError {
    NegativeRate,
    InfiniteRate,
}

impl Error for InvalidExchangeRateError {}

impl fmt::Display for InvalidExchangeRateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::NegativeRate => write!(f, "Exchange rates must be positive."),
            Self::InfiniteRate => write!(f, "Exchange rates must be finite."),
        }
    }
}

#[derive(Debug)]
pub enum QuantityError {
    NegativeQuantity,
    InfiniteQuantity,
    NotAMassUnit,
}

impl Error for QuantityError {}

impl fmt::Display for QuantityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::NegativeQuantity => write!(f, "Quantities must be positive."),
            Self::InfiniteQuantity => write!(f, "Quantities must be finite."),
            Self::NotAMassUnit => write!(f, "Quantity is not a mass unit."),
        }
    }
}

#[derive(Debug)]
pub enum CostError {
    NegativeCost,
    InfiniteCost,
}

impl Error for CostError {}

impl fmt::Display for CostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::NegativeCost => write!(f, "Cost must be positive."),
            Self::InfiniteCost => write!(f, "Cost must be finite."),
        }
    }
}

#[derive(Debug)]
pub struct InvalidCurrencyCodeError;

impl Error for InvalidCurrencyCodeError {}

impl fmt::Display for InvalidCurrencyCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Currency code must be three letters.")
    }
}
