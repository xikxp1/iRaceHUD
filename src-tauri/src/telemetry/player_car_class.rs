use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct PlayerCarClass(String);

impl EmittableEvent for PlayerCarClass {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && !session.player_car_class_name.is_empty()
    }

    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        Box::new(session.player_car_class_name.clone())
    }
}
