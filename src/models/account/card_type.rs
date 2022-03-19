use serde::{Deserialize, Deserializer};

pub enum CardType {
    Black,
    White,
    None
}

impl CardType {
    pub fn is_black(&self) -> bool {
        matches!(self, Self::Black)
    }

    pub fn is_white(&self) -> bool {
        matches!(self, Self::White)
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl std::fmt::Debug for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::White => write!(f, "White"),
            Self::None => write!(f, "None"),
        }
    }
}

impl<'de> Deserialize<'de> for CardType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            let card_type: &str = Deserialize::deserialize(deserializer)?;
            match card_type {
                "black" => return Ok(CardType::Black),
                "white" => return Ok(CardType::White),
                _ => return Ok(CardType::None),
            }
    }
}
