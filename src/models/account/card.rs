use super::card_type::CardType;

use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    id: String,
    balance: f32,
    creditLimit: i32,

    #[serde(rename = "type")]
    card_type: CardType,
    currency_code: i32,
    cashback_type: String,
}

impl Card {
    pub fn balance(&self) -> f32 {
        self.balance
    }

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
}

pub fn format_balance(card: &mut Card) {
    card.balance /= 100f32
}
