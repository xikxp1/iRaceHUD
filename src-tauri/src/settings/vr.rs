use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Type, Serialize, Deserialize, Clone)]
pub struct VrSettings {
    pub enabled: bool,
    pub http_port: u16,
    pub external_access: bool,
}

impl VrSettings {
    pub fn default_settings() -> Self {
        Self {
            enabled: false,
            http_port: 8080,
            external_access: false,
        }
    }
}
