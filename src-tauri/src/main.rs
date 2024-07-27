// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::OnceLock, time::Duration};

use anyhow::Result;
use iracing_telem::{Client, DataUpdateResult};
use log::{debug, info};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu};
use tauri_plugin_log::LogTarget;

static WINDOW: OnceLock<tauri::Window> = OnceLock::new();

struct TelemetryData {
    gear: String,
    speed: u32,
    rpm: u32,
    lap: u32,
    lap_time: Duration,
    brake: u32,
    throttle: u32,
    position: u32,
    incidents: u32,
}

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
                    let mut data = TelemetryData {
                        gear: String::from("N"),
                        speed: 0,
                        rpm: 0,
                        lap: 1,
                        lap_time: Duration::new(0, 0),
                        brake: 0,
                        throttle: 0,
                        position: 1,
                        incidents: 0,
                    };
                    let gear = s.find_var("Gear").unwrap();
                    let speed = s.find_var("Speed").unwrap();
                    let rpm = s.find_var("RPM").unwrap();
                    let lap = s.find_var("Lap").unwrap();
                    let lap_time = s.find_var("SessionTime").unwrap();
                    let brake = s.find_var("Brake").unwrap();
                    let throttle = s.find_var("Throttle").unwrap();
                    let position = s.find_var("PlayerCarClassPosition").unwrap();
                    let incidents = s.find_var("PlayerCarMyIncidentCount").unwrap();
                    loop {
                        match s.wait_for_data(Duration::from_millis(20)) {
                            DataUpdateResult::Updated => {
                                if WINDOW.get().is_none() {
                                    debug!("no main window");
                                    break;
                                }

                                // gear
                                let raw_gear_value: i32 = s.value(&gear)?;
                                if let Some(window) = WINDOW.get() {
                                    let gear_value = match raw_gear_value {
                                        -1 => String::from("R"),
                                        0 => String::from("N"),
                                        _ => raw_gear_value.to_string(),
                                    };
                                    if gear_value != data.gear {
                                        let _ = window.emit("gear", gear_value.clone());
                                        data.gear = gear_value;
                                    }
                                }

                                // speed
                                let raw_speed_value: f32 = s.value(&speed)?;
                                if let Some(window) = WINDOW.get() {
                                    let speed_value = (raw_speed_value * 3.6).round() as u32;
                                    if speed_value != data.speed {
                                        let _ = window.emit("speed", speed_value);
                                        data.speed = speed_value;
                                    }
                                }

                                // rpm
                                let raw_rpm_value: f32 = s.value(&rpm)?;
                                if let Some(window) = WINDOW.get() {
                                    let rpm_value = raw_rpm_value.round() as u32;
                                    if rpm_value != data.rpm {
                                        let _ = window.emit("rpm", rpm_value);
                                        data.rpm = rpm_value;
                                    }
                                }

                                // lap
                                let raw_lap_value: i32 = s.value(&lap)?;
                                if let Some(window) = WINDOW.get() {
                                    let lap_value = raw_lap_value as u32;
                                    if lap_value != data.lap {
                                        let _ = window.emit("lap", lap_value);
                                        data.lap = lap_value;
                                    }
                                }

                                // lap_time
                                let raw_lap_time_value: f64 = s.value(&lap_time)?;
                                if let Some(window) = WINDOW.get() {
                                    let lap_time_value =
                                        Duration::from_secs_f64(raw_lap_time_value);
                                    let _ = window.emit(
                                        "lap_time",
                                        format!(
                                            "{}:{:02}:{:03}",
                                            lap_time_value.as_secs() / 60,
                                            lap_time_value.as_secs() % 60,
                                            lap_time_value.subsec_millis()
                                        ),
                                    );
                                    data.lap_time = lap_time_value;
                                }

                                // brake
                                let raw_brake_value: f32 = s.value(&brake)?;
                                if let Some(window) = WINDOW.get() {
                                    let brake_value = (raw_brake_value * 100.0).round() as u32;
                                    let _ = window.emit("brake", brake_value);
                                    data.brake = brake_value;
                                }

                                // throttle
                                let raw_throttle_value: f32 = s.value(&throttle)?;
                                if let Some(window) = WINDOW.get() {
                                    let throttle_value =
                                        (raw_throttle_value * 100.0).round() as u32;
                                    let _ = window.emit("throttle", throttle_value);
                                    data.throttle = throttle_value;
                                }

                                // position
                                let raw_position_value: i32 = s.value(&position)?;
                                if let Some(window) = WINDOW.get() {
                                    let position_value = raw_position_value as u32;
                                    if position_value != data.position {
                                        let _ = window.emit("position", position_value);
                                        data.position = position_value;
                                    }
                                }

                                // incidents
                                let raw_incidents_value: i32 = s.value(&incidents)?;
                                if let Some(window) = WINDOW.get() {
                                    let incidents_value = raw_incidents_value as u32;
                                    if incidents_value != data.incidents {
                                        let _ = window.emit("incident_count", incidents_value);
                                        data.incidents = incidents_value;
                                    }
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
