use std::fs::File;

use serde::{Serialize, de::DeserializeOwned};
use tauri::{Manager, path::BaseDirectory};
use tauri_plugin_store::StoreExt;

use crate::{
    emitter::settings_emitter::emit_settings_update,
    settings::overlays::common_settings::{CommonSettings, HasCommonSettings},
};

pub fn get_settings<T: DeserializeOwned>(app: tauri::AppHandle, overlay_name: &str) -> T {
    let store = app.store(format!("{}_overlay.json", overlay_name)).unwrap();
    let settings = store.get("settings");
    let settings: serde_json::Value = match settings {
        Some(settings) => settings,
        None => {
            let resource_path = app
                .path()
                .resolve(
                    format!("data/default_settings/{}_overlay.json", overlay_name),
                    BaseDirectory::Resource,
                )
                .unwrap();
            let file = File::open(resource_path).unwrap();
            let settings: serde_json::Value = serde_json::from_reader(file).unwrap();
            let settings = settings["settings"].clone();
            store.set("settings", settings.clone());
            store.save().unwrap();
            settings
        }
    };
    serde_json::from_value(settings).unwrap()
}

pub fn get_common_settings<T: DeserializeOwned + HasCommonSettings>(
    app: tauri::AppHandle,
    overlay_name: &str,
) -> CommonSettings {
    let settings = get_settings::<T>(app, overlay_name);
    settings.common_settings().clone()
}

pub fn set_common_settings<T: Serialize>(
    app: tauri::AppHandle,
    overlay_name: &str,
    common_settings: CommonSettings,
) {
    let store = app.store(format!("{}_overlay.json", overlay_name)).unwrap();
    let old_settings = store.get("settings");
    let mut settings = old_settings.unwrap();
    settings["common_settings"] = serde_json::to_value(&common_settings).unwrap();
    store.set("settings", serde_json::to_value(&settings).unwrap());
    store.save().unwrap();
}

pub fn set_settings<T: Serialize + HasCommonSettings>(
    app: tauri::AppHandle,
    overlay_name: &str,
    settings: T,
) {
    let store = app.store(format!("{}_overlay.json", overlay_name)).unwrap();
    store.set("settings", serde_json::to_value(&settings).unwrap());
    store.save().unwrap();

    // Broadcast settings update via WebSocket
    emit_settings_update(overlay_name);
}
