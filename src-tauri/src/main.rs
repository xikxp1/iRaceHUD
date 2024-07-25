// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::OnceLock, time::Duration};

use anyhow::Result;
use iracing_telem::{Client, DataUpdateResult};
use log::{debug, info};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu};
use tauri_plugin_log::LogTarget;

static WINDOW: OnceLock<tauri::Window> = OnceLock::new();

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

            WINDOW
                .set(window)
                .expect("Failed to save main window reference");

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

pub fn connect() -> Result<(), iracing_telem::Error> {
    let mut c = Client::new();
    loop {
        debug!("start iRacing");
        unsafe {
            match c.wait_for_session(Duration::new(600, 0)) {
                None => {
                    println!("remember to start iRacing!");
                    return Ok(());
                }
                Some(mut s) => {
                    let gear = s.find_var("Gear").unwrap();
                    let speed = s.find_var("Speed").unwrap();
                    loop {
                        match s.wait_for_data(Duration::from_millis(20)) {
                            DataUpdateResult::Updated => {
                                if WINDOW.get().is_none() {
                                    debug!("no main window");
                                    break;
                                }
                                let raw_gear_value: i32 = s.value(&gear)?;
                                if let Some(window) = WINDOW.get() {
                                    let gear_value = match raw_gear_value {
                                        -1 => String::from("R"),
                                        0 => String::from("N"),
                                        _ => raw_gear_value.to_string(),
                                    };
                                    let _ = window.emit("gear", gear_value);
                                }
                                let raw_speed_value: f32 = s.value(&speed)?;
                                if let Some(window) = WINDOW.get() {
                                    let speed_value = (raw_speed_value * 3.6).round() as u32;
                                    let _ = window.emit("speed", speed_value);
                                }
                            }
                            DataUpdateResult::NoUpdate => {
                                debug!("no update")
                            }
                            DataUpdateResult::FailedToCopyRow => {
                                debug!("too slow")
                            }
                            DataUpdateResult::SessionExpired => break,
                        }
                    }
                }
            }
        }
    }
}
