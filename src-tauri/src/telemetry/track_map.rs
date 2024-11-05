use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::driver::Driver;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct TrackMapDriver {
    car_id: u32,
    position: u32,
    is_leader: bool,
    is_player: bool,
    lap_dist_pct: f32,
    is_in_pits: bool,
    is_off_track: bool,
    is_off_world: bool,
}

impl TrackMapDriver {
    pub fn new(driver: &Driver) -> Self {
        TrackMapDriver {
            car_id: driver.car_id,
            position: driver.position,
            is_leader: driver.is_leader,
            is_player: driver.is_player,
            lap_dist_pct: driver.lap_dist_pct,
            is_in_pits: driver.is_in_pits,
            is_off_track: driver.is_off_track,
            is_off_world: driver.is_off_world,
        }
    }
}

#[derive(Default, Type, Serialize)]
pub struct TrackMap(Vec<TrackMapDriver>);

impl EmittableEvent for TrackMap {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && !session.drivers.is_empty()
    }

    fn get_event(&self, session: &SessionData) -> Value {
        let drivers = session
            .drivers
            .values()
            .map(TrackMapDriver::new)
            .map(|driver| serde_json::to_value(driver).unwrap())
            .collect();
        Value::Array(drivers)
    }
}
