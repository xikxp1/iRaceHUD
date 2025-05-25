use std::cmp::min;

use crate::session::session_data::SessionData;

pub fn get_gap(position: u32, session: &SessionData, is_leader: bool) -> String {
    let postions = match is_leader {
        true => session.driver_positions.clone(),
        false => session.player_class_driver_positions.clone(),
    };
    if position < 1 || position as usize > postions.len() {
        return "-".to_string();
    }
    let car_id = postions[position as usize - 1];
    let driver = session.drivers.get(&car_id);
    match driver {
        None => "-".to_string(),
        Some(driver) => {
            if is_leader && driver.is_leader {
                return "-".to_string();
            }
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
