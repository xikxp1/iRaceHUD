use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct LapTime(f64);

impl EmittableEvent for LapTime {
    fn get_event(&self, session: &SessionData) -> Value {
        match session.lap_time.is_positive() {
            true => {
                Value::Number(serde_json::Number::from_f64(session.lap_time.as_secs_f64()).unwrap())
            }
            false => Value::Number(serde_json::Number::from_f64(0.0).unwrap()),
        }
    }
}
