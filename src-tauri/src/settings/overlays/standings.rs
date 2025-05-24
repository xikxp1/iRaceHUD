use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct StandingsOverlaySettings {
    pub enabled: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub opacity: u32,
    pub max_drivers: u32,
    pub top_drivers: u32,
}
