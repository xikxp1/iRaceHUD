use serde::{Deserialize, Serialize};
use specta::Type;

use super::common_settings::{CommonSettings, HasCommonSettings};

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct TelemetryReferenceOverlaySettings {
    pub common_settings: CommonSettings,
    pub show_throttle: bool,
    pub show_steering: bool,
    pub brake_que_0_enabled: bool,
    pub brake_que_1_enabled: bool,
    pub brake_que_1_distance: u32,
    pub brake_que_2_enabled: bool,
    pub brake_que_2_distance: u32,
    pub brake_que_3_enabled: bool,
    pub brake_que_3_distance: u32,
}

impl HasCommonSettings for TelemetryReferenceOverlaySettings {
    fn common_settings(&self) -> &CommonSettings {
        &self.common_settings
    }
}
