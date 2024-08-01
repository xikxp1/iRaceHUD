// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eyre::{OptionExt, Result};
use iracing_telem::{
    flags::Flags, Client, DataUpdateResult, IRSDK_UNLIMITED_LAPS, IRSDK_UNLIMITED_TIME,
};
use log::{debug, error, info};
use std::{collections::HashMap, sync::OnceLock, time::Duration};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu};
use tauri_plugin_log::LogTarget;
use yaml_rust2::YamlLoader;

static WINDOW: OnceLock<tauri::Window> = OnceLock::new();
const SLOW_VAR_RESET_TICKS: u32 = 50;

struct TelemetryData {
    active: bool,
    session_time: Duration,
    player_car_id: u32,
    player_car_class: u32,
    session_flags: Flags,
    player_flags: Flags,
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
    gear_blink_rpm: u32,
    session_info_update: i32,
    drivers: HashMap<u32, Driver>,
}

struct Driver {
    position: u32,
    laps_completed: u32,
    lap_dist_pct: f32,
    total_completed: f32,
    user_name: String,
    car_number: String,
    car_class_id: u32,
    irating: u32,
}

impl TelemetryData {
    fn new() -> Self {
        Self {
            active: false,
            session_time: Duration::new(0, 0),
            player_car_id: 0,
            player_car_class: 0,
            session_flags: Flags::empty(),
            player_flags: Flags::empty(),
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
            gear_blink_rpm: 0,
            session_info_update: 0,
            drivers: HashMap::new(),
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
                    let session_flags = s
                        .find_var("SessionFlags")
                        .ok_or_eyre("SessionFlags variable not found")?;
                    let lap_current_lap_time = s
                        .find_var("LapCurrentLapTime")
                        .ok_or_eyre("LapCurrentLapTime variable not found")?;
                    let gear = s.find_var("Gear").ok_or_eyre("Gear variable not found")?;
                    let speed = s.find_var("Speed").ok_or_eyre("Speed variable not found")?;
                    let rpm = s.find_var("RPM").ok_or_eyre("RPM variable not found")?;
                    let lap = s.find_var("Lap").ok_or_eyre("Lap variable not found")?;
                    let brake = s.find_var("Brake").ok_or_eyre("Brake variable not found")?;
                    let throttle = s
                        .find_var("Throttle")
                        .ok_or_eyre("Throttle variable not found")?;
                    let session_time_total = s
                        .find_var("SessionTimeTotal")
                        .ok_or_eyre("SessionTimeTotal variable not found")?;
                    let session_laps_total = s
                        .find_var("SessionLapsTotal")
                        .ok_or_eyre("SessionLapsTotal variable not found")?;
                    let player_car_my_incident_count = s
                        .find_var("PlayerCarMyIncidentCount")
                        .ok_or_eyre("PlayerCarMyIncidentCount variable not found")?;
                    let player_car_sl_shift_rpm = s
                        .find_var("PlayerCarSLShiftRPM")
                        .ok_or_eyre("PlayerCarSLShiftRPM variable not found")?;
                    let player_car_sl_blink_rpm = s
                        .find_var("PlayerCarSLBlinkRPM")
                        .ok_or_eyre("PlayerCarSLBlinkRPM variable not found")?;
                    let player_car_idx = s
                        .find_var("PlayerCarIdx")
                        .ok_or_eyre("PlayerCarIdx variable not found")?;
                    let player_car_class = s
                        .find_var("PlayerCarClass")
                        .ok_or_eyre("PlayerCarClass variable not found")?;
                    let car_idx_lap_dist_pct = s
                        .find_var("CarIdxLapDistPct")
                        .ok_or_eyre("CarIdxLapDistPct variable not found")?;
                    let car_idx_lap_completed = s
                        .find_var("CarIdxLapCompleted")
                        .ok_or_eyre("CarIdxLapCompleted variable not found")?;
                    let car_idx_session_flags = s
                        .find_var("CarIdxSessionFlags")
                        .ok_or_eyre("CarIdxSessionFlags variable not found")?;
                    let mut slow_var_ticks: u32 = 0;
                    loop {
                        let window_opt = WINDOW.get();

                        if window_opt.is_none() {
                            error!("Window is none");
                            break;
                        }

                        let window = window_opt.ok_or_eyre("Window is none")?;

                        match s.wait_for_data(Duration::from_millis(25)) {
                            DataUpdateResult::Updated => {
                                slow_var_ticks += 1;

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

                                // player_car_idx
                                let player_car_idx_value: i32 = match s.value(&player_car_idx) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get PlayerCarIdx value: {:?}", err);
                                        continue;
                                    }
                                };
                                data.player_car_id = player_car_idx_value as u32;

                                // player_car_class
                                let player_car_class_value: i32 = match s.value(&player_car_class) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get PlayerCarClass value: {:?}", err);
                                        continue;
                                    }
                                };
                                data.player_car_class = player_car_class_value as u32;

                                // session_flags
                                let session_flags_value: Flags = match s.value(&session_flags) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get SessionFlags value: {:?}", err);
                                        continue;
                                    }
                                };
                                data.session_flags = session_flags_value;

                                // player_flags
                                // let car_idx_session_flags: &[i32] = match s
                                //     .value(&car_idx_session_flags)
                                // {
                                //     Ok(value) => value,
                                //     Err(err) => {
                                //         error!("Failed to get CarIdxSessionFlags value: {:?}", err);
                                //         continue;
                                //     }
                                // };
                                // let player_flags_value =
                                //     car_idx_session_flags[player_car_idx_value as usize];
                                // data.player_flags =
                                //     Flags::from_bits_truncate(player_flags_value as u32);

                                let flags = data.session_flags | data.player_flags;
                                let mut flag_value: String = "".to_string();
                                if flags.contains(Flags::CHECKERED) {
                                    flag_value = "CHECKERED".to_string();
                                } else if flags.contains(Flags::GREEN) {
                                    flag_value = "GREEN".to_string();
                                } else if flags.contains(Flags::YELLOW) {
                                    flag_value = "YELLOW".to_string();
                                } else if flags.contains(Flags::RED) {
                                    flag_value = "RED".to_string();
                                } else if flags.contains(Flags::WHITE) {
                                    flag_value = "WHITE".to_string();
                                } else if flags.contains(Flags::BLUE) {
                                    flag_value = "BLUE".to_string();
                                } else if flags.contains(Flags::BLACK) {
                                    flag_value = "BLACK".to_string();
                                }

                                let _ = window.emit("flag", flag_value);

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

                                let lap_dist_pct: &[f32] = match s.value(&car_idx_lap_dist_pct) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get CarIdxLapDistPct value: {:?}", err);
                                        continue;
                                    }
                                };

                                let lap_completed: &[i32] = match s.value(&car_idx_lap_completed) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get CarIdxLapCompleted value: {:?}", err);
                                        continue;
                                    }
                                };

                                for (car_id, driver) in data.drivers.iter_mut() {
                                    let lap_dist_pct_value = lap_dist_pct[*car_id as usize];
                                    let lap_completed_value =
                                        lap_completed[*car_id as usize] as u32;

                                    driver.lap_dist_pct = lap_dist_pct_value;
                                    driver.laps_completed = lap_completed_value;
                                    driver.total_completed =
                                        lap_completed_value as f32 + lap_dist_pct_value;
                                }

                                if data.drivers.contains_key(&data.player_car_id) {
                                    let player = data.drivers.get(&data.player_car_id).unwrap();
                                    let position =
                                        data.drivers
                                            .iter()
                                            .filter(|(_, driver)| {
                                                driver.total_completed > player.total_completed
                                            })
                                            .count() as u32
                                            + 1;
                                    let _ = window.emit("position", position);
                                    data.position = position;
                                }

                                if slow_var_ticks < SLOW_VAR_RESET_TICKS {
                                    continue;
                                }

                                // slow vars

                                // session_time_total
                                let raw_session_time_total_value: f64 = match s
                                    .value(&session_time_total)
                                {
                                    Ok(value) if value >= IRSDK_UNLIMITED_TIME => 0.0,
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
                                    Ok(value) if value >= IRSDK_UNLIMITED_LAPS => 0,
                                    Ok(value) => value,
                                    Err(err) => {
                                        error!("Failed to get SessionLapsTotal value: {:?}", err);
                                        continue;
                                    }
                                };
                                let laps_total_value = raw_session_laps_total_value as u32;
                                let _ = window.emit("laps_total", laps_total_value);
                                data.laps_total = laps_total_value;

                                // incidents
                                let raw_incidents_value: i32 =
                                    match s.value(&player_car_my_incident_count) {
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

                        let session_info_update = s.session_info_update();
                        if data.session_info_update != session_info_update {
                            debug!("Session info updated");
                            let session_info = YamlLoader::load_from_str(&s.session_info())?;
                            let session = &session_info[0];

                            // incident_limit
                            let incident_limit =
                                &session["WeekendInfo"]["WeekendOptions"]["IncidentLimit"];
                            let incident_limit_value = match incident_limit.as_str() {
                                Some(_) => 0u32,
                                None => match incident_limit.as_i64() {
                                    Some(value) => value as u32,
                                    None => 0u32,
                                },
                            };
                            let _ = window.emit("incident_limit", incident_limit_value);
                            data.incident_limit = incident_limit_value;

                            let drivers = &session["DriverInfo"]["Drivers"].as_vec();
                            if drivers.is_none() {
                                error!("No drivers found");
                                continue;
                            }
                            let drivers = drivers.unwrap();
                            for driver in drivers {
                                let car_id = driver["CarIdx"].as_i64().unwrap() as u32;
                                let user_name = driver["UserName"].as_str().unwrap().to_string();
                                let car_number = driver["CarNumber"].as_str().unwrap().to_string();
                                let car_class_id = driver["CarClassID"].as_i64().unwrap() as u32;
                                let irating = driver["IRating"].as_i64().unwrap() as u32;

                                if data.drivers.contains_key(&car_id) {
                                    continue;
                                }

                                if car_class_id != data.player_car_class {
                                    continue;
                                }

                                let driver = Driver {
                                    position: 0,
                                    laps_completed: 0,
                                    lap_dist_pct: 0.0,
                                    total_completed: 0.0,
                                    user_name,
                                    car_number,
                                    car_class_id,
                                    irating,
                                };

                                data.drivers.insert(car_id, driver);
                            }

                            if !data.drivers.is_empty() {
                                let positions_total = data.drivers.len() as u32;
                                let _ = window.emit("positions_total", positions_total);
                                data.positions_total = positions_total;
                            }

                            data.session_info_update = session_info_update;
                        }
                    }
                }
            }
        }
    }
}
