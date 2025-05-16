use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;
use crate::util::get_gap::get_gap;

#[derive(Default, Type, Serialize)]
pub struct GapPrev(String);

impl EmittableEvent for GapPrev {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && !session.drivers.is_empty()
    }

    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        let gap = get_gap(session.position + 1, session, false);
        Box::new(gap)
    }
}
