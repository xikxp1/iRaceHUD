use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;
use crate::util::get_gap::get_gap;

#[derive(Default, Type, Serialize)]
pub struct GapNext(String);

impl EmittableEvent for GapNext {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && !session.drivers.is_empty()
    }

    fn get_event(&self, session: &SessionData) -> Value {
        let gap = get_gap(session.position - 1, session, false);
        Value::String(gap)
    }
}
