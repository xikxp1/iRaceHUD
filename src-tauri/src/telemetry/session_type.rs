use serde::Serialize;
use specta::Type;

use crate::{
    emitter::emittable_event::{EmittableEvent, EmittableValue},
    session::session_data::SessionData,
};

#[derive(Default, Type, Serialize)]
pub struct SessionType(String);

impl EmittableEvent for SessionType {
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        Box::new(session.session_type.to_string())
    }
}
