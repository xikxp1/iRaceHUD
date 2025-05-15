use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;
use crate::util::get_gap::get_gap;

#[derive(Default, Type, Serialize)]
pub struct GapPrev(String);

impl EmittableEvent for GapPrev {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && !session.drivers.is_empty()
    }

    fn get_event(&self, session: &SessionData) -> Vec<u8> {
        let gap = get_gap(session.position + 1, session, false);
        rmp_serde::to_vec(&gap).unwrap()
    }
}
