use std::{collections::HashMap, time::Duration};

use crate::{driver::Driver, signed_duration::SignedDuration};

#[derive(Default)]
pub struct LapTime {
    lap: u32,
    lap_time: SignedDuration,
}

impl LapTime {
    pub fn new(lap: u32, lap_time: SignedDuration) -> Self {
        Self { lap, lap_time }
    }

    pub fn lap(&self) -> u32 {
        self.lap
    }

    pub fn lap_time(&self) -> SignedDuration {
        self.lap_time
    }
}

#[derive(Default)]
pub struct TelemetryData {
    pub active: bool,
    pub session_time: Duration,
    pub player_car_id: u32,
    pub player_car_class: u32,
    pub lap: u32,
    pub race_laps: u32,
    pub lap_time: Duration,
    pub delta_last_time: SignedDuration,
    pub delta_optimal_time: SignedDuration,
    pub gear: i32,
    pub speed: u32,
    pub rpm: u32,
    pub brake: u32,
    pub throttle: u32,
    pub is_left: bool,
    pub is_right: bool,
    pub position: u32,
    pub positions_total: u32,
    pub strength_of_field: u32,
    pub session_time_total: Duration,
    pub laps_total: u32,
    pub incidents: u32,
    pub incident_limit: u32,
    pub gear_shift_rpm: u32,
    pub gear_blink_rpm: u32,
    pub session_info_update: i32,
    pub drivers: HashMap<u32, Driver>,
    pub driver_positions: Vec<u32>,
    pub leader_car_id: u32,
    pub car_class_est_lap_time: SignedDuration,
    pub track_id: u32,
    pub player_lap_times: Vec<LapTime>,
    pub last_lap_time: SignedDuration,
}
