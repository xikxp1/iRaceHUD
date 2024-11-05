use std::cmp::min;

use crate::util::signed_duration::SignedDuration;

pub fn format_delta(value: &SignedDuration) -> String {
    match value.as_secs_f32() {
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
    }
}
