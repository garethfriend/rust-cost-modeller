use reqwest::blocking::get;
use serde_json;

#[derive(Debug)]
pub struct Rate {
   currency_code: String,
   exchange_rate: f64,
}

pub fn get_exchange_rates(currency: &str) {
    let url: String = format!(
        "https://open.exchangerate-api.com/v6/latest/{}",
        currency
    );
    
    let resp: String = get(url).expect("Server respponse error:").text().expect("Response body error:");
    let body: serde_json::Value = serde_json::from_str(&resp).expect("Problem wih API body formatting:");
    let rates = body.get("rates").unwrap().as_object().unwrap().iter();

    let output: &mut Vec<Rate> = &mut Vec::new();

    for rate in rates {
        output.push(Rate {currency_code: String::from(rate.0), exchange_rate: rate.1.as_f64().unwrap()});
    }

    dbg!(output);
    // println!("{:#?}", body.get("result"))    
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