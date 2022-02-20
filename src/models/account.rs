use std::str::FromStr;

mod card;
use card::{Card, format_balance};

mod card_type;

mod constants;
use constants::*;

use serde::Deserialize;
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    id: Option<String>,
    name: Option<String>,
    web_hook_url: Option<String>,

    #[serde(rename = "accounts")]
    cards: Vec<Card>
}

/// public methods
impl Account {
    /// get_info method return personal account info such name and cards info
    /// # Errors
    /// this method will failed if token will be incorrect
    pub fn get_info(token: &str) -> Result<Account, Error> {
        let url = crate::URLS.get(MAP_KEY).unwrap();

        let mut req = Request::new(Method::GET, Url::from_str(*url).unwrap());
        req.headers_mut().append(TOKEN_KEY, HeaderValue::from_str(token).unwrap());

        let client = Client::default();

        let mut account = client.execute(req)?.json::<Account>()?;
        account.format();

        Ok(account)
    }
}

/// private methods
impl Account {
    fn format(&mut self) {
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
            None => "id is none"
        }
    }

    pub fn name(&self) -> &str {
        match &self.name {
            Some(name) => name.as_str(),
            None => "name is none"
        }
    }

    pub fn web_hook_url(&self) -> &str {
        match &self.web_hook_url {
            Some(web_hook_url) => web_hook_url.as_str(),
            None => "web hook is none"
        }
    }

    pub fn main_card(&self) -> Option<&Card> {
        for card in self.cards.iter() {
            if card.is_main() {
                return Some(card)
            }
        }

        None
    }
}
