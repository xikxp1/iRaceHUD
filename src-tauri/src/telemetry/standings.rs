use serde::{Serialize, Serializer};
use specta::Type;
use tauri::Manager;

use crate::emitter::emittable_event::{EmittableEvent, EmittableValue};
use crate::session::driver::Driver;
use crate::session::session_data::SessionData;
use crate::settings::overlays::standings::StandingsOverlaySettings;
use crate::util::format_irating::format_irating;
use crate::util::format_laptime::format_laptime;
use crate::util::get_gap::get_gap;
use crate::util::settings_helper::get_settings;
use crate::WINDOW;

#[derive(Default, Type, PartialEq)]
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
    split_after: bool,
    is_off_world: bool,
    is_off_track: bool,
}

impl Serialize for StandingsDriver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(15))?;
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
        map.serialize_entry("split_after", &self.split_after)?;
        map.serialize_entry("is_off_world", &self.is_off_world)?;
        map.serialize_entry("is_off_track", &self.is_off_track)?;
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
            && session.position != 0
            && !session.drivers.is_empty()
            && session.processed_slow
    }

    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue> {
        let app_handle = WINDOW.get().unwrap().app_handle();
        let settings = get_settings::<StandingsOverlaySettings>(app_handle.clone(), "standings");
        let max_drivers_count = settings.max_drivers as usize;
        let top_drivers_count = settings.top_drivers as usize;

        let mut drivers = session
            .drivers
            .values()
            .filter(|driver| driver.car_class_id == session.player_car_class)
            .cloned()
            .collect::<Vec<Driver>>();
        drivers.sort_by(|a, b| a.position.cmp(&b.position));
        let player_position = session.class_position as usize;
        let mut split_after = false;

        let selected_drivers = if drivers.len() <= max_drivers_count {
            drivers
        } else {
            let mut taken_positions = 0;
            let top_drivers = drivers
                .iter()
                .take(top_drivers_count)
                .cloned()
                .collect::<Vec<Driver>>();
            taken_positions += top_drivers_count;
            let player_already_taken = player_position <= top_drivers_count;
            if !player_already_taken {
                taken_positions += 1;
            }

            let mut drivers_before: Vec<Driver> = Vec::new();
            let mut drivers_after: Vec<Driver> = Vec::new();
            let mut take_before = true;
            let mut delta: usize = 1;

            while taken_positions < max_drivers_count {
                if take_before {
                    if player_position < delta {
                        take_before = false;
                        continue;
                    }
                    let position = player_position - delta;
                    if position <= top_drivers_count {
                        take_before = false;
                        continue;
                    }
                    drivers_before.push(drivers[position - 1].clone());
                    taken_positions += 1;
                } else {
                    let position = player_position + delta;
                    if position <= top_drivers_count {
                        delta += 1;
                        continue;
                    }
                    if position > drivers.len() {
                        take_before = true;
                        delta += 1;
                        continue;
                    }
                    drivers_after.push(drivers[position - 1].clone());
                    taken_positions += 1;
                    delta += 1;
                }
                take_before = !take_before;
            }

            let mut selected_drivers = vec![];
            selected_drivers.extend(top_drivers);
            drivers_before.reverse();
            selected_drivers.extend(drivers_before);
            if !player_already_taken {
                selected_drivers.push(drivers[player_position - 1].clone());
            }
            selected_drivers.extend(drivers_after);
            split_after = selected_drivers[top_drivers_count].class_position as usize
                != top_drivers_count + 1;
            selected_drivers
        };

        let drivers = selected_drivers
            .iter()
            .map(|driver| StandingsDriver {
                car_id: driver.car_id,
                position: driver.class_position,
                user_name: driver.team_name.clone(),
                car_number: driver.car_number.clone(),
                irating: format_irating(driver.irating),
                license: driver.lic_string.clone(),
                leader_gap: get_gap(driver.position, session, true),
                best_lap: format_laptime(driver.best_lap_time),
                last_lap: format_laptime(driver.last_lap_time),
                is_player: driver.is_player,
                is_leader: driver.is_leader,
                is_in_pits: driver.is_in_pits,
                split_after: (driver.class_position as usize == top_drivers_count) && split_after,
                is_off_world: driver.is_off_world,
                is_off_track: driver.is_off_track,
            })
            .collect::<Vec<StandingsDriver>>();
        Box::new(drivers)
    }

    fn is_forced(&self) -> bool {
        true
    }
}
