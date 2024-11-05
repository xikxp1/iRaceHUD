use std::cmp::min;

use crate::session::session_data::SessionData;

pub fn get_relative_gap(position: u32, session: &SessionData) -> String {
    if position < 1 || position as usize > session.driver_positions.len() {
        return "-".to_string();
    }
    let car_id = session.driver_positions[position as usize - 1];
    let driver = session.drivers.get(&car_id);
    match driver {
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
    }
}
