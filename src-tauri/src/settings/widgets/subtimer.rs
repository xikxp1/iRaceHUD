use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct SubTimerWidgetSettings {
    pub enabled: bool,
    pub x: i32,
    pub y: i32,
    pub session_state_width: u32,
    pub gap_enabled: bool,
    pub gap_width: u32,
    pub opacity: u32,
}
