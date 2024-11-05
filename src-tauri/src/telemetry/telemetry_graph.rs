use log::error;
use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct TelemetryGraph {
    ts: f64,
    throttle: u32,
    brake: u32,
    abs_active: bool,
}

impl EmittableEvent for TelemetryGraph {
    fn get_event(&self, session: &SessionData) -> Value {
        let mut telemetry = serde_json::Map::new();
        let session_time = session.session_time.as_secs_f64();
        let session_time = serde_json::Number::from_f64(session_time);
        match session_time {
            Some(session_time) => {
                telemetry.insert("ts".to_string(), Value::Number(session_time));
            }
            None => {
                error!("Failed to convert session time to f64");
            }
        }
        telemetry.insert(
            "throttle".to_string(),
            Value::Number(serde_json::Number::from(session.throttle)),
        );
        telemetry.insert(
            "brake".to_string(),
            Value::Number(serde_json::Number::from(session.brake)),
        );
        telemetry.insert("abs_active".to_string(), Value::Bool(session.abs_active));
        Value::Object(telemetry)
    }
}
