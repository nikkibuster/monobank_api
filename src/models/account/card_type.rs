use serde::{Deserialize, Deserializer};

#[derive(Default, serde::Serialize, Clone, PartialEq, Eq)]
pub enum CardType {
    Black,
    White,
    Platinum,
    Iron,
    Fop,
    Yellow,
    EAid,
    #[default]
    None,
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
            Self::Platinum => write!(f, "Platinum"),
            Self::Iron => write!(f, "Iron"),
            Self::Fop => write!(f, "Fop"),
            Self::Yellow => write!(f, "Yellow"),
            Self::EAid => write!(f, "None"),
            Self::None => write!(f, "None"),
        }
    }
}

impl<'de> Deserialize<'de> for CardType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let card_type: &str = Deserialize::deserialize(deserializer)?;
        match card_type {
            "black" => return Ok(CardType::Black),
            "white" => return Ok(CardType::White),
            "platinum" => return Ok(CardType::Platinum),
            "iron" => return Ok(CardType::Iron),
            "fop" => return Ok(CardType::Fop),
            "yellow" => return Ok(CardType::Yellow),
            "eAid" => return Ok(CardType::EAid),
            _ => return Ok(CardType::None),
        }
    }
}
