use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;
use crate::util::format_delta::format_delta;

#[derive(Default, Type, Serialize)]
pub struct DeltaOptimalTime(String);

impl EmittableEvent for DeltaOptimalTime {
    fn get_event(&self, session: &SessionData) -> Value {
        Value::String(format_delta(&session.delta_optimal_time))
    }
}
