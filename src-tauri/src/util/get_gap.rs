use std::cmp::min;

use crate::session::session_data::SessionData;

pub fn get_gap(position: u32, session: &SessionData, is_leader: bool) -> String {
    if position < 1 || position as usize > session.player_class_driver_positions.len() {
        return "-".to_string();
    }
    let car_id = session.player_class_driver_positions[position as usize - 1];
    let driver = session.drivers.get(&car_id);
    match driver {
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
                    match gap {
                        value if value >= 100.0 => format!("{}", value as i32),
                        value => format!(
                            "{}.{}",
                            value as i32,
                            min((value.fract() * 10.0).round() as i32, 9)
                        ),
                    }
                }
                _ => format!("L{}", gap_laps.abs()),
            }
        }
    }
}
