use serde::{Serialize, Serializer};
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, PartialEq)]
pub struct TelemetryReference {
    lap_dist: u32, // in cm
    throttle: u32,
    brake: u32,
}

// Custom serialization to ensure we get a MessagePack map/object
impl Serialize for TelemetryReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("lap_dist", &self.lap_dist)?;
        map.serialize_entry("throttle", &self.throttle)?;
        map.serialize_entry("brake", &self.brake)?;
        map.end()
    }
}

impl EmittableEvent for TelemetryReference {
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        let telemetry = TelemetryReference {
            lap_dist: session.lap_dist,
            throttle: session.throttle,
            brake: session.brake,
        };
        Box::new(telemetry)
    }
}
