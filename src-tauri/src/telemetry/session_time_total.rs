use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct SessionTimeTotal(String);

impl EmittableEvent for SessionTimeTotal {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && session.processed_slow
    }
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        Box::new(humantime::format_duration(session.session_time_total).to_string())
    }
}
