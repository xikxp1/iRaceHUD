use std::cmp::Ordering;
use std::str::FromStr;
use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, Local};
use log::{debug, error, info};
use simetry::iracing::{SimState, UNLIMITED_LAPS, UNLIMITED_TIME};

use crate::session::session_type::SessionType;
use crate::util::{get_strength_of_field::get_strength_of_field, signed_duration::SignedDuration};

use super::results_position::ResultsPosition;
use super::{driver::Driver, lap_time::LapTime};

#[derive(Default, Debug)]
pub struct SessionData {
    pub abs_active: bool,
    pub activated: bool,
    pub active: bool,
    pub brake: u32,
    pub lap_dist: u32, // in cm
    pub current_time: DateTime<Local>,
    pub delta_last_time: SignedDuration,
    pub delta_best_time: SignedDuration,
    pub delta_optimal_time: SignedDuration,
    pub driver_positions: Vec<u32>,
    pub player_class_driver_positions: Vec<u32>,
    pub drivers: HashMap<u32, Driver>,
    pub gear_blink_rpm: u32,
    pub gear_shift_rpm: u32,
    pub gear: i32,
    pub incident_limit: u32,
    pub incidents: u32,
    pub is_left: bool,
    pub is_right: bool,
    pub lap_time: SignedDuration,
    pub lap: u32,
    pub laps_total: u32,
    pub last_lap_time: SignedDuration,
    pub leader_car_id: u32,
    pub player_car_class: u32,
    pub player_car_id: Option<u32>,
    pub player_lap_times: Vec<LapTime>,
    pub position: u32,
    pub class_position: u32,
    pub positions_total: u32,
    pub race_laps: u32,
    pub rpm: u32,
    pub session_info_update: i32,
    pub session_laps_remaining: u32,
    pub session_time_remaining: SignedDuration,
    pub session_time_total: Duration,
    pub session_time: Duration,
    pub speed: u32,
    pub strength_of_field: u32,
    pub throttle: u32,
    pub track_id: u32,
    pub processed_slow: bool,
    pub session_type: SessionType,
    pub player_car_class_name: String,
    pub best_lap_time: Option<SignedDuration>,
    pub best_lap_time_car_id: Option<u32>,
    pub session_num: u32,
    pub results_positions: Vec<ResultsPosition>,
    pub results_positions_mapping: HashMap<u32, usize>,
    pub results_official: bool,
    pub steering_angle: i32, // in radian * 100
}

#[derive(PartialEq)]
pub enum ProcessTickResult {
    None,
    StateChanged,
}

impl SessionData {
    pub fn process_tick(
        &mut self,
        sim_state: &SimState,
        should_process_slow: bool,
        force_active: bool,
    ) -> ProcessTickResult {
        self.processed_slow = force_active || should_process_slow;

        // current_time
        let current_time = Local::now();
        self.current_time = current_time;

        let session_tick = sim_state.read_name("SessionTick").unwrap_or(0);
        if session_tick == 0 {
            info!("Session tick is 0, skipping");
            return ProcessTickResult::None;
        }

        // active
        let raw_is_on_track_value = sim_state.read_name("IsOnTrack").unwrap_or(false);
        let raw_is_on_track_car_value = sim_state.read_name("IsOnTrackCar").unwrap_or(false);

        let active = force_active || (raw_is_on_track_value && raw_is_on_track_car_value);
        let activated = active != self.active;
        if activated {
            info!(
                "Session state changed to {}",
                if active { "active" } else { "inactive" }
            );
            if active {
                // TODO: save some data for reinitializing the session
                // TODO: allow to show some data while player not on track
                let _ = std::mem::take(self);
            }
            self.active = active;
        }
        self.active = active;
        self.activated = activated;

        if !active {
            return ProcessTickResult::None;
        }

        // slow vars
        if should_process_slow {
            // session_num
            let raw_session_num_value = sim_state.read_name("SessionNum").unwrap_or(0);
            let session_num_value = raw_session_num_value as u32;
            self.session_num = session_num_value;

            // session_time_total
            let raw_session_time_total_value = match sim_state.read_name("SessionTimeTotal") {
                Some(value) if value >= UNLIMITED_TIME => 0.0,
                Some(value) => value,
                None => 0.0,
            };
            let session_time_total_value = Duration::from_secs_f64(raw_session_time_total_value);
            self.session_time_total = session_time_total_value;

            // session_laps_total
            let raw_session_laps_total_value = match sim_state.read_name("SessionLapsTotal") {
                Some(value) if value >= UNLIMITED_LAPS => 0,
                Some(value) => value,
                None => 0,
            };
            let laps_total_value = raw_session_laps_total_value as u32;
            self.laps_total = laps_total_value;

            // incidents
            let raw_incidents_value = sim_state.read_name("PlayerCarMyIncidentCount").unwrap_or(0);
            let incidents_value = raw_incidents_value as u32;
            self.incidents = incidents_value;

            // gear_shift_rpm
            let raw_player_car_sl_shift_rpm_value: f32 =
                sim_state.read_name("PlayerCarSLShiftRPM").unwrap_or(0.0);
            let gear_shift_rpm_value = raw_player_car_sl_shift_rpm_value.round() as u32;
            self.gear_shift_rpm = gear_shift_rpm_value;

            // gear_blink_rpm
            let raw_player_car_sl_blink_rpm_value: f32 =
                sim_state.read_name("PlayerCarSLBlinkRPM").unwrap_or(0.0);
            let gear_blink_rpm_value = raw_player_car_sl_blink_rpm_value.round() as u32;
            self.gear_blink_rpm = gear_blink_rpm_value;
        }

        // session_time
        let raw_session_time_value = sim_state.read_name("SessionTime").unwrap_or(0.0);
        let session_time_value = Duration::from_secs_f64(raw_session_time_value);
        self.session_time = session_time_value;

        // player_car_idx
        let player_car_idx_value = sim_state.read_name("PlayerCarIdx").unwrap_or(0);
        self.player_car_id = Some(player_car_idx_value as u32);

        // player_car_class
        let player_car_class_value = sim_state.read_name("PlayerCarClass").unwrap_or(0);
        self.player_car_class = player_car_class_value as u32;

        // lap
        let raw_lap_value = sim_state.read_name("Lap").unwrap_or(0);
        let lap_value = raw_lap_value as u32;
        self.lap = lap_value;

        //race_laps
        let raw_race_laps_value = sim_state.read_name("RaceLaps").unwrap_or(0);
        let race_laps_value = raw_race_laps_value as u32;
        self.race_laps = race_laps_value;

        // lap_time
        let raw_lap_current_lap_time_value =
            sim_state.read_name("LapCurrentLapTime").unwrap_or(0.0);
        let lap_time_value = SignedDuration::from_secs_f32(raw_lap_current_lap_time_value);
        self.lap_time = lap_time_value;

        // lap_dist
        let raw_lap_dist_value: f32 = sim_state.read_name("LapDist").unwrap_or(0.0);
        let lap_dist_value: u32 = ((raw_lap_dist_value * 100.0 / 20.0).round() * 20.0) as u32; // round to 20cm
        self.lap_dist = lap_dist_value;

        // delta_last_time
        let raw_lap_delta_to_session_last_lap_value = sim_state
            .read_name("LapDeltaToSessionLastlLap")
            .unwrap_or(0.0);
        let delta_last_time_value =
            SignedDuration::from_secs_f32(raw_lap_delta_to_session_last_lap_value);
        self.delta_last_time = delta_last_time_value;

        // delta_best_time
        let raw_lap_delta_to_best_lap_value =
            sim_state.read_name("LapDeltaToBestLap").unwrap_or(0.0);
        let delta_best_time_value = SignedDuration::from_secs_f32(raw_lap_delta_to_best_lap_value);
        self.delta_best_time = delta_best_time_value;

        // delta_optimal_time
        let raw_lap_delta_to_optimal_lap_value =
            sim_state.read_name("LapDeltaToOptimalLap").unwrap_or(0.0);
        let delta_optimal_time_value =
            SignedDuration::from_secs_f32(raw_lap_delta_to_optimal_lap_value);
        self.delta_optimal_time = delta_optimal_time_value;

        // session_state
        let raw_session_time_remain_value = sim_state.read_name("SessionTimeRemain").unwrap_or(0.0);
        self.session_time_remaining = SignedDuration::from_secs_f64(raw_session_time_remain_value);

        let raw_session_laps_remain_ex_value =
            sim_state.read_name("SessionLapsRemainEx").unwrap_or(0);
        self.session_laps_remaining = raw_session_laps_remain_ex_value as u32;

        // gear
        let raw_gear_value = sim_state.read_name("Gear").unwrap_or(0);
        self.gear = raw_gear_value;

        // speed
        let raw_speed_value: f32 = sim_state.read_name("Speed").unwrap_or(0.0);
        let speed_value = (raw_speed_value * 3.6).round() as u32;
        self.speed = speed_value;

        // rpm
        let raw_rpm_value: f32 = sim_state.read_name("RPM").unwrap_or(0.0);
        let rpm_value = raw_rpm_value.round() as u32;
        self.rpm = rpm_value;

        // telemetry (brake+throttle)
        let raw_brake_value: f32 = sim_state.read_name("Brake").unwrap_or(0.0);
        let brake_value = (raw_brake_value * 100.0).round() as u32;
        let raw_throttle_value: f32 = sim_state.read_name("Throttle").unwrap_or(0.0);
        let throttle_value = (raw_throttle_value * 100.0).round() as u32;
        let abs_active_value = sim_state.read_name("BrakeABSactive").unwrap_or(false);
        let raw_steering_angle_value: f32 =
            sim_state.read_name("SteeringWheelAngle").unwrap_or(0.0);
        let steering_angle_value = (raw_steering_angle_value * 100.0).round() as i32;
        self.brake = brake_value;
        self.throttle = throttle_value;
        self.abs_active = abs_active_value;
        self.steering_angle = steering_angle_value;

        // proximity
        // TODO: bitfield parsing doesn't work
        let raw_car_left_right_value = sim_state.read_name("CarLeftRight").unwrap_or(0);
        let is_left = raw_car_left_right_value == 2
            || raw_car_left_right_value == 4
            || raw_car_left_right_value == 5;
        let is_right = raw_car_left_right_value == 3
            || raw_car_left_right_value == 4
            || raw_car_left_right_value == 6;
        self.is_left = is_left;
        self.is_right = is_right;

        // positions+distance
        let lap_dist_pct: Vec<f32> = sim_state.read_name("CarIdxLapDistPct").unwrap_or_default();

        let laps_completed: Vec<i32> = sim_state
            .read_name("CarIdxLapCompleted")
            .unwrap_or_default();

        let laps_started: Vec<i32> = sim_state.read_name("CarIdxLap").unwrap_or_default();

        let car_idx_est_time_value: Vec<f32> =
            sim_state.read_name("CarIdxEstTime").unwrap_or_default();

        let car_idx_best_lap_time_value: Vec<f32> =
            sim_state.read_name("CarIdxBestLapTime").unwrap_or_default();

        let car_idx_last_lap_time_value: Vec<f32> =
            sim_state.read_name("CarIdxLastLapTime").unwrap_or_default();

        let car_idx_track_surface_value: Vec<i32> = sim_state
            .read_name("CarIdxTrackSurface")
            .unwrap_or_default();

        for (car_id, driver) in self.drivers.iter_mut() {
            let result_position_idx = self.results_positions_mapping.get(car_id);
            let result_position = if let Some(idx) = result_position_idx {
                &self.results_positions[*idx]
            } else {
                &ResultsPosition::default()
            };

            let lap_dist_pct_value = match lap_dist_pct[*car_id as usize] {
                value if value < 0.0 => 0.0,
                value => value,
            };
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
            if !driver.best_lap_time.is_positive() && result_position.fastest_time.is_positive() {
                driver.best_lap_time = result_position.fastest_time;
            }
            driver.last_lap_time =
                SignedDuration::from_secs_f32(car_idx_last_lap_time_value[*car_id as usize]);
            if !driver.last_lap_time.is_positive() && result_position.last_time.is_positive() {
                driver.last_lap_time = result_position.last_time;
            }
            driver.is_in_pits = car_idx_track_surface_value[*car_id as usize] == 1
                || car_idx_track_surface_value[*car_id as usize] == 2;
            driver.is_off_track = car_idx_track_surface_value[*car_id as usize] == 0;
            driver.is_off_world = car_idx_track_surface_value[*car_id as usize] == -1;
        }

        let mut driver_positions = self
            .drivers
            .iter()
            .map(|(car_id, driver)| (*car_id, driver))
            .collect::<Vec<(u32, &Driver)>>();
        driver_positions.sort_by(|a, b| {
            a.1.result_position
                .unwrap_or(0)
                .partial_cmp(&b.1.result_position.unwrap_or(0))
                .unwrap()
        });
        if self.session_type == SessionType::Race {
            driver_positions.sort_by(|a, b| {
                a.1.total_completed
                    .partial_cmp(&b.1.total_completed)
                    .unwrap()
                    .reverse()
            });
        } else if self.session_type == SessionType::Practice
            || self.session_type == SessionType::Qualify
        {
            driver_positions.sort_by(|a, b| {
                if a.1.best_lap_time.is_positive() && b.1.best_lap_time.is_positive() {
                    a.1.best_lap_time.partial_cmp(&b.1.best_lap_time).unwrap()
                } else if !a.1.best_lap_time.is_positive() && b.1.best_lap_time.is_positive() {
                    Ordering::Greater
                } else if a.1.best_lap_time.is_positive() && !b.1.best_lap_time.is_positive() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
        }
        self.driver_positions = driver_positions.iter().map(|(car_id, _)| *car_id).collect();

        let mut car_class_positions_count: HashMap<u32, u32> = HashMap::new();
        let mut position_total: u32 = 0;
        let mut player_class_driver_positions: Vec<u32> = Vec::new();

        for (position, car_id) in self.driver_positions.iter().enumerate() {
            let driver = self.drivers.get_mut(car_id);
            if driver.is_none() {
                error!("Driver not found while updating position");
            }
            let driver = driver.unwrap();
            let class_position = car_class_positions_count
                .entry(driver.car_class_id)
                .or_insert(0);
            *class_position += 1;
            driver.class_position = *class_position;
            driver.position = position as u32 + 1;
            if self.player_car_id.is_some() && *car_id == self.player_car_id.unwrap() {
                self.position = driver.position;
                self.class_position = driver.class_position;
            }
            if driver.car_class_id == self.player_car_class && driver.class_position == 1 {
                self.leader_car_id = *car_id;
            }
            if driver.car_class_id == self.player_car_class {
                player_class_driver_positions.push(*car_id);
                position_total += 1;
            }
            if driver.car_class_id == self.player_car_class
                && driver.best_lap_time > SignedDuration::ZERO
                && driver.best_lap_time < self.best_lap_time.unwrap_or(SignedDuration::MAX)
            {
                self.best_lap_time = Some(driver.best_lap_time);
                self.best_lap_time_car_id = Some(*car_id);
            }
        }

        // player_class_driver_positions
        self.player_class_driver_positions = player_class_driver_positions;

        // positions_total
        self.positions_total = position_total;

        // gaps
        if !self.driver_positions.is_empty() && self.player_car_id.is_some() {
            let player = self.drivers.get(&self.player_car_id.unwrap());
            if player.is_none() {
                error!("Player not found");
            }
            let player = player.unwrap();
            let player_total_completed = player.total_completed;
            let player_estimated = player.estimated;
            let player_dist_pct = player.lap_dist_pct;
            let player_car_class_est_lap_time = player.car_class_est_lap_time;

            let leader = self.drivers.get(&self.leader_car_id);
            if leader.is_none() {
                error!("Leader not found");
            }
            let leader = leader.unwrap();
            let leader_total_completed = leader.total_completed;
            let leader_estimated = leader.estimated;

            for driver in self.drivers.values_mut() {
                driver.is_leader = driver.car_id == self.leader_car_id;
                driver.is_player =
                    self.player_car_id.is_some() && driver.car_id == self.player_car_id.unwrap();
                let leader_gap_laps = leader_total_completed - driver.total_completed;
                if leader_gap_laps >= 1.0 {
                    driver.leader_gap_laps = leader_gap_laps as i32;
                    driver.leader_gap = SignedDuration::ZERO;
                } else {
                    driver.leader_gap_laps = 0;
                    driver.leader_gap = match leader_estimated - driver.estimated {
                        value if value >= SignedDuration::ZERO => {
                            leader_estimated - driver.estimated
                        }
                        _ => leader_estimated + driver.car_class_est_lap_time - driver.estimated,
                    };
                }
                let player_gap_laps = player_total_completed - driver.total_completed;
                if player_gap_laps >= 1.0 || player_gap_laps <= -1.0 {
                    driver.player_gap_laps = player_gap_laps as i32;
                    driver.player_gap = SignedDuration::ZERO;
                } else {
                    driver.player_gap_laps = 0;
                    driver.player_gap = match player_gap_laps {
                        value if value >= 0.0 => match player_estimated - driver.estimated {
                            value if value >= SignedDuration::ZERO => {
                                player_estimated - driver.estimated
                            }
                            _ => {
                                player_estimated + driver.car_class_est_lap_time - driver.estimated
                            }
                        },
                        _ => match player_estimated - driver.estimated {
                            value if value >= SignedDuration::ZERO => {
                                driver.estimated + driver.car_class_est_lap_time - player_estimated
                            }
                            _ => player_estimated - driver.estimated,
                        },
                    };
                };
                let dist_pct_diff = player_dist_pct - driver.lap_dist_pct;
                let delta_dist_pct = match dist_pct_diff {
                    value if value > 0.5 => value - 1.0,
                    value if value < -0.5 => value + 1.0,
                    value => value,
                };
                driver.player_relative_gap = player_car_class_est_lap_time * delta_dist_pct;

                if self.session_type == SessionType::Race {
                    let ahead_behind = match player_gap_laps {
                        value if value > 0.5 => -1,
                        value if value < -0.5 => 1,
                        _ => 0,
                    };
                    driver.ahead_behind = ahead_behind;
                }
            }
        }

        // extra slow vars
        if should_process_slow {
            // player_lap_times
            if self.lap > 1 {
                let raw_lap_last_lap_time_value =
                    sim_state.read_name("LapLastLapTime").unwrap_or(0.0);
                let lap_last_lap_time_value =
                    SignedDuration::from_secs_f32(raw_lap_last_lap_time_value);
                match lap_last_lap_time_value {
                    value if value.is_positive() => {
                        // TODO: This check is not exactly safe
                        if lap_last_lap_time_value != self.last_lap_time {
                            self.player_lap_times
                                .insert(0, LapTime::new(self.lap - 1, value));
                            self.last_lap_time = value;
                        }
                    }
                    _ => {}
                }
            }
        }

        let session_info_update = sim_state.header().session_info_update;
        if self.session_info_update != session_info_update {
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
            self.incident_limit = incident_limit_value;

            // track_id
            let track_id = session["WeekendInfo"]["TrackID"].as_i64().unwrap_or(0) as u32;
            self.track_id = track_id;

            // session
            let sessions = session["SessionInfo"]["Sessions"].as_vec();
            if sessions.is_some() {
                let sessions = sessions.unwrap();
                for session in sessions {
                    let session_num = session["SessionNum"].as_i64();
                    let session_type = session["SessionType"].as_str();
                    if session_num.is_none()
                        || session_type.is_none()
                        || session_num.unwrap() as u32 != self.session_num
                    {
                        continue;
                    }
                    let session_type = session_type.unwrap().to_string();
                    self.session_type = SessionType::from_str(&session_type).unwrap();

                    let results_positions = session["ResultsPositions"].as_vec();
                    if results_positions.is_some() {
                        let results_positions = results_positions.unwrap();
                        for (i, result_position) in results_positions.iter().enumerate() {
                            let car_id = result_position["CarIdx"].as_i64().unwrap_or(0) as u32;
                            let position = result_position["Position"].as_i64().unwrap_or(0) as u32;
                            let class_position =
                                (result_position["ClassPosition"].as_i64().unwrap_or(0) + 1) as u32;
                            let fastest_time =
                                result_position["FastestTime"].as_f64().unwrap_or(0.0);
                            let fastest_time = SignedDuration::from_secs_f64(fastest_time);
                            let last_time = result_position["LastTime"].as_f64().unwrap_or(0.0);
                            let last_time = SignedDuration::from_secs_f64(last_time);
                            let reason_out_id =
                                result_position["ReasonOutID"].as_i64().unwrap_or(0) as u32;

                            let results_position = ResultsPosition {
                                car_id,
                                position,
                                class_position,
                                fastest_time,
                                last_time,
                                reason_out_id,
                            };

                            self.results_positions.push(results_position);
                            self.results_positions_mapping.insert(car_id, i);
                        }
                    }

                    let results_official = session["ResultsOfficial"].as_i64().unwrap_or(0) != 0;
                    self.results_official = results_official;
                }
            }

            let drivers = session["DriverInfo"]["Drivers"].as_vec();

            match drivers {
                Some(drivers) => {
                    for driver in drivers {
                        let car_id = driver["CarIdx"].as_i64();
                        if car_id.is_none() {
                            error!("CarIdx not found");
                            continue;
                        }
                        let car_id = car_id.unwrap() as u32;

                        let user_name = driver["UserName"].as_str();
                        if user_name.is_none() {
                            error!("UserName not found");
                            continue;
                        }
                        let user_name = user_name.unwrap().to_string();

                        let team_name = driver["TeamName"].as_str();
                        if team_name.is_none() {
                            error!("TeamName not found");
                            continue;
                        }
                        let team_name = team_name.unwrap().to_string();

                        // Skip pace car
                        if user_name == "Pace Car" {
                            continue;
                        }

                        if self.player_car_id.is_some() && car_id == self.player_car_id.unwrap() {
                            let car_class_short_name = driver["CarClassShortName"].as_str();
                            if car_class_short_name.is_none() {
                                // iRacing doesn't provide this value in AI races: https://github.com/SHWotever/SimHub/issues/1847
                                let car_screen_name_short = driver["CarScreenNameShort"].as_str();
                                if car_screen_name_short.is_none() {
                                    error!("CarScreenNameShort not found");
                                } else {
                                    let car_screen_name_short =
                                        car_screen_name_short.unwrap().to_string();
                                    self.player_car_class_name = car_screen_name_short;
                                }
                            } else {
                                let car_class_short_name =
                                    car_class_short_name.unwrap().to_string();
                                self.player_car_class_name = car_class_short_name;
                            }
                        }

                        let car_number = driver["CarNumber"].as_str();
                        if car_number.is_none() {
                            error!("CarNumber not found");
                            continue;
                        }
                        let car_number = car_number.unwrap().to_string();

                        let car_class_id = driver["CarClassID"].as_i64();
                        if car_class_id.is_none() {
                            error!("CarClassID not found");
                            continue;
                        }
                        let car_class_id = car_class_id.unwrap() as u32;

                        let irating = driver["IRating"].as_i64();
                        if irating.is_none() {
                            error!("IRating not found");
                            continue;
                        }
                        let irating = irating.unwrap() as u32;

                        let lic_string = driver["LicString"].as_str();
                        if lic_string.is_none() {
                            error!("LicString not found");
                            continue;
                        }
                        let lic_string = lic_string.unwrap();

                        let car_class_est_lap_time = driver["CarClassEstLapTime"].as_f64();
                        if car_class_est_lap_time.is_none() {
                            error!("CarClassEstLapTime not found");
                            continue;
                        }
                        let car_class_est_lap_time = car_class_est_lap_time.unwrap();
                        let car_class_est_lap_time =
                            SignedDuration::from_secs_f64(car_class_est_lap_time);

                        let car_class_color = driver["CarClassColor"].as_i64();
                        if car_class_color.is_none() {
                            error!("CarClassColor not found");
                            continue;
                        }
                        let car_class_color = car_class_color.unwrap() as u32;

                        let driver_results_position = self.results_positions_mapping.get(&car_id);
                        let mut result_position: Option<u32> = None;
                        let mut result_class_position: Option<u32> = None;
                        let mut is_out = false;
                        if let Some(position) = driver_results_position {
                            result_position = Some(self.results_positions[*position].position);
                            result_class_position =
                                Some(self.results_positions[*position].class_position);
                            is_out = self.results_positions[*position].reason_out_id != 0;
                        }

                        let driver = self.drivers.get_mut(&car_id);
                        let driver = match driver {
                            Some(driver) => driver,
                            None => {
                                let mut new_driver = Driver::builder()
                                    .car_id(car_id)
                                    .user_name(user_name)
                                    .car_number(car_number)
                                    .car_class_id(car_class_id)
                                    .irating(irating)
                                    .lic_string(lic_string.to_string())
                                    .car_class_est_lap_time(car_class_est_lap_time)
                                    .is_player_class(self.player_car_class == car_class_id)
                                    .car_class_color(car_class_color)
                                    .team_name(team_name)
                                    .build();
                                new_driver.is_out = is_out;
                                new_driver.result_position = result_position;
                                new_driver.result_class_position = result_class_position;
                                self.drivers.insert(car_id, new_driver);
                                self.drivers.get_mut(&car_id).unwrap()
                            }
                        };
                        driver.is_out = is_out;
                        driver.result_position = result_position;
                        driver.result_class_position = result_class_position;
                    }
                }
                None => {
                    error!("No drivers found");
                    return ProcessTickResult::None;
                }
            }

            if !self.drivers.is_empty() {
                // strength_of_field
                let strength_of_field = get_strength_of_field(self);
                self.strength_of_field = strength_of_field;
            }

            self.session_info_update = session_info_update;
        };
        ProcessTickResult::None
    }
}
