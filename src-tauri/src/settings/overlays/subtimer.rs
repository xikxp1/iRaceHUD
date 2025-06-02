use serde::{Deserialize, Serialize};
use specta::Type;

use super::common_settings::{CommonSettings, HasCommonSettings};

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct SubTimerOverlaySettings {
    pub common_settings: CommonSettings,
    pub gap_enabled: bool,
    pub gap_width: u32,
}

impl HasCommonSettings for SubTimerOverlaySettings {
    fn common_settings(&self) -> &CommonSettings {
        &self.common_settings
    }
}
