use super::common_settings::{CommonSettings, HasCommonSettings};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct LapTimesOverlaySettings {
    pub common_settings: CommonSettings,
}

impl HasCommonSettings for LapTimesOverlaySettings {
    fn common_settings(&self) -> &CommonSettings {
        &self.common_settings
    }
}
