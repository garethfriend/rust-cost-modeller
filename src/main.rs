mod currency;
mod errors;
mod ingredients;
use std::collections::HashMap;

fn main() {
    let (rates, location) = currency::get_exchange_rates("GBP").unwrap();
    println!("Got {} exchange rates {}.", rates.len(), location);
    let mut user_rates = HashMap::new();
    user_rates.insert("GBP_TOP".to_string(), 1.2344);
    let _ = currency::rates_to_file(currency::USER_RATES, &user_rates);
}
