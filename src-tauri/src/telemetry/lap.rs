use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct Lap(u32);

impl EmittableEvent for Lap {
    fn get_event(&self, session: &SessionData) -> Value {
        Value::Number(serde_json::Number::from(session.lap))
    }
}
