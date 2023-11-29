use std::str::FromStr;

mod card;
use card::{format_balance, Card};

mod card_type;

mod constants;
use constants::*;

use reqwest::{
    blocking::{Client, Request},
    header::HeaderValue,
    Error, Method, Url,
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    id: Option<String>,
    name: Option<String>,
    web_hook_url: Option<String>,

    #[serde(rename = "accounts")]
    cards: Vec<Card>,
}

/// private methods
impl Account {
    pub(crate) fn format(&mut self) {
        for card in self.cards.iter_mut() {
            format_balance(card)
        }
    }
}

/// getters block
impl Account {
    pub fn id(&self) -> &str {
        match &self.id {
            Some(id) => id.as_str(),
            None => "id is none",
        }
    }

    pub fn name(&self) -> &str {
        match &self.name {
            Some(name) => name.as_str(),
            None => "name is none",
        }
    }

    pub fn web_hook_url(&self) -> &str {
        match &self.web_hook_url {
            Some(web_hook_url) => web_hook_url.as_str(),
            None => "web hook is none",
        }
    }

    pub fn main_card(&self) -> Option<&Card> {
        for card in self.cards.iter() {
            if card.is_main() {
                return Some(card);
            }
        }

        None
    }

    pub fn white_card(&self) -> Option<&Card> {
        for card in self.cards.iter() {
            if card.is_white() {
                return Some(card);
            }
        }

        None
    }
}
