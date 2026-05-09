use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Rate {
    base_code: String,
    pub currency_code: String,
    pub exchange_rate: f64,
}

impl Rate {
    pub fn new(base_code: String, currency_code: String, exchange_rate: f64) -> Self {
        Self {
            base_code,
            currency_code,
            exchange_rate,
        }
    }

    pub fn conversion_rate(&self, other: &Self) -> Result<f64, &str> {
        if self.base_code != other.base_code {
            return Err(
                "Other does not have the same base code: {other.base_code} - {self.base_code} required.",
            );
        };

        Ok(self.exchange_rate / other.exchange_rate)
    }
}
