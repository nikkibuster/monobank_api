use super::models::{
    currency::Currency,
    statement::Statement,
    account::Account
};

use reqwest::Error;



pub struct ClientBuilder;

impl ClientBuilder {
    pub fn new() -> Self {
        ClientBuilder{}
    }

    pub fn with_token(&mut self, token: &str) -> PersonalClientBuilder{

        PersonalClientBuilder::new(token.to_string())
    }

    pub fn build(&self) -> BaseClient {
        BaseClient
    }
}

pub struct PersonalClientBuilder {
    token: String
}

impl PersonalClientBuilder {
    pub fn new(token: String) -> Self {
        Self{token}
    }

    pub fn build(&self) -> PersonalClient {
        PersonalClient{token: self.token.clone()}
    }
}

pub struct BaseClient;

impl Banking for BaseClient{}

trait Banking {
    fn get_currencies(&self) -> Result<Vec<Currency>, Error> {
        Currency::get_list()
    }
}

trait Personal {
    fn get_info(token: &str) -> Result<Account, Error> {
        Account::get_info(token)
    }

    fn get_statements(token: &str, account: &str, from: i64, to: i64) -> Result<Vec<Statement>, Error> {
        Statement::get_list(token, account, from, to)
    }
}

pub struct PersonalClient {
    token: String
}


impl Banking for PersonalClient {}
impl Personal for PersonalClient {}