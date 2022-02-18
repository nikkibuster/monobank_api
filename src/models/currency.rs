use std::{io::{Result, Error as IoError, ErrorKind}};
use serde::{Deserialize};
use reqwest::{
    blocking::get,
};

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
    pub fn get_list() -> Result<Vec<Currency>> {
        let url = *crate::URLS.get("currency").unwrap();

        let result = match get(url) {
            Ok(resp) => match resp.json::<Vec<Currency>>() {
                Ok(currencies) => currencies,
                Err(err) => return Err(IoError::new(ErrorKind::Other, err.to_string())),
            },
            Err(err) => return Err(IoError::new(ErrorKind::Other, err.to_string())),
        };
        Ok(result)
    }

}