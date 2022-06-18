use std::collections::HashMap;
use lazy_static::lazy_static;

pub const TOKEN_KEY: &str = "X-Token";
pub const TOKEN: &str = "MONOBANK_TOKEN";

pub mod models;
pub mod client;

lazy_static! {
    static ref URLS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();

        // "https://api.monobank.ua"
        map.insert("currency", "https://api.monobank.ua/bank/currency");
        map.insert("account", "https://api.monobank.ua/personal/client-info");
        map.insert("statements", "https://api.monobank.ua/personal/statement/{account}/{from}/{to}");
        map
    };
}

/// Build Personal client with token from environment variable
pub fn from_env() -> client::PersonalClient {
    let token = std::env::var(TOKEN).unwrap();
    client::PersonalClientBuilder::new(token).build()
}