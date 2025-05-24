use std::fs::File;

use serde::{de::DeserializeOwned, Serialize};
use tauri::{path::BaseDirectory, Manager};
use tauri_plugin_store::StoreExt;

use crate::emitter::settings_emitter::emit_settings_update;

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

pub fn set_settings<T: Serialize>(app: tauri::AppHandle, overlay_name: &str, settings: T) {
    let store = app.store(format!("{}_overlay.json", overlay_name)).unwrap();
    store.set("settings", serde_json::to_value(&settings).unwrap());
    store.save().unwrap();

    // Broadcast settings update via WebSocket
    emit_settings_update(overlay_name);
}
