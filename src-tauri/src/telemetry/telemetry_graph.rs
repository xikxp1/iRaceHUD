use serde::{Serialize, Serializer};
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, PartialEq)]
pub struct TelemetryGraph {
    ts: f64,
    throttle: u32,
    brake: u32,
    abs_active: bool,
}

// Custom serialization to ensure we get a MessagePack map/object
impl Serialize for TelemetryGraph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("ts", &self.ts)?;
        map.serialize_entry("throttle", &self.throttle)?;
        map.serialize_entry("brake", &self.brake)?;
        map.serialize_entry("abs_active", &self.abs_active)?;
        map.end()
    }
}

impl EmittableEvent for TelemetryGraph {
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        let telemetry = TelemetryGraph {
            ts: session.session_time.as_secs_f64(),
            throttle: session.throttle,
            brake: session.brake,
            abs_active: session.abs_active,
        };
        Box::new(telemetry)
    }
}
