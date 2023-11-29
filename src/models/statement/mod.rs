use serde::Deserialize;

mod constants;
use constants::*;

use super::date::Date;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Statement {
    pub id: String,
    pub time: Date,
    pub description: String,
    pub mcc: i32,
    pub hold: bool,
    pub amount: f64,
    pub operation_amount: f64,
    pub currency_code: i32,
    pub commission_rate: i32,
    pub cashback_amount: f64,
    pub balance: f64,
    pub comment: Option<String>,
    pub receipt_id: Option<String>,
    pub invoice_id: Option<String>,
    pub counter_edrpou: Option<String>,
    pub counter_iban: Option<String>,
    pub counter_name: Option<String>,
}

pub(crate) trait FormattedStatement {
    fn format(&mut self);
}

impl FormattedStatement for Vec<Statement> {
    fn format(&mut self) {
        self.iter_mut().for_each(|st| {
            st.amount /= 100f64;
            st.operation_amount /= 100f64;
            st.balance /= 100f64
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
