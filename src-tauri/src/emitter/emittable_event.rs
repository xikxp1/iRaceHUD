use enum_dispatch::enum_dispatch;
use serde::Serialize;
use std::any::Any;
use strum_macros::{Display, EnumIter, EnumString};

use crate::session::session_data::SessionData;
use crate::telemetry::active::Active;
use crate::telemetry::current_time::CurrentTime;
use crate::telemetry::delta_best_time::DeltaBestTime;
use crate::telemetry::delta_last_time::DeltaLastTime;
use crate::telemetry::gap_next::GapNext;
use crate::telemetry::gap_prev::GapPrev;
use crate::telemetry::gear::Gear;
use crate::telemetry::gear_blink_rpm::GearBlinkRpm;
use crate::telemetry::gear_shift_rpm::GearShiftRpm;
use crate::telemetry::incident_limit::IncidentLimit;
use crate::telemetry::incidents::Incidents;
use crate::telemetry::lap::Lap;
use crate::telemetry::lap_time::LapTime;
use crate::telemetry::laps_total::LapsTotal;
use crate::telemetry::player_car_class::PlayerCarClass;
use crate::telemetry::player_lap_times::PlayerLapTimes;
use crate::telemetry::position::Position;
use crate::telemetry::positions_total::PositionsTotal;
use crate::telemetry::proximity::Proximity;
use crate::telemetry::race_laps::RaceLaps;
use crate::telemetry::relative::Relative;
use crate::telemetry::rpm::Rpm;
use crate::telemetry::session_state::SessionState;
use crate::telemetry::session_time::SessionTime;
use crate::telemetry::session_time_total::SessionTimeTotal;
use crate::telemetry::speed::Speed;
use crate::telemetry::standings::Standings;
use crate::telemetry::strength_of_field::StrengthOfField;
use crate::telemetry::telemetry_graph::TelemetryGraph;
use crate::telemetry::track_id::TrackId;
use crate::telemetry::track_map::TrackMap;

#[enum_dispatch(TelemetryEvent)]
pub trait EmittableEvent {
    fn is_ready(&self, session: &SessionData) -> bool {
        session.active
    }

    fn get_event(&self, session: &SessionData) -> Box<dyn EmittableValue>;

    fn is_forced(&self) -> bool {
        false
    }
}

pub trait EmittableValue: erased_serde::Serialize + Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, other: &dyn EmittableValue) -> bool;
}

impl<T: Serialize + PartialEq + Send + Sync + 'static> EmittableValue for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn EmittableValue) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            self == other
        } else {
            false
        }
    }
}

erased_serde::serialize_trait_object!(EmittableValue);

#[derive(EnumString, EnumIter, Display)]
#[strum(serialize_all = "snake_case")]
#[enum_dispatch]
pub enum TelemetryEvent {
    Active(Active),
    CurrentTime(CurrentTime),
    DeltaLastTime(DeltaLastTime),
    DeltaBestTime(DeltaBestTime),
    GapNext(GapNext),
    GapPrev(GapPrev),
    Gear(Gear),
    GearBlinkRpm(GearBlinkRpm),
    GearShiftRpm(GearShiftRpm),
    IncidentLimit(IncidentLimit),
    Incidents(Incidents),
    Lap(Lap),
    LapTime(LapTime),
    LapsTotal(LapsTotal),
    PlayerLapTimes(PlayerLapTimes),
    Position(Position),
    PositionsTotal(PositionsTotal),
    Proximity(Proximity),
    RaceLaps(RaceLaps),
    Relative(Relative),
    Rpm(Rpm),
    SessionState(SessionState),
    SessionTime(SessionTime),
    SessionTimeTotal(SessionTimeTotal),
    Speed(Speed),
    Standings(Standings),
    StrengthOfField(StrengthOfField),
    TelemetryGraph(TelemetryGraph),
    TrackId(TrackId),
    TrackMap(TrackMap),
    PlayerCarClass(PlayerCarClass),
}
