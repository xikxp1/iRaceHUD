use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;
use crate::util::format_delta::format_delta;

#[derive(Default, Type, Serialize)]
pub struct DeltaOptimalTime(String);

impl EmittableEvent for DeltaOptimalTime {
    fn get_event(&self, session: &SessionData) -> Vec<u8> {
        rmp_serde::to_vec(&format_delta(&session.delta_optimal_time)).unwrap()
    }
}
