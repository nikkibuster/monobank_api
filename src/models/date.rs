use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Date {
    time: NaiveDateTime,
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let temp: i64 = Deserialize::deserialize(deserializer)?;
        let date = NaiveDateTime::from_timestamp(temp, 0); // TODO: fix deprecated

        Ok(Date { time: date })
    }
}
