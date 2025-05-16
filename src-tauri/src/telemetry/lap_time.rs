use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct LapTime(f64);

impl EmittableEvent for LapTime {
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        let lap_time = if session.lap_time.is_positive() {
            session.lap_time.as_secs_f64()
        } else {
            0.0
        };
        Box::new(lap_time)
    }
}
