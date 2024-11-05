use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct SessionState(String);

impl EmittableEvent for SessionState {
    fn get_event(&self, session: &SessionData) -> Value {
        let session_state = match session.laps_total {
            0 => {
                let session_time_remaining = session.session_time_remaining;
                if !session_time_remaining.is_positive() {
                    "Last lap".to_string()
                } else {
                    let ss = session_time_remaining.as_secs();
                    let (hh, ss) = (ss / 3600, ss % 3600);
                    let (mm, ss) = (ss / 60, ss % 60);
                    if hh > 0 {
                        format!("{}:{:02}:{:02} left", hh, mm, ss)
                    } else {
                        format!("{:02}:{:02} left", mm, ss)
                    }
                }
            }
            _ => match session.session_laps_remaining {
                0 => "".to_string(),
                1 => "Last lap".to_string(),
                value => {
                    format!("{} laps left", value)
                }
            },
        };
        Value::String(session_state)
    }
}
