use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct TrackId(u32);

impl EmittableEvent for TrackId {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && session.processed_slow
    }

    fn get_event(&self, session: &SessionData) -> Vec<u8> {
        rmp_serde::to_vec(&session.track_id).unwrap()
    }
}
