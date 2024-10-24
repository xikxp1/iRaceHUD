use std::collections::HashMap;

use chrono::{DateTime, Local, TimeDelta};
use eyre::Result;
use log::error;
use serde_json::Value;
use tauri::ipc::{Channel, InvokeResponseBody};

const FORCED_EMITTER_DURATION_SECS: i64 = 5;

static TELEMETERY_EVENTS: [&str; 30] = [
    "active",
    "session_time_total",
    "laps_total",
    "incidents",
    "gear_shift_rpm",
    "gear_blink_rpm",
    "current_time",
    "session_time",
    "lap",
    "race_laps",
    "lap_time",
    "delta_last_time",
    "delta_optimal_time",
    "session_state",
    "gear",
    "speed",
    "rpm",
    "telemetry",
    "proximity",
    "position",
    "gap_next",
    "gap_prev",
    "track_map",
    "standings",
    "relative",
    "player_lap_times",
    "incident_limit",
    "track_id",
    "positions_total",
    "strength_of_field",
];

pub struct TelemetryEmitter {
    events: HashMap<String, Value>,
    activation_time: Option<DateTime<Local>>,
    forced_emitter_duration: TimeDelta,
    registered_events: HashMap<String, Channel>,
}

impl TelemetryEmitter {
    pub fn activate(&mut self, activation_time: DateTime<Local>) {
        self.activation_time = Some(activation_time);
    }

    pub fn deactivate(&mut self) {
        self.activation_time = None;
    }

    pub fn emit(&mut self, event: &str, value: Value) -> Result<()> {
        if event != "active" && self.activation_time.is_none() {
            error!("Emitter is not activated");
            return Ok(());
        }

        let mut is_forced = false;

        if let Some(activation_time) = self.activation_time {
            let current_time = Local::now();
            let elapsed = current_time.signed_duration_since(activation_time);
            if elapsed < self.forced_emitter_duration {
                is_forced = true;
            }
        }

        if !is_forced && self.events.contains_key(event) && self.events[event] == value {
            return Ok(());
        }

        let channel = match self.registered_events.get(event) {
            Some(channel) => channel,
            None => {
                return Ok(());
            }
        };
        let response_body = InvokeResponseBody::from(value.to_string());
        channel.send(response_body)?;
        self.events.insert(event.to_string(), value);
        Ok(())
    }

    pub fn register(&mut self, event: String, channel: Channel) {
        if !TELEMETERY_EVENTS.contains(&event.as_str()) {
            error!("Event {} is not supported", event);
            return;
        }
        self.registered_events.insert(event, channel);
    }

    pub fn unregister(&mut self, event: &str) {
        match self.registered_events.remove(event) {
            Some(_) => {}
            None => {
                error!("Event {} is not registered", event);
            }
        }
    }
}

impl Default for TelemetryEmitter {
    fn default() -> Self {
        TelemetryEmitter {
            events: HashMap::new(),
            activation_time: None,
            forced_emitter_duration: TimeDelta::seconds(FORCED_EMITTER_DURATION_SECS),
            registered_events: HashMap::new(),
        }
    }
}
