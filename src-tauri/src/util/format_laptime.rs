use super::signed_duration::SignedDuration;

pub fn format_laptime(lap_time: SignedDuration) -> String {
    match lap_time {
        value if value.is_positive() => {
            let ss = value.as_secs();
            let (mm, ss) = (ss / 60, ss % 60);
            format!("{}:{:02}.{:03}", mm, ss, value.subsec_millis())
        }
        _ => "–:--:--".to_string(),
    }
}
