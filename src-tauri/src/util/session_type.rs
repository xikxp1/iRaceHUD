use strum_macros::{Display, EnumIter, EnumString};

#[derive(EnumString, EnumIter, Display, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum SessionType {
    #[strum(default)]
    Unknown(String),
    Practice,
    Qualify,
    Race,
}

impl Default for SessionType {
    fn default() -> Self {
        SessionType::Unknown("unknown".to_string())
    }
}
