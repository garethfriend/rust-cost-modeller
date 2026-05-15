use std::error::Error;

use crate::errors::{CostError, InvalidCurrencyCodeError, InvalidExchangeRateError, QuantityError};
use regex::Regex;
use uom::si::mass;

#[derive(Debug)]
pub struct Ingredient {
    name: String,
    pricing_currency: String,
    pricing_unit: mass::Units,
    cost_per_unit: f64,
    quantity: f64,
}

#[allow(unreachable_code)]
impl Ingredient {
    pub fn new(
        name: String,
        quantity: f64,
        pricing_currency: String,
        pricing_unit: mass::Units,
        cost_per_unit: f64,
        exchange_rate: f64,
    ) -> Result<Self, Box<dyn Error>> {
        match quantity {
            q if q.is_sign_negative() => Err(Box::new(QuantityError::NegativeQuantity)),
            q if q.is_infinite() => Err(Box::new(QuantityError::InfiniteQuantity)),
            _ => !unreachable!("should never happen"),
        }?;

        match cost_per_unit {
            c if c.is_sign_negative() => Err(Box::new(CostError::NegativeCost)),
            c if c.is_infinite() => Err(Box::new(CostError::InfiniteCost)),
            _ => !unreachable!("should never happen"),
        }?;

        match exchange_rate {
            r if r.is_sign_negative() => Err(Box::new(InvalidExchangeRateError::NegativeRate)),
            r if r.is_infinite() => Err(Box::new(InvalidExchangeRateError::InfiniteRate)),
            _ => !unreachable!("should never happen"),
        }?;

        let re = Regex::new(r"^[A-Z]{3}$").unwrap();
        if !re.is_match(&pricing_currency) {
            return Err(Box::new(InvalidCurrencyCodeError));
        }
        
        Ok(Self {
            name,
            pricing_currency,
            pricing_unit,
            cost_per_unit,
            quantity,
        })
    }
}

