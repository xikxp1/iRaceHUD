use serde::{Deserialize, Serialize};
use specta::Type;

use super::common_settings::{CommonSettings, HasCommonSettings};

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct TelemetryOverlaySettings {
    pub common_settings: CommonSettings,
}

impl HasCommonSettings for TelemetryOverlaySettings {
    fn common_settings(&self) -> &CommonSettings {
        &self.common_settings
    }
}
