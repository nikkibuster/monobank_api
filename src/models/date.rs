use std::str::FromStr;

use chrono::{Utc, NaiveDateTime};
use serde::{Deserialize, Deserializer, de::Error};

#[derive(Debug)]
pub struct Date {
    time: NaiveDateTime
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        let temp: i64 = Deserialize::deserialize(deserializer)?;
        let date = NaiveDateTime::from_timestamp(temp, 0);

        Ok(Date{time: date})
    }

}