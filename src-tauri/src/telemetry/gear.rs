use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct Gear(String);

impl EmittableEvent for Gear {
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        let gear_str = match session.gear {
            -1 => "R".to_string(),
            0 => "N".to_string(),
            value => value.to_string(),
        };
        Box::new(gear_str)
    }
}
