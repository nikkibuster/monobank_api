mod models;
use std::collections::HashMap;

use models::{
    currency::Currency
};

use reqwest::blocking::get;
use serde_json::{from_str, Value};

#[test]
fn currency_json() {
    let data = r#"
    [
        {
          "currencyCodeA": 840,
          "currencyCodeB": 980,
          "date": 1552392228,
          "rateSell": 27,
          "rateBuy": 27.2,
          "rateCross": 27.1
        }
      ]
    "#;

    let curs: Vec<Currency> = from_str(data).unwrap();

    println!("{:?}", curs)
}

#[test]
fn api_call() {
    let url = "https://api.monobank.ua/bank/currency";
    let resp = get(url).unwrap();
    let data = resp.text().unwrap();

    let res: Vec<HashMap<&str, Value>>  = from_str(data.as_str()).unwrap();
   
    let map = res.get(0).unwrap();

    for (k, v) in map {
        println!("key: {}, value: {}", k, v)
    }

    let rate =  map.get("rateBuy").unwrap();

    println!("{}", rate)
}


#[test]
fn currency_request() {
    let url = "https://api.monobank.ua/bank/currency";
    let resp = get(url).unwrap();

    let curs: Vec<Currency> = resp.json::<Vec<Currency>>().unwrap();

    print!("{:?}", curs[0])
}