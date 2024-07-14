// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iracehud::connect;
use log::info;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu};
use tauri_plugin_log::LogTarget;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .setup(|app| {
            let window = app.get_window("main").expect("Failed to get main window");
            window
                .set_ignore_cursor_events(true)
                .expect("Failed to set ignore cursor events on main window");

            tauri::async_runtime::spawn(async move {
                connect().expect("Error connecting");
            });

            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|_, event| match event {
            tauri::SystemTrayEvent::MenuItemClick { id, .. } if id == "quit" => {
                info!("Quit menu item clicked, quitting application");
                std::process::exit(0);
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
