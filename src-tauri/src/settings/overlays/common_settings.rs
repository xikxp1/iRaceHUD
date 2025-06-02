use serde::{Deserialize, Serialize};
use specta::Type;

pub trait HasCommonSettings {
    fn common_settings(&self) -> &CommonSettings;
}

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct CommonSettings {
    pub enabled: bool,
    pub width: u32,
    pub height: u32,
    pub opacity: u32,
    pub scale: u32,
    pub x: u32,
    pub y: u32,
}
