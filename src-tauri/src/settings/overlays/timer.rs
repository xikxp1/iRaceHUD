use serde::{Deserialize, Serialize};
use specta::Type;

use super::common_settings::{CommonSettings, HasCommonSettings};

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct TimerOverlaySettings {
    pub common_settings: CommonSettings,
    pub delta_enabled: bool,
    pub delta_width: u32,
}

impl HasCommonSettings for TimerOverlaySettings {
    fn common_settings(&self) -> &CommonSettings {
        &self.common_settings
    }
}
