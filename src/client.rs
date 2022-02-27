use super::models::{
    currency::Currency,
    statement::Statement,
    account::Account
};

use reqwest::Error;


macro_rules! personal_client {
    ($client:ident) => {
        {
            impl $client {
                pub fn get_info(&self) -> Result<Account, Error> {
                    Account::get_info(self.token.as_str())
                }

                pub fn get_statements(&self, account: &str, from: i64, to: i64) -> Result<Vec<Statement>, Error> {
                    Statement::get_list(self.token.as_str(), account, from, to)
                }
            }
        }
    };
}

pub struct ClientBuilder {
    token: String,
}

impl ClientBuilder {
    pub fn new() -> Self {
        ClientBuilder{token: String::default()}
    }

    pub fn with_token(&mut self, token: &str) -> &Self {
        self.token.push_str(token);

        self
    }

    pub fn build(&self) -> Client {
        let client = Client{token: self.token.clone()};
        if !self.token.is_empty() {
            personal_client!(Client);
        }

        client
    }
}

pub struct Client {
    token: String
}


impl Client {
    pub fn get_currencies(&self) -> Result<Vec<Currency>, Error> {
        Currency::get_list()
    }
}
