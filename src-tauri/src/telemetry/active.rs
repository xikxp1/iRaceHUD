use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct Active(bool);

impl EmittableEvent for Active {
    fn is_ready(&self, _session: &SessionData) -> bool {
        true
    }

    fn get_event(&self, session: &SessionData) -> Value {
        Value::Bool(session.active)
    }
}
