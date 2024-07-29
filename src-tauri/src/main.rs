// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eyre::{OptionExt, Result};
use iracing_telem::{Client, DataUpdateResult};
use log::{debug, error, info};
use std::{sync::OnceLock, time::Duration};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu};
use tauri_plugin_log::LogTarget;
use yaml_rust2::YamlLoader;

static WINDOW: OnceLock<tauri::Window> = OnceLock::new();
const SLOW_VAR_RESET_TICKS: u32 = 50;

struct TelemetryData {
    active: bool,
    session_time: Duration,
    lap: u32,
    lap_time: Duration,
    gear: String,
    speed: u32,
    rpm: u32,
    brake: u32,
    throttle: u32,
    position: u32,
    positions_total: u32,
    session_time_total: Duration,
    laps_total: u32,
    incidents: u32,
    incident_limit: u32,
    gear_shift_rpm: u32,
    gear_last_rpm: u32,
    gear_blink_rpm: u32,
    session_info_update: i32,
}

impl TelemetryData {
    fn new() -> Self {
        Self {
            active: false,
            session_time: Duration::new(0, 0),
            lap: 0,
            lap_time: Duration::new(0, 0),
            gear: String::from("N"),
            speed: 0,
            rpm: 0,
            brake: 0,
            throttle: 0,
            position: 0,
            positions_total: 0,
            session_time_total: Duration::new(0, 0),
            laps_total: 0,
            incidents: 0,
            incident_limit: 0,
            gear_shift_rpm: 0,
            gear_last_rpm: 0,
            gear_blink_rpm: 0,
            session_info_update: 0,
        }
    }
}

fn main() {
    let _ = color_eyre::install();

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
                .expect("Failed to set ignore cursor events");

            WINDOW.set(window).expect("Failed to set window");

            tauri::async_runtime::spawn(async move {
                connect().expect("Error while connecting to iRacing");
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
        .expect("Error while running tauri application");
}

pub fn connect() -> Result<()> {
    let mut c = Client::new();
    loop {
        info!("Start iRacing");
        unsafe {
            match c.wait_for_session(Duration::new(600, 0)) {
                None => {
                    info!("Remember to start iRacing!");
                    return Ok(());
                }
                Some(mut s) => {
                    let mut data = TelemetryData::new();
                    let is_on_track = s
                        .find_var("IsOnTrack")
                        .ok_or_eyre("IsOnTrack variable not found")?;
                    let is_on_track_car = s
                        .find_var("IsOnTrackCar")
                        .ok_or_eyre("IsOnTrackCar variable not found")?;
                    let session_time = s
                        .find_var("SessionTime")
                        .ok_or_eyre("SessionTime variable not found")?;
                    let lap_current_lap_time = s
                        .find_var("LapCurrentLapTime")
                        .ok_or_eyre("LapCurrentLapTime variable not found")?;
                    let lap_dist_pct = s
                        .find_var("LapDistPct")
                        .ok_or_eyre("LapDistPct variable not found")?;
                    let gear = s.find_var("Gear").ok_or_eyre("Gear variable not found")?;
                    let speed = s.find_var("Speed").ok_or_eyre("Speed variable not found")?;
                    let rpm = s.find_var("RPM").ok_or_eyre("RPM variable not found")?;
                    let lap = s.find_var("Lap").ok_or_eyre("Lap variable not found")?;
                    let brake = s.find_var("Brake").ok_or_eyre("Brake variable not found")?;
                    let throttle = s
                        .find_var("Throttle")
                        .ok_or_eyre("Throttle variable not found")?;
                    let position = s
                        .find_var("PlayerCarClassPosition")
                        .ok_or_eyre("PlayerCarClassPosition variable not found")?;
                    let session_time_total = s
                        .find_var("SessionTimeTotal")
                        .ok_or_eyre("SessionTimeTotal variable not found")?;
                    let session_laps_total = s
                        .find_var("SessionLapsTotal")
                        .ok_or_eyre("SessionLapsTotal variable not found")?;
                    let incidents = s
                        .find_var("PlayerCarMyIncidentCount")
                        .ok_or_eyre("PlayerCarMyIncidentCount variable not found")?;
                    // let incident_limit = s
                    //     .find_var("IncidentLimit")
                    //     .ok_or_eyre("IncidentLimit variable not found")?;
                    // let cars = s.find_var("Cars").ok_or_eyre("Cars variable not found")?;
                    let player_car_sl_shift_rpm = s
                        .find_var("PlayerCarSLShiftRPM")
                        .ok_or_eyre("PlayerCarSLShiftRPM variable not found")?;
                    let player_car_sl_blink_rpm = s
                        .find_var("PlayerCarSLBlinkRPM")
                        .ok_or_eyre("PlayerCarSLBlinkRPM variable not found")?;
                    let mut slow_var_ticks: u32 = 0;
                    loop {
                        match s.wait_for_data(Duration::from_millis(20)) {
                            DataUpdateResult::Updated => {
                                slow_var_ticks += 1;
                                let window_opt = WINDOW.get();

                                if window_opt.is_none() {
                                    error!("Window is none");
                                    break;
                                }

                                let window = window_opt.ok_or_eyre("Window is none")?;

                                // active
                                let raw_is_on_track_value: bool = match s.value(&is_on_track) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get IsOnTrack value: {:?}", err);
                                        continue;
                                    }
                                };
                                let raw_is_on_track_car_value: bool =
                                    match s.value(&is_on_track_car) {
                                        Ok(value) => value,
                                        Err(err) => {
                                            error!("Failed to get IsOnTrackCar value: {:?}", err);
                                            continue;
                                        }
                                    };

                                let active = raw_is_on_track_value && raw_is_on_track_car_value;
                                let _ = window.emit("active", active);

                                if active != data.active {
                                    info!(
                                        "Session state changed to {}",
                                        if active { "active" } else { "inactive" }
                                    );
                                    data.active = active;
                                }

                                if !active {
                                    continue;
                                }

                                // session_time
                                let raw_session_time_value: f64 = match s.value(&session_time) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get SessionTime value: {:?}", err);
                                        continue;
                                    }
                                };
                                let session_time_value =
                                    Duration::from_secs_f64(raw_session_time_value);
                                let _ = window.emit(
                                    "session_time",
                                    format!(
                                        "{}:{:02}",
                                        session_time_value.as_secs() / 60,
                                        session_time_value.as_secs() % 60,
                                    ),
                                );
                                data.session_time = session_time_value;

                                // lap
                                let raw_lap_value: i32 = match s.value(&lap) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get Lap value: {:?}", err);
                                        continue;
                                    }
                                };
                                let lap_value = raw_lap_value as u32;
                                let _ = window.emit("lap", lap_value);
                                data.lap = lap_value;

                                // lap_time
                                let raw_lap_current_lap_time_value: f32 = match s
                                    .value(&lap_current_lap_time)
                                {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get LapCurrentLapTime value: {:?}", err);
                                        continue;
                                    }
                                };
                                let lap_time_value =
                                    Duration::from_secs_f32(raw_lap_current_lap_time_value);
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

                                // gear
                                let raw_gear_value: i32 = match s.value(&gear) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get Gear value: {:?}", err);
                                        continue;
                                    }
                                };
                                let gear_value = match raw_gear_value {
                                    -1 => String::from("R"),
                                    0 => String::from("N"),
                                    _ => raw_gear_value.to_string(),
                                };
                                let _ = window.emit("gear", gear_value.clone());
                                data.gear = gear_value;

                                // speed
                                let raw_speed_value: f32 = match s.value(&speed) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get Speed value: {:?}", err);
                                        continue;
                                    }
                                };
                                let speed_value = (raw_speed_value * 3.6).round() as u32;
                                let _ = window.emit("speed", speed_value);
                                data.speed = speed_value;

                                // rpm
                                let raw_rpm_value: f32 = match s.value(&rpm) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get RPM value: {:?}", err);
                                        continue;
                                    }
                                };
                                let rpm_value = raw_rpm_value.round() as u32;
                                let _ = window.emit("rpm", rpm_value);
                                data.rpm = rpm_value;

                                // brake
                                let raw_brake_value: f32 = match s.value(&brake) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get Brake value: {:?}", err);
                                        continue;
                                    }
                                };
                                let brake_value = (raw_brake_value * 100.0).round() as u32;
                                let _ = window.emit("brake", brake_value);
                                data.brake = brake_value;

                                // throttle
                                let raw_throttle_value: f32 = match s.value(&throttle) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get Throttle value: {:?}", err);
                                        continue;
                                    }
                                };
                                let throttle_value = (raw_throttle_value * 100.0).round() as u32;
                                let _ = window.emit("throttle", throttle_value);
                                data.throttle = throttle_value;

                                // position
                                let raw_position_value: i32 = match s.value(&position) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!(
                                            "Failed to get PlayerCarClassPosition value: {:?}",
                                            err
                                        );
                                        continue;
                                    }
                                };
                                let position_value = raw_position_value as u32;
                                let _ = window.emit("position", position_value);
                                data.position = position_value;

                                if slow_var_ticks < SLOW_VAR_RESET_TICKS {
                                    continue;
                                }

                                // slow vars

                                // session_time_total
                                let raw_session_time_total_value: f64 = match s
                                    .value(&session_time_total)
                                {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get SessionTimeTotal value: {:?}", err);
                                        continue;
                                    }
                                };
                                let session_time_total_value =
                                    Duration::from_secs_f64(raw_session_time_total_value);
                                let _ = window.emit(
                                    "session_time_total",
                                    format!(
                                        "{}:{:02}",
                                        session_time_total_value.as_secs() / 60,
                                        session_time_total_value.as_secs() % 60,
                                    ),
                                );
                                data.session_time_total = session_time_total_value;

                                // session_laps_total
                                let raw_session_laps_total_value: i32 = match s
                                    .value(&session_laps_total)
                                {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get SessionLapsTotal value: {:?}", err);
                                        continue;
                                    }
                                };
                                let mut laps_total_value = raw_session_laps_total_value as u32;
                                if laps_total_value >= i16::MAX as u32 {
                                    laps_total_value = 0;
                                }
                                let _ = window.emit("laps_total", laps_total_value);
                                data.laps_total = laps_total_value;

                                // incidents
                                let raw_incidents_value: i32 = match s.value(&incidents) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!(
                                            "Failed to get PlayerCarMyIncidentCount value: {:?}",
                                            err
                                        );
                                        continue;
                                    }
                                };
                                let incidents_value = raw_incidents_value as u32;
                                let _ = window.emit("incidents", incidents_value);
                                data.incidents = incidents_value;

                                // incident_limit
                                // let raw_incident_limit_value: i32 = match s.value(&incident_limit) {
                                //     Ok(value) => value,
                                //     Err(err) => {
                                //         error!("Failed to get IncidentLimit value: {:?}", err);
                                //         continue;
                                //     }
                                // };
                                // let incident_limit_value = raw_incident_limit_value as u32;
                                // let _ = window.emit("incident_limit", incident_limit_value);
                                // data.incident_limit = incident_limit_value;

                                // positions_total
                                // let raw_cars_value: i32 = match s.value(&cars) {
                                //     Ok(value) => value,
                                //     Err(err) => {
                                //         error!("Failed to get Cars value: {:?}", err);
                                //         continue;
                                //     }
                                // };
                                // let positions_total_value = raw_cars_value as u32;
                                // let _ = window.emit("positions_total", positions_total_value);
                                // data.positions_total = positions_total_value;

                                // gear_shift_rpm
                                let raw_player_car_sl_shift_rpm_value: f32 =
                                    match s.value(&player_car_sl_shift_rpm) {
                                        Ok(value) => value,
                                        Err(err) => {
                                            error!(
                                                "Failed to get PlayerCarSLShiftRPM value: {:?}",
                                                err
                                            );
                                            continue;
                                        }
                                    };
                                let gear_shift_rpm_value =
                                    raw_player_car_sl_shift_rpm_value.round() as u32;
                                let _ = window.emit("gear_shift_rpm", gear_shift_rpm_value);
                                data.gear_shift_rpm = gear_shift_rpm_value;

                                // gear_blink_rpm
                                let raw_player_car_sl_blink_rpm_value: f32 =
                                    match s.value(&player_car_sl_blink_rpm) {
                                        Ok(value) => value,
                                        Err(err) => {
                                            error!(
                                                "Failed to get PlayerCarSLBlinkRPM value: {:?}",
                                                err
                                            );
                                            continue;
                                        }
                                    };
                                let gear_blink_rpm_value =
                                    raw_player_car_sl_blink_rpm_value.round() as u32;
                                let _ = window.emit("gear_blink_rpm", gear_blink_rpm_value);
                                data.gear_blink_rpm = gear_blink_rpm_value;

                                slow_var_ticks = 0;
                            }
                            DataUpdateResult::NoUpdate => {
                                debug!("No update")
                            }
                            DataUpdateResult::FailedToCopyRow => {
                                debug!("Too slow")
                            }
                            DataUpdateResult::SessionExpired => {
                                info!("Session expired");
                                break;
                            }
                        }
                        // s.dump_vars();
                        let session_info_update = s.session_info_update();
                        if data.session_info_update != session_info_update {
                            debug!("Session info updated");
                            let session_info = YamlLoader::load_from_str(&s.session_info())?;
                            // debug!("{:#?}", session_info);
                            data.session_info_update = session_info_update;
                        }
                    }
                }
            }
        }
    }
}
