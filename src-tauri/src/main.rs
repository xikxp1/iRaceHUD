// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod driver;
pub mod signed_duration;
pub mod telemetry;
pub mod telemetry_data;
pub mod telemetry_emitter;

use crate::driver::Driver;
use crate::signed_duration::SignedDuration;
use crate::telemetry_data::{LapTime, TelemetryData};
use crate::telemetry_emitter::TelemetryEmitter;
use chrono::Local;
use eyre::{eyre, OptionExt, Result};
use log::{debug, error, info};
use serde_json::json;
use simetry::iracing::{Client, UNLIMITED_LAPS, UNLIMITED_TIME};
use std::{f32::consts::LN_2, sync::OnceLock, time::Duration};
use tauri::ipc::Channel;
use tauri::{
    async_runtime,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};
use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_log::{Target, TargetKind};
use tokio::sync::Mutex;

static BR1: f32 = 1600. / LN_2;
static WINDOW: OnceLock<tauri::WebviewWindow> = OnceLock::new();
const WAIT_FOR_SESSION_SECS: u64 = 600;
const SESSION_UPDATE_PERIOD_MILLIS: u64 = 25;
const SLOW_VAR_RESET_TICKS: u32 = 50;
const MAX_LAP_TIMES: usize = 5;

#[tokio::main]
async fn main() {
    let _ = color_eyre::install();

    #[cfg(debug_assertions)]
    specta::export::ts("../src/lib/types/telemetry.ts").expect("Failed to export types");

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, args, cwd| {
            info!(
                "Single instance started with args: {:?}, cwd: {:?}",
                args, cwd
            );
        }))
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .target(Target::new(TargetKind::LogDir { file_name: None }))
                .target(Target::new(TargetKind::Stdout))
                .target(Target::new(TargetKind::Webview))
                .build(),
        )
        .setup(|app| {
            #[cfg(not(debug_assertions))]
            {
                use tauri_plugin_updater::UpdaterExt;

                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    if let Err(err) = update(handle).await {
                        error!("Failed to update: {:?}", err);
                    }
                });
            }

            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let settings = MenuItemBuilder::with_id("settings", "Settings").build(app)?;
            let tray_menu = MenuBuilder::new(app)
                .item(&settings)
                .separator()
                .item(&quit)
                .build()?;
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_menu_event({
                    let app_handle = app.handle().clone();
                    move |_, event| {
                        if event.id().as_ref() == "quit" {
                            info!("Quit menu item clicked, quitting application");
                            app_handle.exit(0);
                        } else if event.id().as_ref() == "settings" {
                            info!("Settings menu item clicked, opening settings");
                            WebviewWindowBuilder::new(
                                &app_handle,
                                "settings",
                                WebviewUrl::App("/settings".into()),
                            )
                            .title("iRaceHUD Settings")
                            .resizable(false)
                            .center()
                            .build()
                            .expect("Failed to open settings");
                        }
                    }
                })
                .title("iRaceHUD")
                .build(app)?;

            app.manage(Mutex::new(TelemetryEmitter::default()));

            let window = app
                .get_webview_window("main")
                .ok_or_eyre("Failed to get window")?;

            #[cfg(debug_assertions)]
            window.open_devtools();

            window
                .set_ignore_cursor_events(true)
                .map_err(|err| eyre!("Failed to set ignore cursor events: {:?}", err))?;

            WINDOW
                .set(window)
                .map_err(|err| eyre!("Failed to set window: {:?}", err))?;

            async_runtime::spawn(async move {
                if let Err(err) = connect().await {
                    error!("Failed to connect: {:?}", err);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register_event_emitter,
            unregister_event_emitter,
            set_autostart,
            get_autostart,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

async fn connect() -> Result<()> {
    loop {
        info!("Start iRacing");
        let mut client = Client::connect(Duration::from_secs(WAIT_FOR_SESSION_SECS)).await;
        let mut data = TelemetryData::default();
        let mut slow_var_ticks: u32 = SLOW_VAR_RESET_TICKS;
        while let Some(sim_state) = client.next_sim_state().await {
            let window = WINDOW.get().ok_or_eyre("Failed to get window")?;
            let emitter_state = window.app_handle().state::<Mutex<TelemetryEmitter>>();
            let mut emitter = emitter_state.lock().await;

            slow_var_ticks += 1;

            let current_time = Local::now();

            let session_tick = sim_state.read_name("SessionTick").unwrap_or(0);
            if session_tick == 0 {
                debug!("Session tick is 0, skipping");
                continue;
            }

            // active
            let raw_is_on_track_value = sim_state.read_name("IsOnTrack").unwrap_or(false);
            let raw_is_on_track_car_value = sim_state.read_name("IsOnTrackCar").unwrap_or(false);

            let active = raw_is_on_track_value && raw_is_on_track_car_value;
            emitter.emit("active", json!(telemetry::Active::new(active)))?;

            if active != data.active {
                info!(
                    "Session state changed to {}",
                    if active { "active" } else { "inactive" }
                );
                if active {
                    emitter.activate(current_time);
                    // TODO: properly clear session state
                    data.player_lap_times.clear();
                    data.last_lap_time = SignedDuration::ZERO;
                } else {
                    emitter.deactivate();
                }
                data.active = active;
            }

            if !active {
                continue;
            }

            // slow vars
            if slow_var_ticks >= SLOW_VAR_RESET_TICKS {
                // session_time_total
                let raw_session_time_total_value = match sim_state.read_name("SessionTimeTotal") {
                    Some(value) if value >= UNLIMITED_TIME => 0.0,
                    Some(value) => value,
                    None => 0.0,
                };
                let session_time_total_value =
                    Duration::from_secs_f64(raw_session_time_total_value);
                emitter.emit(
                    "session_time_total",
                    json!(telemetry::SessionTimeTotal::new(session_time_total_value)),
                )?;
                data.session_time_total = session_time_total_value;

                // session_laps_total
                let raw_session_laps_total_value = match sim_state.read_name("SessionLapsTotal") {
                    Some(value) if value >= UNLIMITED_LAPS => 0,
                    Some(value) => value,
                    None => 0,
                };
                let laps_total_value = raw_session_laps_total_value as u32;
                emitter.emit(
                    "laps_total",
                    json!(telemetry::LapsTotal::new(laps_total_value)),
                )?;
                data.laps_total = laps_total_value;

                // incidents
                let raw_incidents_value =
                    sim_state.read_name("PlayerCarMyIncidentCount").unwrap_or(0);
                let incidents_value = raw_incidents_value as u32;
                emitter.emit(
                    "incidents",
                    json!(telemetry::Incidents::new(incidents_value)),
                )?;
                data.incidents = incidents_value;

                // gear_shift_rpm
                let raw_player_car_sl_shift_rpm_value: f32 =
                    sim_state.read_name("PlayerCarSLShiftRPM").unwrap_or(0.0);
                let gear_shift_rpm_value = raw_player_car_sl_shift_rpm_value.round() as u32;
                emitter.emit(
                    "gear_shift_rpm",
                    json!(telemetry::GearRPM::new(gear_shift_rpm_value)),
                )?;
                data.gear_shift_rpm = gear_shift_rpm_value;

                // gear_blink_rpm
                let raw_player_car_sl_blink_rpm_value: f32 =
                    sim_state.read_name("PlayerCarSLBlinkRPM").unwrap_or(0.0);
                let gear_blink_rpm_value = raw_player_car_sl_blink_rpm_value.round() as u32;
                emitter.emit(
                    "gear_blink_rpm",
                    json!(telemetry::GearRPM::new(gear_blink_rpm_value)),
                )?;
                data.gear_blink_rpm = gear_blink_rpm_value;
            }

            // current_time
            let current_time_value = current_time.format("%H:%M");
            emitter.emit(
                "current_time",
                json!(telemetry::CurrentTime::new(current_time_value.to_string())),
            )?;

            // session_time
            let raw_session_time_value = sim_state.read_name("SessionTime").unwrap_or(0.0);
            let session_time_value = Duration::from_secs_f64(raw_session_time_value);
            emitter.emit(
                "session_time",
                json!(telemetry::SessionTime::new(session_time_value)),
            )?;
            data.session_time = session_time_value;

            // player_car_idx
            let player_car_idx_value = sim_state.read_name("PlayerCarIdx").unwrap_or(0);
            data.player_car_id = player_car_idx_value as u32;

            // player_car_class
            let player_car_class_value = sim_state.read_name("PlayerCarClass").unwrap_or(0);
            data.player_car_class = player_car_class_value as u32;

            // lap
            let raw_lap_value = sim_state.read_name("Lap").unwrap_or(0);
            let lap_value = raw_lap_value as u32;
            emitter.emit("lap", json!(telemetry::Laps::new(lap_value)))?;
            data.lap = lap_value;

            //race_laps
            let raw_race_laps_value = sim_state.read_name("RaceLaps").unwrap_or(0);
            let race_laps_value = raw_race_laps_value as u32;
            emitter.emit("race_laps", json!(telemetry::Laps::new(race_laps_value)))?;
            data.race_laps = race_laps_value;

            // lap_time
            let raw_lap_current_lap_time_value =
                sim_state.read_name("LapCurrentLapTime").unwrap_or(0.0);
            let lap_time_value = Duration::from_secs_f32(raw_lap_current_lap_time_value);
            emitter.emit("lap_time", json!(telemetry::LapTime::new(lap_time_value)))?;
            data.lap_time = lap_time_value;

            // delta_last_time
            let raw_lap_delta_to_session_last_lap_value = sim_state
                .read_name("LapDeltaToSessionLastlLap")
                .unwrap_or(0.0);
            let delta_last_time_value =
                SignedDuration::from_secs_f32(raw_lap_delta_to_session_last_lap_value);
            emitter.emit(
                "delta_last_time",
                json!(telemetry::DeltaTime::new(delta_last_time_value)),
            )?;
            data.delta_last_time = delta_last_time_value;

            // delta_optimal_time
            let raw_lap_delta_to_optimal_lap_value =
                sim_state.read_name("LapDeltaToOptimalLap").unwrap_or(0.0);
            let delta_optimal_time_value =
                SignedDuration::from_secs_f32(raw_lap_delta_to_optimal_lap_value);
            emitter.emit(
                "delta_optimal_time",
                json!(telemetry::DeltaTime::new(delta_optimal_time_value)),
            )?;
            data.delta_optimal_time = delta_optimal_time_value;

            // session_state
            let raw_session_time_remain_value =
                sim_state.read_name("SessionTimeRemain").unwrap_or(0.0);
            let raw_session_laps_remain_ex_value =
                sim_state.read_name("SessionLapsRemainEx").unwrap_or(0);
            emitter.emit(
                "session_state",
                json!(telemetry::SessionState::new(
                    data.laps_total,
                    raw_session_laps_remain_ex_value,
                    raw_session_time_remain_value
                )),
            )?;

            // gear
            let raw_gear_value = sim_state.read_name("Gear").unwrap_or(0);
            emitter.emit("gear", json!(telemetry::Gear::new(raw_gear_value)))?;
            data.gear = raw_gear_value;

            // speed
            let raw_speed_value: f32 = sim_state.read_name("Speed").unwrap_or(0.0);
            let speed_value = (raw_speed_value * 3.6).round() as u32;
            emitter.emit("speed", json!(telemetry::Speed::new(speed_value)))?;
            data.speed = speed_value;

            // rpm
            let raw_rpm_value: f32 = sim_state.read_name("RPM").unwrap_or(0.0);
            let rpm_value = raw_rpm_value.round() as u32;
            emitter.emit("rpm", json!(telemetry::RPM::new(rpm_value)))?;
            data.rpm = rpm_value;

            // telemetry (brake+throttle)
            let raw_brake_value: f32 = sim_state.read_name("Brake").unwrap_or(0.0);
            let brake_value = (raw_brake_value * 100.0).round() as u32;
            let raw_throttle_value: f32 = sim_state.read_name("Throttle").unwrap_or(0.0);
            let throttle_value = (raw_throttle_value * 100.0).round() as u32;
            let abs_active_value = sim_state.read_name("BrakeABSactive").unwrap_or(false);
            emitter.emit(
                "telemetry",
                json!(telemetry::Telemetry::new(
                    session_time_value.as_secs_f64(),
                    throttle_value,
                    brake_value,
                    abs_active_value
                )),
            )?;
            data.brake = brake_value;
            data.throttle = throttle_value;

            // proximity
            // TODO: bitfield parsing doesn't work
            let raw_car_left_right_value = sim_state.read_name("CarLeftRight").unwrap_or(0);
            let is_left = raw_car_left_right_value == 2
                || raw_car_left_right_value == 4
                || raw_car_left_right_value == 5;
            let is_right = raw_car_left_right_value == 3
                || raw_car_left_right_value == 4
                || raw_car_left_right_value == 6;
            emitter.emit(
                "proximity",
                json!(telemetry::Proximity::new(is_left, is_right)),
            )?;
            data.is_left = is_left;
            data.is_right = is_right;

            // positions+distance
            let lap_dist_pct = sim_state.read_name("CarIdxLapDistPct").unwrap_or(vec![]);

            let laps_completed = sim_state.read_name("CarIdxLapCompleted").unwrap_or(vec![]);

            let laps_started = sim_state.read_name("CarIdxLap").unwrap_or(vec![]);

            let car_idx_est_time_value = sim_state.read_name("CarIdxEstTime").unwrap_or(vec![]);

            let car_idx_best_lap_time_value =
                sim_state.read_name("CarIdxBestLapTime").unwrap_or(vec![]);

            let car_idx_last_lap_time_value =
                sim_state.read_name("CarIdxLastLapTime").unwrap_or(vec![]);

            let car_idx_track_surface_value: Vec<i32> =
                sim_state.read_name("CarIdxTrackSurface").unwrap_or(vec![]);

            for (car_id, driver) in data.drivers.iter_mut() {
                let lap_dist_pct_value = lap_dist_pct[*car_id as usize];
                let mut laps_completed_value = match laps_completed[*car_id as usize] {
                    value if value >= UNLIMITED_LAPS || value <= 0 => 0,
                    value => value,
                } as u32;
                let laps_started_value = match laps_started[*car_id as usize] {
                    value if value >= UNLIMITED_LAPS || value <= 0 => 0,
                    value => value,
                } as u32;
                let est_time_value = car_idx_est_time_value[*car_id as usize];
                if laps_started_value == 0 {
                    laps_completed_value = 0;
                }
                driver.lap_dist_pct = lap_dist_pct_value;
                driver.laps_completed = laps_completed_value;
                driver.total_completed = laps_completed_value as f32 + lap_dist_pct_value;
                driver.estimated = SignedDuration::from_secs_f32(est_time_value);
                driver.best_lap_time =
                    SignedDuration::from_secs_f32(car_idx_best_lap_time_value[*car_id as usize]);
                driver.last_lap_time =
                    SignedDuration::from_secs_f32(car_idx_last_lap_time_value[*car_id as usize]);
                driver.is_in_pits = car_idx_track_surface_value[*car_id as usize] == 1
                    || car_idx_track_surface_value[*car_id as usize] == 2;
                driver.is_off_track = car_idx_track_surface_value[*car_id as usize] == 0;
                driver.is_off_world = car_idx_track_surface_value[*car_id as usize] == -1;
            }

            let mut driver_positions = data
                .drivers
                .iter()
                .map(|(car_id, driver)| (*car_id, driver.total_completed))
                .collect::<Vec<(u32, f32)>>();
            driver_positions.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            data.driver_positions = driver_positions
                .iter()
                .rev()
                .map(|(car_id, _)| *car_id)
                .collect();

            for (position, car_id) in data.driver_positions.iter().enumerate() {
                let driver = data
                    .drivers
                    .get_mut(car_id)
                    .ok_or_eyre("Driver not found while updating position")?;
                driver.position = position as u32 + 1;
                if *car_id == data.player_car_id {
                    emitter.emit("position", json!(telemetry::Position::new(driver.position)))?;
                    data.position = driver.position;
                }
                if position == 0 {
                    data.leader_car_id = *car_id;
                }
            }

            // gaps
            if !data.driver_positions.is_empty() {
                let player = data
                    .drivers
                    .get(&data.player_car_id)
                    .ok_or_eyre("Player not found")?
                    .to_owned();
                let leader = data
                    .drivers
                    .get(&data.leader_car_id)
                    .ok_or_eyre("Leader not found")?
                    .to_owned();

                for driver in data.drivers.values_mut() {
                    driver.is_leader = driver.car_id == data.leader_car_id;
                    driver.is_player = driver.car_id == data.player_car_id;
                    let leader_gap_laps = leader.total_completed - driver.total_completed;
                    if leader_gap_laps >= 1.0 {
                        driver.leader_gap_laps = leader_gap_laps as i32;
                        driver.leader_gap = SignedDuration::ZERO;
                    } else {
                        driver.leader_gap_laps = 0;
                        driver.leader_gap = match leader.estimated - driver.estimated {
                            value if value >= SignedDuration::ZERO => {
                                leader.estimated - driver.estimated
                            }
                            _ => leader.estimated + data.car_class_est_lap_time - driver.estimated,
                        };
                    }
                    let player_gap_laps = player.total_completed - driver.total_completed;
                    if player_gap_laps >= 1.0 || player_gap_laps <= -1.0 {
                        driver.player_gap_laps = player_gap_laps as i32;
                        driver.player_gap = SignedDuration::ZERO;
                    } else {
                        driver.player_gap_laps = 0;
                        driver.player_gap = match player_gap_laps {
                            value if value >= 0.0 => match player.estimated - driver.estimated {
                                value if value >= SignedDuration::ZERO => {
                                    player.estimated - driver.estimated
                                }
                                _ => {
                                    player.estimated + data.car_class_est_lap_time
                                        - driver.estimated
                                }
                            },
                            _ => match player.estimated - driver.estimated {
                                value if value >= SignedDuration::ZERO => {
                                    driver.estimated + data.car_class_est_lap_time
                                        - player.estimated
                                }
                                _ => player.estimated - driver.estimated,
                            },
                        };
                    };
                    driver.player_relative_gap = match player.estimated - driver.estimated {
                        value if value >= data.car_class_est_lap_time * 0.5 => {
                            value - data.car_class_est_lap_time
                        }
                        value if value <= -data.car_class_est_lap_time * 0.5 => {
                            value + data.car_class_est_lap_time
                        }
                        value => value,
                    };
                }

                let player_position = player.position;
                emitter.emit(
                    "gap_next",
                    json!(telemetry::Gap::new(
                        player_position - 1,
                        &data.driver_positions,
                        &data.drivers,
                        false
                    )),
                )?;
                emitter.emit(
                    "gap_prev",
                    json!(telemetry::Gap::new(
                        player_position + 1,
                        &data.driver_positions,
                        &data.drivers,
                        false
                    )),
                )?;
            }

            // track_map
            emitter.emit("track_map", json!(telemetry::TrackMap::new(&data.drivers)))?;

            // extra slow vars
            if slow_var_ticks >= SLOW_VAR_RESET_TICKS {
                // standings
                if !data.drivers.is_empty() {
                    let standings =
                        telemetry::Standings::new(&data.driver_positions, &data.drivers);
                    emitter.emit("standings", json!(standings))?;
                }

                // relative
                if !data.drivers.is_empty() {
                    let relative = telemetry::Relative::new(&data.driver_positions, &data.drivers);
                    emitter.emit("relative", json!(relative))?;
                }

                // player_lap_times
                if data.lap > 1 {
                    let raw_lap_last_lap_time_value =
                        sim_state.read_name("LapLastLapTime").unwrap_or(0.0);
                    let lap_last_lap_time_value =
                        SignedDuration::from_secs_f32(raw_lap_last_lap_time_value);
                    match lap_last_lap_time_value {
                        value if value.is_positive() => {
                            // This check is not exactly safe
                            if lap_last_lap_time_value != data.last_lap_time {
                                data.player_lap_times
                                    .insert(0, LapTime::new(data.lap - 1, value));
                                if data.player_lap_times.len() > MAX_LAP_TIMES {
                                    data.player_lap_times.pop();
                                }
                                let player_lap_times_value =
                                    telemetry::LapTimes::new(&data.player_lap_times);
                                emitter.emit("player_lap_times", json!(player_lap_times_value))?;
                                data.last_lap_time = value;
                            }
                        }
                        _ => {}
                    }
                }
            }

            let session_info_update = sim_state.header().session_info_update;
            if data.session_info_update != session_info_update {
                debug!("Session info updated");
                let session = sim_state.session_info();

                // incident_limit
                let incident_limit = &session["WeekendInfo"]["WeekendOptions"]["IncidentLimit"];
                let incident_limit_value = match incident_limit.as_str() {
                    Some(_) => 0u32,
                    None => match incident_limit.as_i64() {
                        Some(value) => value as u32,
                        None => 0u32,
                    },
                };
                emitter.emit(
                    "incident_limit",
                    json!(telemetry::Incidents::new(incident_limit_value)),
                )?;
                data.incident_limit = incident_limit_value;

                // track_id
                let track_id = session["WeekendInfo"]["TrackID"].as_i64().unwrap_or(0) as u32;
                emitter.emit("track_id", json!(telemetry::TrackID::new(track_id)))?;
                data.track_id = track_id;

                let drivers = session["DriverInfo"]["Drivers"].as_vec();

                match drivers {
                    Some(drivers) => {
                        for driver in drivers {
                            let car_id =
                                driver["CarIdx"].as_i64().ok_or_eyre("CarIdx not found")? as u32;
                            let user_name = driver["UserName"]
                                .as_str()
                                .ok_or_eyre("UserName not found")?
                                .to_string();
                            let car_number = driver["CarNumber"]
                                .as_str()
                                .ok_or_eyre("CarNumber not found")?
                                .to_string();
                            let car_class_id = driver["CarClassID"]
                                .as_i64()
                                .ok_or_eyre("CarClassID not found")?
                                as u32;
                            let irating =
                                driver["IRating"].as_i64().ok_or_eyre("IRating not found")? as u32;
                            let lic_string = driver["LicString"]
                                .as_str()
                                .ok_or_eyre("LicString not found")?;

                            if data.drivers.contains_key(&car_id) {
                                continue;
                            }

                            if car_class_id != data.player_car_class {
                                continue;
                            }

                            if car_id == data.player_car_id {
                                let car_class_est_lap_time = driver["CarClassEstLapTime"]
                                    .as_f64()
                                    .ok_or_eyre("CarClassEstLapTime not found")?;
                                data.car_class_est_lap_time =
                                    SignedDuration::from_secs_f64(car_class_est_lap_time);
                            }

                            let driver = Driver::new(
                                car_id,
                                user_name,
                                car_number,
                                car_class_id,
                                irating,
                                lic_string.to_string(),
                            );

                            data.drivers.insert(car_id, driver);
                        }
                    }
                    None => {
                        error!("No drivers found");
                        continue;
                    }
                }

                // positions_total
                if !data.drivers.is_empty() {
                    let positions_total = data.drivers.len() as u32;
                    emitter.emit(
                        "positions_total",
                        json!(telemetry::Position::new(positions_total)),
                    )?;
                    data.positions_total = positions_total;

                    // strength_of_field
                    let sum_of_exp = data
                        .drivers
                        .values()
                        .map(|driver| (-(driver.irating as f32) / BR1).exp())
                        .sum::<f32>();
                    let strength_of_field =
                        (BR1 * (positions_total as f32 / sum_of_exp).ln()) as u32;
                    emitter.emit(
                        "strength_of_field",
                        json!(telemetry::StrengthOfField::new(strength_of_field)),
                    )?;
                    data.strength_of_field = strength_of_field;
                }

                data.session_info_update = session_info_update;
            }

            if slow_var_ticks >= SLOW_VAR_RESET_TICKS {
                slow_var_ticks = 0;
            }

            tokio::time::sleep(Duration::from_millis(SESSION_UPDATE_PERIOD_MILLIS)).await;
        }
    }
}

#[cfg(not(debug_assertions))]
async fn update(app: tauri::AppHandle) -> Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        info!("Update found: {:?}", update.version);

        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    info!("Downloaded {downloaded} from {content_length:?}");
                },
                || {
                    info!("Download finished");
                },
            )
            .await?;

        info!("Update installed, restarting app");
        app.restart();
    }

    Ok(())
}

#[tauri::command]
async fn register_event_emitter(app: tauri::AppHandle, event: String, on_event: Channel) {
    debug!("Registering event emitter for {}", event);
    let emitter_state = app.app_handle().state::<Mutex<TelemetryEmitter>>();
    let mut emitter = emitter_state.lock().await;
    emitter.register(event, on_event);
}

#[tauri::command]
async fn unregister_event_emitter(app: tauri::AppHandle, event: String) {
    let emitter_state = app.app_handle().state::<Mutex<TelemetryEmitter>>();
    let mut emitter = emitter_state.lock().await;
    emitter.unregister(&event);
}

#[tauri::command]
async fn set_autostart(app: tauri::AppHandle, enabled: bool) {
    info!("Setting autostart to {}", enabled);
    let autostart_manager = app.autolaunch();
    if enabled {
        autostart_manager
            .enable()
            .expect("Failed to enable autostart");
    } else {
        autostart_manager
            .disable()
            .expect("Failed to disable autostart");
    }
}

#[tauri::command]
async fn get_autostart(app: tauri::AppHandle) -> bool {
    info!("Getting autostart status");
    app.autolaunch().is_enabled().unwrap_or(false)
}
