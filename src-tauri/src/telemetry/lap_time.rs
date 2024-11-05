use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;
use crate::util::format_laptime::format_laptime;

#[derive(Default, Type, Serialize)]
pub struct LapTime(String);

impl EmittableEvent for LapTime {
    fn get_event(&self, session: &SessionData) -> Value {
        let value = format_laptime(session.lap_time);
        Value::String(value)
    }
}
