use std::str::FromStr;

use models::currency::Currency;
use models::statement::FormattedStatement;

use reqwest::Request;
use thiserror::Error;

mod constants;
mod models;

use models::methods::Method;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Build client error")]
    HttpClient(#[from] reqwest::Error),

    #[error("Failed to set authorization token")]
    InvalidToken(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Cannot parse url: {0}")]
    ParseUrl(#[from] url::ParseError),

    // InvalidUrl(#[from] reqwest::Url)
    #[error("Validation: {0}")]
    Validation(&'static str),

    #[error("some internal error")]
    Internal,
}

pub struct Public;
pub struct Private;

pub struct Client<State = Public> {
    client: reqwest::Client,
    state: std::marker::PhantomData<State>,
}

impl Client {
    pub fn new() -> Result<Self, ClientError> {
        let client = reqwest::Client::builder().build()?;

        Ok(Self {
            client,
            state: std::marker::PhantomData::default(),
        })
    }
}

impl<State> Client<State> {
    pub async fn currencies(&self) -> Result<Vec<Currency>, ClientError> {
        let currencies: Vec<Currency> = self
            .client
            .get(Method::Currencies.path())
            .send()
            .await?
            .json()
            .await?;

        Ok(currencies)
    }
}

impl Client<Public> {
    pub fn with_token<'a>(self, token: &'a str) -> Result<Client<Private>, ClientError> {
        if token.is_empty() {
            return Err(ClientError::Validation("token required"));
        }

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            constants::TOKEN_KEY,
            reqwest::header::HeaderValue::from_str(token)?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Client {
            client: client,
            state: std::marker::PhantomData::<Private>,
        })
    }
}

impl Client<Private> {
    pub async fn account_info(&self) -> Result<models::account::Account, ClientError> {
        let req = Request::new(
            reqwest::Method::GET,
            reqwest::Url::from_str(Method::Account.path())?,
        );

        let mut account: models::account::Account = self.client.execute(req).await?.json().await?;
        account.format();

        Ok(account)
    }

    pub async fn statements(
        &self,
        account: &'static str,
        from: i64,
        to: i64,
    ) -> Result<Vec<models::statement::Statement>, ClientError> {
        let url = models::statement::prepare_url(account, from, to);

        let req = Request::new(reqwest::Method::GET, reqwest::Url::from_str(&url)?);

        let mut result = self
            .client
            .execute(req)
            .await?
            .json::<Vec<models::statement::Statement>>()
            .await?;

        result.format();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_currencies() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new()?;

        let result = client.currencies().await?;

        assert!(!result.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn info() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
        let token = std::env::var(super::constants::TOKEN)?;

        let client = Client::new()?.with_token(&token)?;

        let result = client.account_info().await?;

        assert!(!result.name().is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn statements() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
        let token = std::env::var(super::constants::TOKEN)?;

        let client = Client::new()?.with_token(&token)?;

        let from = chrono::Utc::now();

        let from = from - chrono::Duration::hours(24);

        let result = client.statements("0", from.timestamp(), 0).await?;

        assert!(!result.is_empty());

        Ok(())
    }
}
