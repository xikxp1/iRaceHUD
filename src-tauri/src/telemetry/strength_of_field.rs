use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct StrengthOfField(u32);

impl EmittableEvent for StrengthOfField {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && !session.drivers.is_empty() && session.processed_slow
    }

    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        Box::new(session.strength_of_field)
    }
}
