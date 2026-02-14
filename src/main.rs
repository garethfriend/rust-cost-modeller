mod currency;

fn main() {
    let body = currency::get_exchange_rates("GBP");
    // println!("body = {body:?}");cls
    // let rows = currency::split_response(&body);

}
