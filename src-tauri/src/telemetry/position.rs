use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct Position(u32);

impl EmittableEvent for Position {
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        Box::new(session.class_position)
    }
}
