use serde::{Serialize, Serializer};
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, PartialEq)]
pub struct Proximity {
    is_left: bool,
    is_right: bool,
}

impl Serialize for Proximity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("is_left", &self.is_left)?;
        map.serialize_entry("is_right", &self.is_right)?;
        map.end()
    }
}

impl EmittableEvent for Proximity {
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        let proximity = Proximity {
            is_left: session.is_left,
            is_right: session.is_right,
        };
        Box::new(proximity)
    }
}
