use crate::errors::InvalidExchangeRateError;
use regex;
use reqwest::blocking;
use serde_json;
use std::{
    collections::HashMap,
    error::Error,
    fs::OpenOptions,
    io::{Read, Write},
};

pub const CURRENCY_API_URL: &str = "https://open.exchangerate-api.com/v6/latest/";
pub const API_RATES_CACHE: &str = "rates_cache.txt";
pub const USER_RATES: &str = "user_rates.txt";

fn find_currency_code(code: &str) -> Option<String> {
    let re = regex::Regex::new(r"[A-Z]{3}").unwrap();
    let code_ = code.to_uppercase();
    let output = re.find(&code_)?.as_str();
    Some(output.to_string())
}

pub fn get_exchange_rates(currency: &str) -> Option<(HashMap<String, f64>, &str)> {
    // let rates = rates_from_file(API_RATES_CACHE)?;
    match rates_from_api(currency) {
        Ok(rates) => {
            let _ = rates_to_file(API_RATES_CACHE, &rates);
            Some((rates, "from API"))
        }
        Err(e) => {
            eprintln!("Error retrieving exchange rates from API: {e}. Using cached values.");
            let rates = rates_from_file(API_RATES_CACHE).ok()?;
            Some((rates, "from backup"))
        }
    }
}

pub fn rates_to_file(
    file: &str,
    rates: &HashMap<String, f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let rates_json = serde_json::to_string_pretty(&rates)?;
    // println!("{rates_json}");
    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file)?;
    Ok(write!(&mut output_file, "{rates_json}")?)
}

pub fn rates_from_file(file: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let mut input_file = OpenOptions::new().read(true).open(file)?;

    let mut rate_string = String::new();
    input_file.read_to_string(&mut rate_string)?;
    // println!("{:#?}", rate_string);
    let rates = serde_json::from_str::<HashMap<String, f64>>(&rate_string)?;
    Ok(rates)
}

pub fn rates_from_api(currency: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let url = format!("{}{}", CURRENCY_API_URL, &currency);
    let resp = blocking::get(url)?.text()?;
    let body: serde_json::Value = serde_json::from_str(&resp)?;

    let rates = body
        .get("rates")
        .ok_or("Exchage rates not found.")?
        .as_object()
        .ok_or("Could not convert to object.")?;

    let output = rates
        .into_iter()
        .fold(HashMap::new(), |mut acc, (code, rate)| {
            if let Some(exchange_rate) = rate.to_string().parse::<f64>().ok() {
                acc.insert(code.to_string(), exchange_rate);
                acc
            } else {
                acc
            }
        });
    Ok(output)
}

#[allow(unused)]
fn save_user_rate(
    base_code: &str,
    to_code: &str,
    rate: f64,
) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let base_code_ =
        find_currency_code(base_code).ok_or("No currency code found in base_code string.")?;
    let to_code_ =
        find_currency_code(to_code).ok_or("No currency code found in to_code string.")?;
    match rate {
        rate if rate.is_infinite() => Err(Box::new(InvalidExchangeRateError::InfiniteRate)),
        rate if rate.is_sign_negative() => Err(Box::new(InvalidExchangeRateError::NegativeRate)),
        _ => {
            let mut output = HashMap::new();
            output.insert(format!("{base_code_}_{to_code_}"), rate);
            Ok(output)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
