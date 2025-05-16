use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct Speed(u32);

impl EmittableEvent for Speed {
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        Box::new(session.speed)
    }
}
