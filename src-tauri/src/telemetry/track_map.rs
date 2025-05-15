use serde::{Serialize, Serializer as SerdeSerializer};
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::driver::Driver;
use crate::session::session_data::SessionData;

#[derive(Default, Type)]
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

// Custom serialization to ensure we get a MessagePack map/object instead of an array
impl Serialize for TrackMapDriver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: SerdeSerializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("car_id", &self.car_id)?;
        map.serialize_entry("position", &self.position)?;
        map.serialize_entry("is_leader", &self.is_leader)?;
        map.serialize_entry("is_player", &self.is_player)?;
        map.serialize_entry("lap_dist_pct", &self.lap_dist_pct)?;
        map.serialize_entry("is_in_pits", &self.is_in_pits)?;
        map.serialize_entry("is_off_track", &self.is_off_track)?;
        map.serialize_entry("is_off_world", &self.is_off_world)?;
        map.end()
    }
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

    fn get_event(&self, session: &SessionData) -> Vec<u8> {
        let drivers: Vec<TrackMapDriver> = session
            .drivers
            .values()
            .map(TrackMapDriver::new)
            .collect();

        // Serialize the vector of drivers directly
        rmp_serde::to_vec(&drivers).unwrap_or_default()
    }
}
