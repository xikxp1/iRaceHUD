use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;
use crate::util::get_gap::get_gap;

#[derive(Default, Type, Serialize)]
pub struct GapNext(String);

impl EmittableEvent for GapNext {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active
            && !session.drivers.is_empty()
            && !session.driver_positions.is_empty()
            && !session.player_class_driver_positions.is_empty()
            && session.class_position > 0
    }

    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        let gap = get_gap(session.class_position - 1, session, false);
        Box::new(gap)
    }
}
