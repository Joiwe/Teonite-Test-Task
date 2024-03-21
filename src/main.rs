use rust_decimal::Decimal;
use rusty_money::{Money, iso, ExchangeRate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;


#[derive(Debug, Serialize, Deserialize)]
struct Rates<'a>{
    base: &'a str,
    rates: HashMap<&'a str, Decimal>,
}

fn main() {
   /* Collect input args */
    let args: Vec<String> = env::args().collect();
    let input_currency = iso::find(&args[1])
        .expect("Input currency");
    let output_currency = iso::find(&args[3])
        .expect("Output Currency");
    let input_amount = Money::from_str(&args[2], input_currency).unwrap();



    /* Api Request */
    
    let request_url = format!("https://api.exchangeratesapi.net/v1/exchange-rates/latest?access_key={apiKey}&base={base}&symbols={output}",
        apiKey = "ZJmEMh33RoLEexIi",
        base = input_currency.to_string(),
        output = output_currency.to_string());
    let body = reqwest::blocking::get(&request_url)
        .expect("Api Response")
        .text()
        .unwrap();


/* Deserialize JSON response */
let rates_struct: Rates = serde_json::from_str(&body).unwrap();

/* Exctract exchange rate */
let decimal_rate = rates_struct.rates.get(output_currency.iso_alpha_code)
    .expect("Rate in Api response");
let rate = ExchangeRate::new(input_currency, output_currency, *decimal_rate)
    .unwrap();

/* Convert the currency */
let output = rate.convert(input_amount.clone()).unwrap();

print!("{} is {} after conversion", input_amount, output);

}
