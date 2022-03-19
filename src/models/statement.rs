use serde::{Deserialize};
use reqwest::{
    header::HeaderValue,
    blocking::{
        Request,
        Client,
    },
    Method,
    Url,
    Error
};

mod constants;
use constants::*;
use crate::models::date::Date;

#[derive(Deserialize, Debug)]
#[serde(rename = "camelCase")]
pub struct Statement {
    id: String,
    time: Date,
    description: Option<String>,
    mcc: i32,
    hold: bool,
    amount: f64,
    operationAmount: f64,
    currencyCode: i32,
    commissionRate: i32,
    balance: f64,
    comment: Option<String>,
    receipt_id: Option<String>,
    counter_edrpou: Option<String>,
    counter_iban: Option<String>,
}

impl Statement {
    pub fn get_list(token: &str, account: &str, from: i64, to: i64) -> Result<Vec<Statement>, Error> {
        let base = *crate::URLS.get("statements").unwrap();
        let from_str = from.to_string();
        let to_str = to.to_string();

        let splits: Vec<&str> = base
            .split("/")
            .map(|part| {
                match part {
                    KEY_ACCOUNT => account,
                    KEY_FROM => from_str.as_str(),
                    KEY_TO => to_str.as_str(),
                    _ => part
                }
            }).collect();

        let url = splits.join("/");
        
        let mut req = Request::new(Method::GET, Url::parse(url.as_str()).unwrap());
        req.headers_mut().append(crate::TOKEN_KEY, HeaderValue::from_str(token).unwrap());

        let client = Client::default();
    
        client.execute(req)?.json::<Vec<Statement>>()
    }

    pub fn text_resp(token: &str, account: &str, from: i64, to: i64) -> Result<String, Error> {
        let base = *crate::URLS.get("statements").unwrap();
        let from_str = from.to_string();
        let to_str = to.to_string();

        let splits: Vec<&str> = base
            .split("/")
            .map(|part| {
                match part {
                    KEY_ACCOUNT => account,
                    KEY_FROM => from_str.as_str(),
                    KEY_TO => to_str.as_str(),
                    _ => part
                }
            }).collect();

        let url = splits.join("/");
        
        let mut req = Request::new(Method::GET, Url::parse(url.as_str()).unwrap());
        req.headers_mut().append(crate::TOKEN_KEY, HeaderValue::from_str(token).unwrap());

        let client = Client::default();
    
        client.execute(req)?.text()
    }

}