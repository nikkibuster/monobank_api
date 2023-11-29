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
