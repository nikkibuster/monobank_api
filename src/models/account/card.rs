use super::card_type::CardType;

use serde::Deserialize;

#[derive(Deserialize, serde::Serialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: String,
    pub send_id: String, // for send.monobank.ua
    pub balance: f32,
    pub credit_limit: i32,

    #[serde(rename = "type")]
    pub card_type: CardType,
    pub currency_code: i32,
    pub cashback_type: String,
    pub masked_pan: Vec<String>,
    pub iban: Option<String>,
}

impl Card {
    pub fn is_main(&self) -> bool {
        if self.card_type.is_black() && self.currency_code == 980 {
            return true;
        };

        false
    }

    pub fn is_white(&self) -> bool {
        if self.card_type.is_white() {
            return true;
        };

        false
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub(crate) fn format_balance(&mut self) {
        self.balance /= 100f32
    }
}
