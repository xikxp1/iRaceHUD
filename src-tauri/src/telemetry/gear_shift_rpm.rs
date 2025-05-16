use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct GearShiftRpm(u32);

impl EmittableEvent for GearShiftRpm {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && session.processed_slow
    }

    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        Box::new(session.gear_shift_rpm)
    }
}
