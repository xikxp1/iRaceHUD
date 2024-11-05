use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct LapsTotal(u32);

impl EmittableEvent for LapsTotal {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && session.processed_slow
    }

    fn get_event(&self, session: &SessionData) -> Value {
        Value::Number(serde_json::Number::from(session.laps_total))
    }
}
