use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;
use crate::util::format_laptime::format_laptime;
use crate::util::signed_duration::SignedDuration;

const MAX_LAP_TIMES: usize = 5;

#[derive(Default, Type, Serialize)]
pub struct PlayerLapTimesData {
    lap: u32,
    lap_time: String,
}

#[derive(Default, Type, Serialize)]
pub struct PlayerLapTimes(Vec<PlayerLapTimesData>);

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

    fn get_event(&self, session: &SessionData) -> Value {
        let lap_times = session
            .player_lap_times
            .iter()
            .take(MAX_LAP_TIMES)
            .map(|lap_time| PlayerLapTimesData::new(lap_time.lap(), lap_time.lap_time()))
            .map(|lap_time| serde_json::to_value(lap_time).unwrap())
            .collect();
        Value::Array(lap_times)
    }
}
