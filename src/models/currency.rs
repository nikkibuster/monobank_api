use serde::Deserialize;

use super::date::Date;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub currency_code_a: i32,
    pub currency_code_b: i32,
    pub date: Date,
    pub rate_buy: Option<f32>,
    pub rate_sell: Option<f32>,
    pub rate_cross: Option<f32>,
}
