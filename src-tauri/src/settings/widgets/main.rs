use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Type, Serialize, Deserialize, Default, Clone)]
pub struct MainWidgetSettings {
    pub enabled: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub opacity: u32,
}
