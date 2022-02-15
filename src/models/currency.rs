use serde::{Deserialize};
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
    /// get_list static method which return vector of currencies from mono API.
    /// # Examples
    /// ```
    ///     use monobank_api::models::currency::Currency;
    /// 
    ///     let list = Currency::get_list();
    /// ```
    pub fn get_list() -> Vec<Currency> {
        let url = "https://api.monobank.ua/bank/currency";
        
        get(url).unwrap().json::<Vec<Currency>>().unwrap()
    }
}