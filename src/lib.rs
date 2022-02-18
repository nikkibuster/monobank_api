use std::collections::HashMap;
use lazy_static::lazy_static;

pub mod models;

lazy_static! {
    static ref URLS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();

        // "https://api.monobank.ua"
        map.insert("currency", "https://api.monobank.ua/bank/currency");
        map.insert("account", "https://api.monobank.ua/personal/client-info");
        map
    };
}