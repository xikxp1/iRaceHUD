use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct LapTimesWidgetSettings {
    pub enabled: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub opacity: u32,
}
