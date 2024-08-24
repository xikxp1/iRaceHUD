// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Local, TimeDelta};
use eyre::{eyre, OptionExt, Result};
use iracing_telem::{Client, DataUpdateResult, IRSDK_UNLIMITED_LAPS, IRSDK_UNLIMITED_TIME};
use log::{debug, error, info};
use serde_json::{json, Value};
use std::{cmp::min, collections::HashMap, f32::consts::LN_2, sync::OnceLock, time::Duration};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu};
use tauri_plugin_log::LogTarget;
use yaml_rust2::YamlLoader;

static BR1: f32 = 1600. / LN_2;
static WINDOW: OnceLock<tauri::Window> = OnceLock::new();
const WAIT_FOR_SESSION_SECS: u64 = 600;
const SESSION_UPDATE_PERIOD_MILLIS: u64 = 25;
const SLOW_VAR_RESET_TICKS: u32 = 50;
const FORCED_EMITTER_DURATION_SECS: i64 = 10;

struct TelemetryData {
    active: bool,
    session_time: Duration,
    player_car_id: u32,
    player_car_class: u32,
    lap: u32,
    race_laps: u32,
    lap_time: Duration,
    delta_last_time: SignedDuration,
    delta_optimal_time: SignedDuration,
    gear: String,
    speed: u32,
    rpm: u32,
    brake: u32,
    throttle: u32,
    position: u32,
    positions_total: u32,
    strength_of_field: u32,
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

#[derive(Debug)]
struct Emitter {
    events: HashMap<String, Value>,
    activation_time: Option<DateTime<Local>>,
    forced_emitter_duration: TimeDelta,
}

struct SignedDuration {
    is_positive: bool,
    duration: Duration,
}

impl TelemetryData {
    fn new() -> Self {
        Self {
            active: false,
            session_time: Duration::ZERO,
            player_car_id: 0,
            player_car_class: 0,
            lap: 0,
            race_laps: 0,
            lap_time: Duration::ZERO,
            delta_last_time: SignedDuration::ZERO,
            delta_optimal_time: SignedDuration::ZERO,
            gear: String::from("N"),
            speed: 0,
            rpm: 0,
            brake: 0,
            throttle: 0,
            position: 0,
            positions_total: 0,
            strength_of_field: 0,
            session_time_total: Duration::ZERO,
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

impl Emitter {
    fn new(forced_emitter_duration: TimeDelta) -> Self {
        Self {
            events: HashMap::new(),
            activation_time: None,
            forced_emitter_duration,
        }
    }

    fn activate(&mut self, activation_time: DateTime<Local>) {
        self.activation_time = Some(activation_time);
    }

    fn deactivate(&mut self) {
        self.activation_time = None;
    }

    fn emit(&mut self, event: &str, value: Value) -> Result<()> {
        if event != "active" && self.activation_time.is_none() {
            error!("Emitter is not activated");
            return Ok(());
        }

        let mut is_forced = false;

        if let Some(activation_time) = self.activation_time {
            let current_time = Local::now();
            let elapsed = current_time.signed_duration_since(activation_time);
            if elapsed < self.forced_emitter_duration {
                is_forced = true;
            }
        }

        if !is_forced && self.events.contains_key(event) && self.events[event] == value {
            return Ok(());
        }

        let window = WINDOW.get().ok_or_eyre("Failed to get window")?;

        match window.emit(event, value.clone()) {
            Ok(_) => {}
            Err(err) => error!("Failed to emit event {}: {:?}", event, err),
        }
        self.events.insert(event.to_string(), value);
        Ok(())
    }
}

impl SignedDuration {
    const ZERO: SignedDuration = SignedDuration {
        is_positive: true,
        duration: Duration::ZERO,
    };

    fn from_secs_f64(secs: f64) -> Self {
        Self {
            is_positive: secs >= 0.0,
            duration: Duration::from_secs_f64(secs.abs()),
        }
    }

    fn from_secs_f32(secs: f32) -> Self {
        Self {
            is_positive: secs >= 0.0,
            duration: Duration::from_secs_f32(secs.abs()),
        }
    }

    fn as_secs_f32(&self) -> f32 {
        if self.is_positive {
            self.duration.as_secs_f32()
        } else {
            -self.duration.as_secs_f32()
        }
    }

    fn as_secs_f64(&self) -> f64 {
        if self.is_positive {
            self.duration.as_secs_f64()
        } else {
            -self.duration.as_secs_f64()
        }
    }
}

impl std::fmt::Debug for SignedDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}.{:03}",
            if self.is_positive { "+" } else { "-" },
            self.duration.as_secs(),
            self.duration.subsec_millis()
        )
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
            let window = app.get_window("main").ok_or_eyre("Failed to get window")?;

            window
                .set_ignore_cursor_events(true)
                .map_err(|err| eyre!("Failed to set ignore cursor events: {:?}", err))?;

            WINDOW
                .set(window)
                .map_err(|err| eyre!("Failed to set window: {:?}", err))?;

            let emitter = Emitter::new(TimeDelta::seconds(FORCED_EMITTER_DURATION_SECS));

            tauri::async_runtime::spawn(async move {
                connect(emitter).expect("Error while connecting to iRacing");
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

fn connect(mut emitter: Emitter) -> Result<()> {
    let mut c = Client::new();
    loop {
        info!("Start iRacing");
        unsafe {
            match c.wait_for_session(Duration::from_secs(WAIT_FOR_SESSION_SECS)) {
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
                    let session_time_remain = s
                        .find_var("SessionTimeRemain")
                        .ok_or_eyre("SessionTimeRemain variable not found")?;
                    let session_laps_remain_ex = s
                        .find_var("SessionLapsRemainEx")
                        .ok_or_eyre("SessionLapsRemainEx variable not found")?;
                    let lap_current_lap_time = s
                        .find_var("LapCurrentLapTime")
                        .ok_or_eyre("LapCurrentLapTime variable not found")?;
                    let lap_delta_to_optimal_lap = s
                        .find_var("LapDeltaToOptimalLap")
                        .ok_or_eyre("LapDeltaToSessionOptimalLap variable not found")?;
                    let lap_delta_to_session_last_lap = s
                        .find_var("LapDeltaToSessionLastlLap")
                        .ok_or_eyre("LapDeltaToSessionLastlLap variable not found")?;
                    let gear = s.find_var("Gear").ok_or_eyre("Gear variable not found")?;
                    let speed = s.find_var("Speed").ok_or_eyre("Speed variable not found")?;
                    let rpm = s.find_var("RPM").ok_or_eyre("RPM variable not found")?;
                    let lap = s.find_var("Lap").ok_or_eyre("Lap variable not found")?;
                    let race_laps = s
                        .find_var("RaceLaps")
                        .ok_or_eyre("RaceLaps variable not found")?;
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
                    let car_idx_lap = s
                        .find_var("CarIdxLap")
                        .ok_or_eyre("CarIdxLap variable not found")?;
                    let mut slow_var_ticks: u32 = SLOW_VAR_RESET_TICKS;
                    loop {
                        match s.wait_for_data(Duration::from_millis(SESSION_UPDATE_PERIOD_MILLIS)) {
                            DataUpdateResult::Updated => {
                                slow_var_ticks += 1;

                                let current_time = Local::now();

                                // active
                                let raw_is_on_track_value =
                                    s.var_value(&is_on_track).as_bool().map_err(|err| {
                                        eyre!("Failed to get IsOnTrack value: {:?}", err)
                                    })?;
                                let raw_is_on_track_car_value =
                                    s.var_value(&is_on_track_car).as_bool().map_err(|err| {
                                        eyre!("Failed to get IsOnTrackCar value: {:?}", err)
                                    })?;

                                let active = raw_is_on_track_value && raw_is_on_track_car_value;
                                emitter.emit("active", json!(active))?;

                                if active != data.active {
                                    info!(
                                        "Session state changed to {}",
                                        if active { "active" } else { "inactive" }
                                    );
                                    if active {
                                        emitter.activate(current_time);
                                    } else {
                                        emitter.deactivate();
                                    }
                                    data.active = active;
                                }

                                if !active {
                                    continue;
                                }

                                // current_time
                                let current_time_value = current_time.format("%H:%M");
                                emitter
                                    .emit("current_time", json!(current_time_value.to_string()))?;

                                // session_time
                                let raw_session_time_value =
                                    s.var_value(&session_time).as_f64().map_err(|err| {
                                        eyre!("Failed to get SessionTime value: {:?}", err)
                                    })?;
                                let session_time_value =
                                    Duration::from_secs_f64(raw_session_time_value);
                                let ss = session_time_value.as_secs();
                                let (hh, ss) = (ss / 3600, ss % 3600);
                                let (mm, ss) = (ss / 60, ss % 60);
                                emitter.emit(
                                    "session_time",
                                    json!(format!("{:0>2}:{:02}:{:02}", hh, mm, ss)),
                                )?;
                                data.session_time = session_time_value;

                                // player_car_idx
                                let player_car_idx_value =
                                    s.var_value(&player_car_idx).as_i32().map_err(|err| {
                                        eyre!("Failed to get PlayerCarIdx value: {:?}", err)
                                    })?;
                                data.player_car_id = player_car_idx_value as u32;

                                // player_car_class
                                let player_car_class_value =
                                    s.var_value(&player_car_class).as_i32().map_err(|err| {
                                        eyre!("Failed to get PlayerCarClass value: {:?}", err)
                                    })?;
                                data.player_car_class = player_car_class_value as u32;

                                // lap
                                let raw_lap_value = s
                                    .var_value(&lap)
                                    .as_i32()
                                    .map_err(|err| eyre!("Failed to get Lap value: {:?}", err))?;
                                let lap_value = raw_lap_value as u32;
                                emitter.emit("lap", json!(lap_value))?;
                                data.lap = lap_value;

                                //race_laps
                                let raw_race_laps_value =
                                    s.var_value(&race_laps).as_i32().map_err(|err| {
                                        eyre!("Failed to get RaceLaps value: {:?}", err)
                                    })?;
                                let race_laps_value = raw_race_laps_value as u32;
                                emitter.emit("race_laps", json!(race_laps_value))?;
                                data.race_laps = race_laps_value;

                                // lap_time
                                let raw_lap_current_lap_time_value =
                                    s.var_value(&lap_current_lap_time).as_f32().map_err(|err| {
                                        eyre!("Failed to get LapCurrentLapTime value: {:?}", err)
                                    })?;
                                let lap_time_value =
                                    Duration::from_secs_f32(raw_lap_current_lap_time_value);
                                emitter.emit(
                                    "lap_time",
                                    json!(format!(
                                        "{}:{:02}.{:03}",
                                        lap_time_value.as_secs() / 60,
                                        lap_time_value.as_secs() % 60,
                                        lap_time_value.subsec_millis()
                                    )),
                                )?;
                                data.lap_time = lap_time_value;

                                // delta_last_time
                                let raw_lap_delta_to_session_last_lap_value = s
                                    .var_value(&lap_delta_to_session_last_lap)
                                    .as_f32()
                                    .map_err(|err| {
                                        eyre!(
                                            "Failed to get LapDeltaToSessionLastlLap value: {:?}",
                                            err
                                        )
                                    })?;
                                let delta_last_time_value = SignedDuration::from_secs_f32(
                                    raw_lap_delta_to_session_last_lap_value,
                                );
                                let delta_last_time_str = match delta_last_time_value.as_secs_f32()
                                {
                                    value if value >= 100.0 => "–".to_string(),
                                    value if value <= -100.0 => "–".to_string(),
                                    value if value >= 10.0 => {
                                        format!(
                                            "+{:02}.{:01}",
                                            value as i32,
                                            min((value.fract().abs() * 10.0).round() as i32, 9)
                                        )
                                    }
                                    value if value <= -10.0 => {
                                        format!(
                                            "-{:02}.{:01}",
                                            value.abs() as i32,
                                            min((value.fract().abs() * 10.0).round() as i32, 9)
                                        )
                                    }
                                    value if value > 0.0 => {
                                        format!(
                                            "+{:01}.{:02}",
                                            value as i32,
                                            min((value.fract().abs() * 100.0).round() as i32, 99)
                                        )
                                    }
                                    value if value < 0.0 => {
                                        format!(
                                            "-{:01}.{:02}",
                                            value.abs() as i32,
                                            min((value.fract().abs() * 100.0).round() as i32, 99)
                                        )
                                    }
                                    _ => "0.00".to_string(),
                                };
                                emitter.emit("delta_last_time", json!(delta_last_time_str))?;
                                data.delta_last_time = delta_last_time_value;

                                // delta_optimal_time
                                let raw_lap_delta_to_optimal_lap_value = s
                                    .var_value(&lap_delta_to_optimal_lap)
                                    .as_f32()
                                    .map_err(|err| {
                                        eyre!("Failed to get LapDeltaToOptimalLap value: {:?}", err)
                                    })?;
                                let delta_optimal_time_value = SignedDuration::from_secs_f32(
                                    raw_lap_delta_to_optimal_lap_value,
                                );
                                let delta_optimal_time_str = match delta_optimal_time_value
                                    .as_secs_f32()
                                {
                                    value if value >= 100.0 => "–".to_string(),
                                    value if value <= -100.0 => "–".to_string(),
                                    value if value >= 10.0 => {
                                        format!(
                                            "+{:02}.{:01}",
                                            value as i32,
                                            min((value.fract().abs() * 10.0).round() as i32, 9)
                                        )
                                    }
                                    value if value <= -10.0 => {
                                        format!(
                                            "-{:02}.{:01}",
                                            value.abs() as i32,
                                            min((value.fract().abs() * 10.0).round() as i32, 9)
                                        )
                                    }
                                    value if value > 0.0 => {
                                        format!(
                                            "+{:01}.{:02}",
                                            value as i32,
                                            min((value.fract().abs() * 100.0).round() as i32, 99)
                                        )
                                    }
                                    value if value < 0.0 => {
                                        format!(
                                            "-{:01}.{:02}",
                                            value.abs() as i32,
                                            min((value.fract().abs() * 100.0).round() as i32, 99)
                                        )
                                    }
                                    _ => "0.00".to_string(),
                                };
                                emitter
                                    .emit("delta_optimal_time", json!(delta_optimal_time_str))?;
                                data.delta_optimal_time = delta_optimal_time_value;

                                // session_state
                                let session_state = match data.laps_total {
                                    0 => {
                                        let raw_session_time_remain_value = s
                                            .var_value(&session_time_remain)
                                            .as_f64()
                                            .map_err(|err| {
                                                eyre!(
                                                    "Failed to get SessionTimeRemain value: {:?}",
                                                    err
                                                )
                                            })?;
                                        if raw_session_time_remain_value <= 0. {
                                            "Last lap".to_string()
                                        } else {
                                            let session_time_remain_value =
                                                Duration::try_from_secs_f64(
                                                    raw_session_time_remain_value,
                                                )?;
                                            let ss = session_time_remain_value.as_secs();
                                            let (hh, ss) = (ss / 3600, ss % 3600);
                                            let (mm, ss) = (ss / 60, ss % 60);
                                            if hh > 0 {
                                                format!("{}:{:02}:{:02} left", hh, mm, ss)
                                            } else {
                                                format!("{:02}:{:02 } left", mm, ss)
                                            }
                                        }
                                    }
                                    _ => {
                                        let raw_session_laps_remain_ex_value = s
                                            .var_value(&session_laps_remain_ex)
                                            .as_i32()
                                            .map_err(|err| {
                                                eyre!(
                                                    "Failed to get SessionLapsRemainEx value: {:?}",
                                                    err
                                                )
                                            })?;

                                        match raw_session_laps_remain_ex_value {
                                            0 => "".to_string(),
                                            1 => "Last lap".to_string(),
                                            _ => {
                                                format!(
                                                    "{} laps left",
                                                    raw_session_laps_remain_ex_value
                                                )
                                            }
                                        }
                                    }
                                };
                                emitter.emit("session_state", json!(session_state))?;

                                // gear
                                let raw_gear_value = s
                                    .var_value(&gear)
                                    .as_i32()
                                    .map_err(|err| eyre!("Failed to get Gear value: {:?}", err))?;
                                let gear_value = match raw_gear_value {
                                    -1 => String::from("R"),
                                    0 => String::from("N"),
                                    _ => raw_gear_value.to_string(),
                                };
                                emitter.emit("gear", json!(gear_value))?;
                                data.gear = gear_value;

                                // speed
                                let raw_speed_value = s
                                    .var_value(&speed)
                                    .as_f32()
                                    .map_err(|err| eyre!("Failed to get Speed value: {:?}", err))?;
                                let speed_value = (raw_speed_value * 3.6).round() as u32;
                                emitter.emit("speed", json!(speed_value))?;
                                data.speed = speed_value;

                                // rpm
                                let raw_rpm_value = s
                                    .var_value(&rpm)
                                    .as_f32()
                                    .map_err(|err| eyre!("Failed to get RPM value: {:?}", err))?;
                                let rpm_value = raw_rpm_value.round() as u32;
                                emitter.emit("rpm", json!(rpm_value))?;
                                data.rpm = rpm_value;

                                // telemetry (brake+throttle)
                                let raw_brake_value = s
                                    .var_value(&brake)
                                    .as_f32()
                                    .map_err(|err| eyre!("Failed to get Brake value: {:?}", err))?;
                                let brake_value = (raw_brake_value * 100.0).round() as u32;
                                let raw_throttle_value =
                                    s.var_value(&throttle).as_f32().map_err(|err| {
                                        eyre!("Failed to get Throttle value: {:?}", err)
                                    })?;
                                let throttle_value = (raw_throttle_value * 100.0).round() as u32;
                                emitter.emit("telemetry", json!({"ts": session_time_value.as_secs_f64(), "brake": brake_value, "throttle": throttle_value}))?;
                                data.brake = brake_value;
                                data.throttle = throttle_value;

                                // positions+distance
                                let lap_dist_pct = s
                                    .var_value(&car_idx_lap_dist_pct)
                                    .as_f32s()
                                    .map_err(|err| {
                                        eyre!("Failed to get CarIdxLapDistPct value: {:?}", err)
                                    })?;

                                let laps_completed = s
                                    .var_value(&car_idx_lap_completed)
                                    .as_i32s()
                                    .map_err(|err| {
                                        eyre!("Failed to get CarIdxLapCompleted value: {:?}", err)
                                    })?;

                                let laps_started =
                                    s.var_value(&car_idx_lap).as_i32s().map_err(|err| {
                                        eyre!("Failed to get CarIdxLap value: {:?}", err)
                                    })?;

                                for (car_id, driver) in data.drivers.iter_mut() {
                                    let lap_dist_pct_value = lap_dist_pct[*car_id as usize];
                                    let laps_completed_value = match laps_completed
                                        [*car_id as usize]
                                    {
                                        value if value >= IRSDK_UNLIMITED_LAPS || value <= 0 => 0,
                                        value => value,
                                    }
                                        as u32;
                                    let laps_started_value = match laps_started[*car_id as usize] {
                                        value if value >= IRSDK_UNLIMITED_LAPS || value <= 0 => 0,
                                        value => value,
                                    }
                                        as u32;
                                    driver.lap_dist_pct = lap_dist_pct_value;
                                    driver.laps_completed = laps_completed_value;
                                    driver.total_completed = laps_completed_value as f32;
                                    if laps_started_value > 0 {
                                        driver.total_completed += lap_dist_pct_value;
                                    }
                                }

                                if data.drivers.contains_key(&data.player_car_id) {
                                    let player = data
                                        .drivers
                                        .get(&data.player_car_id)
                                        .ok_or_eyre("Player not found")?;
                                    let position =
                                        data.drivers
                                            .iter()
                                            .filter(|(_, driver)| {
                                                driver.total_completed > player.total_completed
                                            })
                                            .count() as u32
                                            + 1;
                                    emitter.emit("position", json!(position))?;
                                    data.position = position;
                                }

                                if slow_var_ticks < SLOW_VAR_RESET_TICKS {
                                    continue;
                                }

                                // slow vars

                                // session_time_total
                                let raw_session_time_total_value = s
                                    .var_value(&session_time_total)
                                    .as_f64()
                                    .map(|value| {
                                        if value >= IRSDK_UNLIMITED_TIME {
                                            0.
                                        } else {
                                            value
                                        }
                                    })
                                    .map_err(|err| {
                                        eyre!("Failed to get SessionTimeTotal value: {:?}", err)
                                    })?;
                                let session_time_total_value =
                                    Duration::from_secs_f64(raw_session_time_total_value);
                                emitter.emit(
                                    "session_time_total",
                                    json!(humantime::format_duration(session_time_total_value)
                                        .to_string()),
                                )?;
                                data.session_time_total = session_time_total_value;

                                // session_laps_total
                                let raw_session_laps_total_value: i32 = s
                                    .var_value(&session_laps_total)
                                    .as_i32()
                                    .map(|value| {
                                        if value >= IRSDK_UNLIMITED_LAPS {
                                            0
                                        } else {
                                            value
                                        }
                                    })
                                    .map_err(|err| {
                                        eyre!("Failed to get SessionLapsTotal value: {:?}", err)
                                    })?;
                                let laps_total_value = raw_session_laps_total_value as u32;
                                emitter.emit("laps_total", json!(laps_total_value))?;
                                data.laps_total = laps_total_value;

                                // incidents
                                let raw_incidents_value = s
                                    .var_value(&player_car_my_incident_count)
                                    .as_i32()
                                    .map_err(|err| {
                                        eyre!(
                                            "Failed to get PlayerCarMyIncidentCount value: {:?}",
                                            err
                                        )
                                    })?;
                                let incidents_value = raw_incidents_value as u32;
                                emitter.emit("incidents", json!(incidents_value))?;
                                data.incidents = incidents_value;

                                // gear_shift_rpm
                                let raw_player_car_sl_shift_rpm_value = s
                                    .var_value(&player_car_sl_shift_rpm)
                                    .as_f32()
                                    .map_err(|err| {
                                        eyre!("Failed to get PlayerCarSLShiftRPM value: {:?}", err)
                                    })?;
                                let gear_shift_rpm_value =
                                    raw_player_car_sl_shift_rpm_value.round() as u32;
                                emitter.emit("gear_shift_rpm", json!(gear_shift_rpm_value))?;
                                data.gear_shift_rpm = gear_shift_rpm_value;

                                // gear_blink_rpm
                                let raw_player_car_sl_blink_rpm_value = s
                                    .var_value(&player_car_sl_blink_rpm)
                                    .as_f32()
                                    .map_err(|err| {
                                        eyre!("Failed to get PlayerCarSLBlinkRPM value: {:?}", err)
                                    })?;
                                let gear_blink_rpm_value =
                                    raw_player_car_sl_blink_rpm_value.round() as u32;
                                emitter.emit("gear_blink_rpm", json!(gear_blink_rpm_value))?;
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
                            emitter.emit("incident_limit", json!(incident_limit_value))?;
                            data.incident_limit = incident_limit_value;

                            let drivers = session["DriverInfo"]["Drivers"].as_vec();

                            match drivers {
                                Some(drivers) => {
                                    for driver in drivers {
                                        let car_id = driver["CarIdx"]
                                            .as_i64()
                                            .ok_or_eyre("CarIdx not found")?
                                            as u32;
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
                                        let irating = driver["IRating"]
                                            .as_i64()
                                            .ok_or_eyre("IRating not found")?
                                            as u32;

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
                                }
                                None => {
                                    error!("No drivers found");
                                    continue;
                                }
                            }

                            // positions_total
                            if !data.drivers.is_empty() {
                                let positions_total = data.drivers.len() as u32;
                                emitter.emit("positions_total", json!(positions_total))?;
                                data.positions_total = positions_total;

                                // strength_of_field
                                let sum_of_exp = data
                                    .drivers
                                    .values()
                                    .map(|driver| (-(driver.irating as f32) / BR1).exp())
                                    .sum::<f32>();
                                let strength_of_field =
                                    (BR1 * (positions_total as f32 / sum_of_exp).ln()) as u32;
                                emitter.emit("strength_of_field", json!(strength_of_field))?;
                                data.strength_of_field = strength_of_field;
                            }

                            data.session_info_update = session_info_update;
                        }
                    }
                }
            }
        }
    }
}
