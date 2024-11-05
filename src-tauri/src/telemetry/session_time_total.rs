use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct SessionTimeTotal(String);

impl EmittableEvent for SessionTimeTotal {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && session.processed_slow
    }

    fn get_event(&self, session: &SessionData) -> Value {
        Value::String(humantime::format_duration(session.session_time_total).to_string())
    }
}
