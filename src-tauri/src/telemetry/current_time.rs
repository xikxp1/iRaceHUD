use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct CurrentTime(String);

impl EmittableEvent for CurrentTime {
    fn is_ready(&self, _session: &SessionData) -> bool {
        true
    }
    fn get_event(&self, session: &SessionData) -> Vec<u8> {
        rmp_serde::to_vec(&session.current_time.format("%H:%M").to_string()).unwrap()
    }
}
