
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Type, Serialize, Deserialize, Clone)]
pub struct TimerWidgetSettings {
    pub enabled: bool,
    pub x: i32,
    pub y: i32,
    pub lap_time_width: u32,
    pub delta_enabled: bool,
    pub delta_width: u32,
    pub opacity: u32,
}
