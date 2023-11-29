mod card;
use card::Card;

mod card_type;
mod jars;

use serde::Deserialize;

use self::jars::Jar;

#[derive(Deserialize, serde::Serialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(rename = "clientId")]
    id: Option<String>,
    name: Option<String>,
    permissions: Option<String>,
    web_hook_url: Option<String>,

    #[serde(rename = "accounts")]
    cards: Vec<Card>,

    jars: Vec<Jar>,
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
            None => "web hook is not set",
        }
    }

    pub fn main_card(&self) -> Option<&Card> {
        self.cards.iter().find(|c| c.is_main())
    }

    pub fn white_card(&self) -> Option<&Card> {
        self.cards.iter().find(|c| c.is_white())
    }

    pub fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    pub fn jars(&self) -> Vec<Jar> {
        self.jars.clone()
    }
}

impl crate::FormattedAmount for Account {
    fn format(&mut self) {
        self.cards.iter_mut().for_each(|card| card.format_balance())
    }
}
