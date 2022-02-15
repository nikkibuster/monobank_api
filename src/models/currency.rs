use std::collections::HashMap;

use serde::{Deserialize};
use serde_json::{from_str, Value};
use reqwest::blocking::get;

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    currency_code_a: i32,
    currency_code_b: i32,
    date: i64,
    rate_buy: Option<f32>,
    rate_sell: Option<f32>,
    rate_cross: Option<f32>
}

impl Currency {
    pub fn get_list() -> Vec<Currency> {
        let url = "https://api.monobank.ua/bank/currency";
        
        get(url).unwrap().json::<Vec<Currency>>().unwrap()
    }
}