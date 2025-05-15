use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;
use crate::util::format_delta::format_delta;

#[derive(Default, Type, Serialize)]
pub struct DeltaLastTime(String);

impl EmittableEvent for DeltaLastTime {
    fn get_event(&self, session: &SessionData) -> Vec<u8> {
        rmp_serde::to_vec(&format_delta(&session.delta_last_time)).unwrap()
    }
}
