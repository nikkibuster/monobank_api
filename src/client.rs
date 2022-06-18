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

pub trait Banking {
    fn get_currencies(&self) -> Result<Vec<Currency>, Error> {
        Currency::get_list()
    }
}


#[derive(Clone)]
pub struct PersonalClient {
    token: String
}


impl PersonalClient {
    pub fn get_currencies(&self) -> Result<Vec<Currency>, Error> {
        Currency::get_list()
    }

    pub fn get_info(&self) -> Result<Account, Error> {
        Account::get_info(&self.token)
    }

    pub fn get_statements(&self, account: &str, from: i64, to: i64) -> Result<Vec<Statement>, Error>  {
        Statement::get_list(&self.token, account, from, to)
    }
}