use serde::{Deserialize, Serialize};
use specta::Type;

use super::common_settings::{CommonSettings, HasCommonSettings};

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct StandingsOverlaySettings {
    pub common_settings: CommonSettings,
    pub max_drivers: u32,
    pub top_drivers: u32,
}

impl HasCommonSettings for StandingsOverlaySettings {
    fn common_settings(&self) -> &CommonSettings {
        &self.common_settings
    }
}
