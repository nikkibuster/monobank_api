use std::collections::HashMap;

use lazy_static::lazy_static;

pub(crate) const TOKEN_KEY: &str = "X-Token";

lazy_static! {
    pub(crate) static ref URLS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();

        // "https://api.monobank.ua"
        map.insert("currency", "https://api.monobank.ua/bank/currency");
        map.insert("account", "https://api.monobank.ua/personal/client-info");
        map.insert("statements", "https://api.monobank.ua/personal/statement/{account}/{from}/{to}");
        map
    };
}

#[derive(Debug, Clone)]
pub enum Method {
    Account,
    Currencies,
    Statements,
}

impl Method {
    pub fn path(&self) -> &'static str {
        match self {
            Method::Account => "https://api.monobank.ua/personal/client-info",
            Method::Currencies => "https://api.monobank.ua/bank/currency",
            Method::Statements => {
                "https://api.monobank.ua/personal/statement/{account}/{from}/{to}"
            }
        }
    }
}
