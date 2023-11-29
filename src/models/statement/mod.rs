use serde::Deserialize;

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
