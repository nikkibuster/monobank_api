pub mod account;
pub mod currency;
mod date;
pub(crate) mod methods;
pub mod statement;

const KEY_ACCOUNT: &str = "{account}";
const KEY_FROM: &str = "{from}";
const KEY_TO: &str = "{to}";

pub struct StatementReq {
    pub account: String,
    pub from: i64,
    pub to: Option<i64>,
}

impl StatementReq {
    pub(crate) fn prepare_url(self) -> String {
        let base = crate::models::methods::Method::Statements.path();
        let from_str = self.from.to_string();
        let to_str = match self.to {
            Some(to) => to.to_string(),
            None => String::new(),
        };

        let splits: Vec<&str> = base
            .split("/")
            .map(|part| match part {
                KEY_ACCOUNT => &self.account,
                KEY_FROM => &from_str,
                KEY_TO => &to_str,
                _ => part,
            })
            .collect();

        splits.join("/")
    }
}

impl Default for StatementReq {
    fn default() -> Self {
        Self {
            account: String::from("0"),
            from: default_from(),
            to: None,
        }
    }
}

impl StatementsArgs for StatementReq {
    fn unwrap(self) -> StatementReq {
        self
    }
}

pub trait StatementsArgs {
    fn unwrap(self) -> StatementReq;
}

fn default_from() -> i64 {
    (chrono::Utc::now() - chrono::Duration::hours(24)).timestamp()
}

impl StatementsArgs for () {
    fn unwrap(self) -> StatementReq {
        StatementReq::default()
    }
}

impl StatementsArgs for &'static str {
    fn unwrap(self) -> StatementReq {
        let mut account = self;
        if account.is_empty() {
            account = "0"
        }

        StatementReq {
            account: account.to_string(),
            ..StatementReq::default()
        }
    }
}

impl StatementsArgs for (i64, i64) {
    fn unwrap(self) -> StatementReq {
        StatementReq {
            from: self.0,
            to: Some(self.1),
            ..StatementReq::default()
        }
    }
}

impl StatementsArgs for (&'static str, i64) {
    fn unwrap(self) -> StatementReq {
        let mut account = self.0;

        if account.is_empty() {
            account = "0"
        }

        StatementReq {
            account: account.to_string(),
            from: self.1,
            to: None,
        }
    }
}

impl StatementsArgs for (&'static str, i64, i64) {
    fn unwrap(self) -> StatementReq {
        let mut account = self.0;

        if account.is_empty() {
            account = "0"
        }
        StatementReq {
            account: account.to_string(),
            from: self.1,
            to: Some(self.2),
        }
    }
}

impl StatementsArgs
    for (
        &'static str,
        chrono::DateTime<chrono::Utc>,
        chrono::DateTime<chrono::Utc>,
    )
{
    fn unwrap(self) -> StatementReq {
        let mut account = self.0;
        if account.is_empty() {
            account = "0"
        }

        StatementReq {
            account: account.to_string(),
            from: self.1.timestamp(),
            to: Some(self.2.timestamp()),
        }
    }
}

impl StatementsArgs for (&'static str, chrono::DateTime<chrono::Utc>) {
    fn unwrap(self) -> StatementReq {
        let mut account = self.0;
        if account.is_empty() {
            account = "0"
        }

        StatementReq {
            account: account.to_string(),
            from: self.1.timestamp(),
            to: None,
        }
    }
}

impl StatementsArgs for (chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>) {
    fn unwrap(self) -> StatementReq {
        StatementReq {
            from: self.0.timestamp(),
            to: Some(self.1.timestamp()),
            ..StatementReq::default()
        }
    }
}

impl StatementsArgs for chrono::DateTime<chrono::Utc> {
    fn unwrap(self) -> StatementReq {
        StatementReq {
            from: self.timestamp(),
            ..StatementReq::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_url<T: StatementsArgs>(args: T) -> String {
        args.unwrap().prepare_url()
    }

    #[test]
    fn prepare_url() {
        let url = get_url(());

        println!("{}", url)
    }

    #[test]
    fn from_time() {
        let from = chrono::Utc::now();

        let url = get_url(from);

        println!("{}", url)
    }
}
