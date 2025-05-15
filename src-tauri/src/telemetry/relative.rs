use serde::{Serialize, Serializer};
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::driver::Driver;
use crate::session::session_data::SessionData;
use crate::util::format_irating::format_irating;
use crate::util::get_relative_gap::get_relative_gap;

const RELATIVE_DRIVERS_BEFORE: usize = 3;
const RELATIVE_DRIVERS_AFTER: usize = 3;

#[derive(Default, Type, Clone)]
pub struct RelativeDriver {
    car_id: u32,
    position: u32,
    user_name: String,
    car_number: String,
    irating: String,
    license: String,
    player_relative_gap: String,
    is_player: bool,
    is_in_pits: bool,
    is_off_track: bool,
    is_off_world: bool,
}

impl Serialize for RelativeDriver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(11))?;
        map.serialize_entry("car_id", &self.car_id)?;
        map.serialize_entry("position", &self.position)?;
        map.serialize_entry("user_name", &self.user_name)?;
        map.serialize_entry("car_number", &self.car_number)?;
        map.serialize_entry("irating", &self.irating)?;
        map.serialize_entry("license", &self.license)?;
        map.serialize_entry("player_relative_gap", &self.player_relative_gap)?;
        map.serialize_entry("is_player", &self.is_player)?;
        map.serialize_entry("is_in_pits", &self.is_in_pits)?;
        map.serialize_entry("is_off_track", &self.is_off_track)?;
        map.serialize_entry("is_off_world", &self.is_off_world)?;
        map.end()
    }
}

impl RelativeDriver {
    pub fn new(driver: &Driver, session: &SessionData) -> Self {
        Self {
            car_id: driver.car_id,
            position: driver.position,
            user_name: driver.user_name.clone(),
            car_number: driver.car_number.clone(),
            irating: format_irating(driver.irating),
            license: driver.lic_string.clone(),
            player_relative_gap: get_relative_gap(driver.position, session),
            is_player: driver.is_player,
            is_in_pits: driver.is_in_pits,
            is_off_track: driver.is_off_track,
            is_off_world: driver.is_off_world,
        }
    }
}

#[derive(Default, Type)]
pub struct Relative(Vec<RelativeDriver>);

impl Serialize for Relative {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl EmittableEvent for Relative {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && !session.drivers.is_empty()
    }

    fn get_event(&self, session: &SessionData) -> Vec<u8> {
        let mut drivers: Vec<Driver> = session
            .drivers
            .values()
            .filter(|driver| driver.is_player || !driver.is_off_world)
            .cloned()
            .collect();
        drivers.sort_by(|a, b| {
            a.player_relative_gap
                .partial_cmp(&b.player_relative_gap)
                .unwrap()
        });
        let player_idx = drivers
            .iter()
            .enumerate()
            .find(|(_, driver)| driver.is_player)
            .unwrap()
            .0;
        let mut result: Vec<RelativeDriver> =
            vec![RelativeDriver::default(); RELATIVE_DRIVERS_BEFORE + RELATIVE_DRIVERS_AFTER + 1];
        for idx in 0..RELATIVE_DRIVERS_BEFORE {
            if player_idx <= idx {
                break;
            }
            let driver = drivers.get(player_idx - idx - 1);
            let value = match driver {
                Some(driver) => RelativeDriver::new(driver, session),
                None => RelativeDriver::default(),
            };
            result[RELATIVE_DRIVERS_BEFORE - idx - 1] = value;
        }
        let player = drivers.get(player_idx);
        let value = match player {
            Some(driver) => RelativeDriver::new(driver, session),
            None => RelativeDriver::default(),
        };
        result[RELATIVE_DRIVERS_BEFORE] = value;
        for idx in 0..RELATIVE_DRIVERS_AFTER {
            let relative_idx = player_idx + idx + 1;
            if relative_idx >= drivers.len() {
                break;
            };
            let driver = drivers.get(relative_idx);
            let value = match driver {
                Some(driver) => RelativeDriver::new(driver, session),
                None => RelativeDriver::default(),
            };
            result[RELATIVE_DRIVERS_BEFORE + idx + 1] = value;
        }
        rmp_serde::to_vec(&result).unwrap_or_default()
    }
}
