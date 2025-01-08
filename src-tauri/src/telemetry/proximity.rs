use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct Proximity {
    is_left: bool,
    is_right: bool,
}

impl EmittableEvent for Proximity {
    fn get_event(&self, session: &SessionData) -> Value {
        let mut telemetry = serde_json::Map::new();
        telemetry.insert("is_left".to_string(), Value::Bool(session.is_left));
        telemetry.insert("is_right".to_string(), Value::Bool(session.is_right));
        Value::Object(telemetry)
    }
}
