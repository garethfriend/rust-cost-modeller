mod currency;

fn main() {
    let rates = currency::get_exchange_rates("GBP");
    let conv = rates[10].conversion_rate(&rates[4]).unwrap();
    println!(
        "{:?} to {:?}",
        rates[10].currency_code, rates[4].currency_code
    );
    println!("{conv:?}");
}
