use reqwest::{
    blocking::{Client, Request},
    header::HeaderValue,
    Error, Method, Url,
};
use serde::Deserialize;

mod constants;
use crate::models::date::Date;
use constants::*;

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

pub(crate) trait FormattedStatement {
    fn format(&mut self);
}

impl FormattedStatement for Vec<Statement> {
    fn format(&mut self) {
        self.iter_mut().for_each(|st| {
            st.amount /= 100f64;
            st.operationAmount /= 100f64;
        });
    }
}

pub(crate) fn prepare_url(account: &'static str, from: i64, to: i64) -> String {
    let base = crate::models::methods::Method::Statements.path();
    let from_str = from.to_string();
    let mut to_str = to.to_string();

    if to_str == "0" {
        to_str.clear()
    }

    let splits: Vec<&str> = base
        .split("/")
        .map(|part| match part {
            KEY_ACCOUNT => account,
            KEY_FROM => from_str.as_str(),
            KEY_TO => to_str.as_str(),
            _ => part,
        })
        .collect();

    splits.join("/")
}
