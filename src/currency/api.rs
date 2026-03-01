use reqwest::blocking::get;
use serde_json::{Value, from_str};

use super::rates::Rate;

pub fn get_exchange_rates(currency: &str) -> Vec<Rate> {
    let url: String = format!("https://open.exchangerate-api.com/v6/latest/{}", currency);

    let resp: String = get(url)
        .expect("Server response error:")
        .text()
        .expect("Response body error:");
    let body: Value = from_str(&resp).expect("Problem wih API body formatting:");
    let rates = body.get("rates").unwrap().as_object().unwrap().iter();

    let mut output: Vec<Rate> = Vec::new();

    for rate in rates {
        output.push(Rate::new(
            currency.to_string(),
            rate.0.to_string(),
            rate.1.as_f64().unwrap(),
        ));
    }

    output
}

// pub fn split_response(api_repsonse: &String) {
//     let re = Regex::new(r#"([A-Z]{3})\\\":(\d+.\d+)"#).unwrap();
//     let captures = re.captures(&api_repsonse)
//         .expect("No Captures")
//         .iter()
//     for capture in captures {
//         println!("{:?}", capture)
//     }
// }
