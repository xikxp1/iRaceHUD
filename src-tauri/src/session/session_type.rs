use std::str::FromStr;

use strum_macros::{Display, EnumIter};

#[derive(EnumIter, Display, PartialEq, Debug)]
#[strum(serialize_all = "PascalCase")]
pub enum SessionType {
    #[strum(default)]
    Unknown(String),
    Practice,
    Qualify,
    Race,
}

impl Default for SessionType {
    fn default() -> Self {
        SessionType::Unknown("Unknown".to_string())
    }
}

impl FromStr for SessionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Practice" => Ok(SessionType::Practice),
            "Lone Qualify" => Ok(SessionType::Qualify),
            "Open Qualify" => Ok(SessionType::Qualify),
            "Race" => Ok(SessionType::Race),
            _ => Ok(SessionType::Unknown(s.to_string())),
        }
    }
}
