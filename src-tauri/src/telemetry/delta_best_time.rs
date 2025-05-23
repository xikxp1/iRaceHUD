use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;
use crate::util::format_delta::format_delta;

#[derive(Default, Type, Serialize)]
pub struct DeltaBestTime(String);

impl EmittableEvent for DeltaBestTime {
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        Box::new(format_delta(&session.delta_best_time))
    }
}
