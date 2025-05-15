use serde::{Serialize, Serializer};
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::driver::Driver;
use crate::session::session_data::SessionData;
use crate::util::format_irating::format_irating;
use crate::util::format_laptime::format_laptime;
use crate::util::get_gap::get_gap;
use crate::util::session_type::SessionType;

#[derive(Default, Type)]
pub struct StandingsDriver {
    car_id: u32,
    position: u32,
    user_name: String,
    car_number: String,
    irating: String,
    license: String,
    leader_gap: String,
    best_lap: String,
    last_lap: String,
    is_player: bool,
    is_leader: bool,
    is_in_pits: bool,
}

impl Serialize for StandingsDriver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(12))?;
        map.serialize_entry("car_id", &self.car_id)?;
        map.serialize_entry("position", &self.position)?;
        map.serialize_entry("user_name", &self.user_name)?;
        map.serialize_entry("car_number", &self.car_number)?;
        map.serialize_entry("irating", &self.irating)?;
        map.serialize_entry("license", &self.license)?;
        map.serialize_entry("leader_gap", &self.leader_gap)?;
        map.serialize_entry("best_lap", &self.best_lap)?;
        map.serialize_entry("last_lap", &self.last_lap)?;
        map.serialize_entry("is_player", &self.is_player)?;
        map.serialize_entry("is_leader", &self.is_leader)?;
        map.serialize_entry("is_in_pits", &self.is_in_pits)?;
        map.end()
    }
}

#[derive(Default, Type)]
pub struct Standings(Vec<StandingsDriver>);

impl Serialize for Standings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl EmittableEvent for Standings {
    fn is_ready(&self, session: &SessionData) -> bool {
        // TODO: qualify implementation
        session.active
            && session.session_type == SessionType::Race
            && !session.drivers.is_empty()
            && session.processed_slow
    }

    fn get_event(&self, session: &SessionData) -> Vec<u8> {
        let mut drivers = session.drivers.values().cloned().collect::<Vec<Driver>>();
        drivers.sort_by(|a, b| a.position.cmp(&b.position));
        let drivers = drivers
            .iter()
            .map(|driver| StandingsDriver {
                car_id: driver.car_id,
                position: driver.position,
                user_name: driver.user_name.clone(),
                car_number: driver.car_number.clone(),
                irating: format_irating(driver.irating),
                license: driver.lic_string.clone(),
                leader_gap: get_gap(driver.position, session, true),
                best_lap: format_laptime(driver.best_lap_time),
                last_lap: format_laptime(driver.last_lap_time),
                is_player: driver.is_player,
                is_leader: driver.is_leader,
                is_in_pits: driver.is_in_pits,
            }).collect::<Vec<StandingsDriver>>();
        rmp_serde::to_vec(&drivers).unwrap_or_default()
    }

    fn is_forced(&self) -> bool {
        true
    }
}
