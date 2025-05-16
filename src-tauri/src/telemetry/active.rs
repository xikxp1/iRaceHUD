use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct Active(bool);

impl EmittableEvent for Active {
    fn is_ready(&self, _session: &SessionData) -> bool {
        true
    }

    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        Box::new(session.active)
    }
}
