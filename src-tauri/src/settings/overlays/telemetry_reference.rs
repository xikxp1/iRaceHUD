use serde::{Deserialize, Serialize};
use specta::Type;

use super::common_settings::{CommonSettings, HasCommonSettings};

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct TelemetryReferenceOverlaySettings {
    pub common_settings: CommonSettings,
}

impl HasCommonSettings for TelemetryReferenceOverlaySettings {
    fn common_settings(&self) -> &CommonSettings {
        &self.common_settings
    }
}
