use std::fs::File;

use serde::{de::DeserializeOwned, Serialize};
use tauri::{path::BaseDirectory, Emitter, Manager};
use tauri_plugin_store::StoreExt;

pub fn get_settings<T: DeserializeOwned>(app: tauri::AppHandle, widget_name: &str) -> T {
    let store = app.store(format!("{}_widget.json", widget_name)).unwrap();
    let settings = store.get("settings");
    let settings: serde_json::Value = match settings {
        Some(settings) => settings,
        None => {
            let resource_path = app
                .path()
                .resolve(
                    format!("data/default_settings/{}_widget.json", widget_name),
                    BaseDirectory::Resource,
                )
                .unwrap();
            let file = File::open(resource_path).unwrap();
            let settings: serde_json::Value = serde_json::from_reader(file).unwrap();
            store.set("settings", settings.clone());
            store.save().unwrap();
            settings
        }
    };
    serde_json::from_value(settings).unwrap()
}

pub fn set_settings<T: Serialize + Clone>(app: tauri::AppHandle, widget_name: &str, settings: T) {
    let store = app.store(format!("{}_widget.json", widget_name)).unwrap();
    store.set("settings", serde_json::to_value(&settings).unwrap());
    store.save().unwrap();
    app.emit(&format!("{}_widget_settings_changed", widget_name), settings)
        .unwrap();
}
