use std::{collections::HashMap, fmt, error::Error, env::VarError};

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

#[derive(Debug)]
struct ClientError {
    details: String
}

impl ClientError {
    fn new(msg: &str) -> ClientError {
        ClientError{details: msg.to_string()}
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ClientError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Build Personal client with token from environment variable
pub fn from_env() -> Result<client::PersonalClient, VarError> {
    let token = std::env::var(TOKEN)?;

    Ok(client::PersonalClientBuilder::new(token).build())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_env() -> Result<(), Box<dyn Error>> {
        let client = from_env()?;
        

        Ok(())
    }

}