use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::session_data::SessionData;
use crate::util::format_laptime::format_laptime;

#[derive(Default, Type, Serialize)]
pub struct FastestLap(String);

impl EmittableEvent for FastestLap {
    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        if session.best_lap_time.is_none()
            || session.best_lap_time_car_id.is_none()
            || session.drivers.is_empty()
        {
            return Box::new("-:--:--");
        }
        let fastest_lap = format_laptime(session.best_lap_time.unwrap());
        let fastest_lap_car_id = session.best_lap_time_car_id.unwrap();
        let fastest_lap_driver_name = session
            .drivers
            .get(&fastest_lap_car_id)
            .unwrap()
            .user_name
            .clone();
        Box::new(format!("{} ({})", fastest_lap, fastest_lap_driver_name))
    }
}
