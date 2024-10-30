use std::{cmp::min, collections::HashMap, time::Duration};

use serde::Serialize;
use specta::Type;

const RELATIVE_DRIVERS_BEFORE: usize = 3;
const RELATIVE_DRIVERS_AFTER: usize = 3;

use crate::{telemetry_data, Driver, SignedDuration};

#[derive(Type, Serialize)]
pub struct Active(bool);

#[derive(Type, Serialize)]
pub struct SessionTimeTotal(String);

#[derive(Type, Serialize)]
pub struct LapsTotal(u32);

#[derive(Type, Serialize)]
pub struct Incidents(u32);

#[derive(Type, Serialize)]
pub struct GearRPM(u32);

#[derive(Type, Serialize)]
pub struct CurrentTime(String);

#[derive(Type, Serialize)]
pub struct SessionTime(String);

#[derive(Type, Serialize)]
pub struct Laps(u32);

#[derive(Type, Serialize)]
pub struct LapTime(String);

#[derive(Type, Serialize)]
pub struct DeltaTime(String);

#[derive(Type, Serialize)]
pub struct SessionState(String);

#[derive(Type, Serialize)]
pub struct Gear(String);

#[derive(Type, Serialize)]
pub struct Speed(u32);

#[derive(Type, Serialize)]
pub struct RPM(u32);

#[derive(Type, Serialize)]
pub struct Telemetry {
    ts: f64,
    throttle: u32,
    brake: u32,
    abs_active: bool,
}

#[derive(Type, Serialize)]
pub struct Proximity {
    is_left: bool,
    is_right: bool,
}

#[derive(Type, Serialize)]
pub struct Position(u32);

#[derive(Type, Serialize, Clone)]
pub struct Gap(String);

#[derive(Type, Serialize, Clone)]
pub struct RelativeGap(String);

#[derive(Type, Serialize)]
pub struct TrackMapDriver {
    car_id: u32,
    position: u32,
    is_leader: bool,
    is_player: bool,
    lap_dist_pct: f32,
    is_in_pits: bool,
    is_off_track: bool,
    is_off_world: bool,
    x: f32,
    y: f32,
}

#[derive(Type, Serialize)]
pub struct TrackMap(Vec<TrackMapDriver>);

#[derive(Type, Serialize, Clone)]
pub struct IRating(String);

#[derive(Type, Serialize)]
pub struct StandingsDriver {
    car_id: u32,
    position: u32,
    user_name: String,
    car_number: String,
    irating: IRating,
    license: String,
    leader_gap: Gap,
    best_lap: LapTime,
    last_lap: LapTime,
    is_player: bool,
    is_leader: bool,
    is_in_pits: bool,
}

#[derive(Type, Serialize)]
pub struct Standings(Vec<StandingsDriver>);

#[derive(Type, Serialize, Clone)]
pub struct RelativeDriver {
    car_id: u32,
    position: u32,
    user_name: String,
    car_number: String,
    irating: IRating,
    license: String,
    player_relative_gap: RelativeGap,
    is_player: bool,
    is_in_pits: bool,
    is_off_track: bool,
    is_off_world: bool,
}

#[derive(Type, Serialize)]
pub struct Relative(Vec<RelativeDriver>);

#[derive(Type, Serialize)]
pub struct LapTimeData {
    lap: u32,
    lap_time: LapTime,
}

#[derive(Type, Serialize)]
pub struct LapTimes(Vec<LapTimeData>);

#[derive(Type, Serialize)]
pub struct TrackID(u32);

#[derive(Type, Serialize)]
pub struct StrengthOfField(u32);

impl Active {
    pub fn new(value: bool) -> Self {
        Active(value)
    }
}

impl SessionTimeTotal {
    pub fn new(value: Duration) -> Self {
        SessionTimeTotal(humantime::format_duration(value).to_string())
    }
}

impl LapsTotal {
    pub fn new(value: u32) -> Self {
        LapsTotal(value)
    }
}

impl Incidents {
    pub fn new(value: u32) -> Self {
        Incidents(value)
    }
}

impl GearRPM {
    pub fn new(value: u32) -> Self {
        GearRPM(value)
    }
}

impl CurrentTime {
    pub fn new(value: String) -> Self {
        CurrentTime(value)
    }
}

impl SessionTime {
    pub fn new(value: Duration) -> Self {
        let ss = value.as_secs();
        let (hh, ss) = (ss / 3600, ss % 3600);
        let (mm, ss) = (ss / 60, ss % 60);
        SessionTime(format!("{:0>2}:{:02}:{:02}", hh, mm, ss))
    }
}

impl Laps {
    pub fn new(value: u32) -> Self {
        Laps(value)
    }
}

impl LapTime {
    pub fn new(value: Duration) -> Self {
        let ss = value.as_secs();
        let (mm, ss) = (ss / 60, ss % 60);
        LapTime(format!("{}:{:02}:{:03}", mm, ss, value.subsec_millis()))
    }

    pub fn new_from_sd(value: SignedDuration) -> Self {
        let lap_time = match value {
            value if value.as_secs_f32() <= 0.0 => "–:--:--".to_string(),
            value => {
                let ss = value.as_secs();
                let (mm, ss) = (ss / 60, ss % 60);
                format!("{}:{:02}.{:03}", mm, ss, value.subsec_millis())
            }
        };
        LapTime(lap_time)
    }
}

impl DeltaTime {
    pub fn new(value: SignedDuration) -> Self {
        let delta_last_time_str = match value.as_secs_f32() {
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
        DeltaTime(delta_last_time_str)
    }
}

impl SessionState {
    pub fn new(laps_total: u32, laps_remain: i32, time_remain: f64) -> Self {
        let session_state = match laps_total {
            0 => {
                if time_remain <= 0. {
                    "Last lap".to_string()
                } else {
                    let session_time_remain_value = Duration::try_from_secs_f64(time_remain);
                    match session_time_remain_value {
                        Ok(session_time_remain_value) => {
                            let ss = session_time_remain_value.as_secs();
                            let (hh, ss) = (ss / 3600, ss % 3600);
                            let (mm, ss) = (ss / 60, ss % 60);
                            if hh > 0 {
                                format!("{}:{:02}:{:02} left", hh, mm, ss)
                            } else {
                                format!("{:02}:{:02} left", mm, ss)
                            }
                        }
                        Err(_) => "".to_string(),
                    }
                }
            }
            _ => match laps_remain {
                0 => "".to_string(),
                1 => "Last lap".to_string(),
                _ => {
                    format!("{} laps left", laps_remain)
                }
            },
        };
        SessionState(session_state)
    }
}

impl Gear {
    pub fn new(value: i32) -> Self {
        let gear_str = match value {
            -1 => "R".to_string(),
            0 => "N".to_string(),
            _ => value.to_string(),
        };
        Gear(gear_str)
    }
}

impl Speed {
    pub fn new(value: u32) -> Self {
        Speed(value)
    }
}

impl RPM {
    pub fn new(value: u32) -> Self {
        RPM(value)
    }
}

impl Telemetry {
    pub fn new(ts: f64, throttle: u32, brake: u32, abs_active: bool) -> Self {
        Telemetry {
            ts,
            throttle,
            brake,
            abs_active,
        }
    }
}

impl Proximity {
    pub fn new(is_left: bool, is_right: bool) -> Self {
        Proximity { is_left, is_right }
    }
}

impl Position {
    pub fn new(value: u32) -> Self {
        Position(value)
    }
}

impl Gap {
    pub fn new(
        position: u32,
        driver_positions: &[u32],
        drivers: &HashMap<u32, Driver>,
        is_leader: bool,
    ) -> Self {
        if position < 1 || position as usize > driver_positions.len() {
            return Gap("-".to_string());
        }
        let car_id = driver_positions[position as usize - 1];
        let driver = drivers.get(&car_id);
        let gap = match driver {
            None => "-".to_string(),
            Some(driver) => {
                let gap = match is_leader {
                    true => driver.leader_gap,
                    false => driver.player_gap,
                };
                let gap_laps = match is_leader {
                    true => driver.leader_gap_laps,
                    false => driver.player_gap_laps,
                };
                match gap_laps {
                    0 => {
                        let gap = gap.as_abs_secs_f32();
                        format!(
                            "{}.{}",
                            gap as i32,
                            min((gap.fract() * 10.0).round() as i32, 9)
                        )
                    }
                    _ => format!("L{}", gap_laps.abs()),
                }
            }
        };
        Gap(gap)
    }
}

impl RelativeGap {
    pub fn new(position: u32, driver_positions: &[u32], drivers: &HashMap<u32, Driver>) -> Self {
        if position < 1 || position as usize > driver_positions.len() {
            return RelativeGap("-".to_string());
        }
        let car_id = driver_positions[position as usize - 1];
        let driver = drivers.get(&car_id);
        let relative_gap = match driver {
            None => "-".to_string(),
            Some(driver) => {
                let raw_gap = driver.player_relative_gap;
                match raw_gap.as_abs_secs_f32() {
                    0.0 => "-".to_string(),
                    value => format!(
                        "{}.{}",
                        value as i32,
                        min((value.fract() * 10.0).round() as i32, 9)
                    ),
                }
            }
        };
        RelativeGap(relative_gap)
    }
}

impl TrackMapDriver {
    pub fn new(driver: &Driver) -> Self {
        TrackMapDriver {
            car_id: driver.car_id,
            position: driver.position,
            is_leader: driver.is_leader,
            is_player: driver.is_player,
            lap_dist_pct: driver.lap_dist_pct,
            is_in_pits: driver.is_in_pits,
            is_off_track: driver.is_off_track,
            is_off_world: driver.is_off_world,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl TrackMap {
    pub fn new(drivers: &HashMap<u32, Driver>) -> Self {
        let drivers = drivers
            .iter()
            .map(|(_, driver)| TrackMapDriver::new(driver))
            .collect();
        TrackMap(drivers)
    }
}

impl IRating {
    pub fn new(value: u32) -> Self {
        IRating(format!("{:.1}k", value as f32 / 1000.0))
    }
}

impl StandingsDriver {
    pub fn new(driver: &Driver, driver_positions: &[u32], drivers: &HashMap<u32, Driver>) -> Self {
        StandingsDriver {
            car_id: driver.car_id,
            position: driver.position,
            user_name: driver.user_name.clone(),
            car_number: driver.car_number.clone(),
            irating: IRating::new(driver.irating),
            license: driver.lic_string.clone(),
            leader_gap: Gap::new(driver.position, driver_positions, drivers, true),
            best_lap: LapTime::new_from_sd(driver.best_lap_time),
            last_lap: LapTime::new_from_sd(driver.last_lap_time),
            is_player: driver.is_player,
            is_leader: driver.is_leader,
            is_in_pits: driver.is_in_pits,
        }
    }
}

impl Standings {
    pub fn new(driver_positions: &[u32], drivers: &HashMap<u32, Driver>) -> Self {
        let mut sorted_drivers = drivers.values().cloned().collect::<Vec<Driver>>();
        sorted_drivers.sort_by(|a, b| a.position.cmp(&b.position));
        let drivers = sorted_drivers
            .iter()
            .map(|driver| StandingsDriver::new(driver, driver_positions, drivers))
            .collect();
        Standings(drivers)
    }
}

impl RelativeDriver {
    pub fn new(driver: &Driver, driver_positions: &[u32], drivers: &HashMap<u32, Driver>) -> Self {
        RelativeDriver {
            car_id: driver.car_id,
            position: driver.position,
            user_name: driver.user_name.clone(),
            car_number: driver.car_number.clone(),
            irating: IRating::new(driver.irating),
            license: driver.lic_string.clone(),
            player_relative_gap: RelativeGap::new(driver.position, driver_positions, drivers),
            is_player: driver.is_player,
            is_in_pits: driver.is_in_pits,
            is_off_track: driver.is_off_track,
            is_off_world: driver.is_off_world,
        }
    }
}

impl Default for RelativeDriver {
    fn default() -> Self {
        RelativeDriver {
            car_id: 0,
            position: 0,
            user_name: "".to_string(),
            car_number: "".to_string(),
            irating: IRating("".to_string()),
            license: "".to_string(),
            player_relative_gap: RelativeGap("".to_string()),
            is_player: false,
            is_in_pits: false,
            is_off_track: false,
            is_off_world: false,
        }
    }
}

impl Relative {
    pub fn new(driver_positions: &[u32], drivers: &HashMap<u32, Driver>) -> Self {
        let mut sorted_drivers: Vec<Driver> = drivers
            .values()
            .filter(|driver| driver.is_player || !driver.is_off_world)
            .cloned()
            .collect();
        sorted_drivers.sort_by(|a, b| {
            a.player_relative_gap
                .partial_cmp(&b.player_relative_gap)
                .unwrap()
        });
        let player_idx = sorted_drivers
            .iter()
            .enumerate()
            .find(|(_, driver)| driver.is_player)
            .unwrap()
            .0;
        let mut result: Vec<RelativeDriver> =
            vec![RelativeDriver::default(); RELATIVE_DRIVERS_BEFORE + RELATIVE_DRIVERS_AFTER + 1];
        for idx in 0..RELATIVE_DRIVERS_BEFORE {
            if player_idx <= idx {
                break;
            }
            let driver = sorted_drivers.get(player_idx - idx - 1);
            let value = match driver {
                Some(driver) => RelativeDriver::new(driver, driver_positions, drivers),
                None => RelativeDriver::default(),
            };
            result[RELATIVE_DRIVERS_BEFORE - idx - 1] = value;
        }
        let player = sorted_drivers.get(player_idx);
        let value = match player {
            Some(driver) => RelativeDriver::new(driver, driver_positions, drivers),
            None => RelativeDriver::default(),
        };
        result[RELATIVE_DRIVERS_BEFORE] = value;
        for idx in 0..RELATIVE_DRIVERS_AFTER {
            let relative_idx = player_idx + idx + 1;
            if relative_idx >= drivers.len() {
                break;
            };
            let driver = sorted_drivers.get(relative_idx);
            let value = match driver {
                Some(driver) => RelativeDriver::new(driver, driver_positions, drivers),
                None => RelativeDriver::default(),
            };
            result[RELATIVE_DRIVERS_BEFORE + idx + 1] = value;
        }
        Relative(result)
    }
}

impl LapTimeData {
    pub fn new(lap: u32, lap_time: SignedDuration) -> Self {
        LapTimeData {
            lap,
            lap_time: LapTime::new_from_sd(lap_time),
        }
    }
}

impl LapTimes {
    pub fn new(player_lap_times: &[telemetry_data::LapTime]) -> Self {
        let lap_times = player_lap_times
            .iter()
            .map(|lap_time| LapTimeData::new(lap_time.lap(), lap_time.lap_time()))
            .collect();
        LapTimes(lap_times)
    }
}

impl TrackID {
    pub fn new(value: u32) -> Self {
        TrackID(value)
    }
}

impl StrengthOfField {
    pub fn new(value: u32) -> Self {
        StrengthOfField(value)
    }
}
