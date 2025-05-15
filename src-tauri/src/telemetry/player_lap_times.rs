use serde::{Serialize, Serializer};
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;
use crate::util::format_laptime::format_laptime;
use crate::util::signed_duration::SignedDuration;

const MAX_LAP_TIMES: usize = 5;

#[derive(Default, Type)]
pub struct PlayerLapTimesData {
    lap: u32,
    lap_time: String,
}

impl Serialize for PlayerLapTimesData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("lap", &self.lap)?;
        map.serialize_entry("lap_time", &self.lap_time)?;
        map.end()
    }
}

#[derive(Default, Type)]
pub struct PlayerLapTimes(Vec<PlayerLapTimesData>);

impl Serialize for PlayerLapTimes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl PlayerLapTimesData {
    pub fn new(lap: u32, lap_time: SignedDuration) -> Self {
        PlayerLapTimesData {
            lap,
            lap_time: format_laptime(lap_time),
        }
    }
}

impl EmittableEvent for PlayerLapTimes {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active && session.processed_slow
    }

    fn get_event(&self, session: &SessionData) -> Vec<u8> {
        let lap_times: Vec<PlayerLapTimesData> = session
            .player_lap_times
            .iter()
            .take(MAX_LAP_TIMES)
            .map(|lap_time| PlayerLapTimesData::new(lap_time.lap(), lap_time.lap_time()))
            .collect();
        rmp_serde::to_vec(&lap_times).unwrap_or_default()
    }
}
