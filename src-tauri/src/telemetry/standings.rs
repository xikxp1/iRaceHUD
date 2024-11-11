use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::driver::Driver;
use crate::session::session_data::SessionData;
use crate::util::format_irating::format_irating;
use crate::util::format_laptime::format_laptime;
use crate::util::get_gap::get_gap;

#[derive(Default, Type, Serialize)]
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

#[derive(Default, Type, Serialize)]
pub struct Standings(Vec<StandingsDriver>);

impl EmittableEvent for Standings {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && !session.drivers.is_empty() && session.processed_slow
    }

    fn get_event(&self, session: &SessionData) -> Value {
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
            })
            .map(|driver| serde_json::to_value(driver).unwrap())
            .collect::<Vec<Value>>();
        Value::Array(drivers)
    }

    fn is_forced(&self) -> bool {
        true
    }
}
