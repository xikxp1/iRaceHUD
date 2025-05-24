use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct ProximityOverlaySettings {
    pub enabled: bool,
    pub x: i32,
    pub y: i32,
    pub gap_width: u32,
}
