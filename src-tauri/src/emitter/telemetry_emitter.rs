use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use eyre::Result;
use log::error;
use serde_json::Value;
use strum::IntoEnumIterator;
use tauri::ipc::{Channel, InvokeResponseBody};

use crate::{
    emitter::emittable_event::{EmittableEvent, TelemetryEvent},
    session::session_data::SessionData,
};

#[derive(Default)]
pub struct TelemetryEmitter {
    latest_events: HashMap<String, Value>,
    registered_events: HashMap<String, Channel>,
    forced_events: HashSet<String>,
}

impl TelemetryEmitter {
    pub fn emit_all(&mut self, session: &SessionData) -> Result<()> {
        for (event, channel) in self.registered_events.iter() {
            let telemetry_event = match TelemetryEvent::from_str(event) {
                Ok(event) => Some(event),
                Err(_) => None,
            };
            if telemetry_event.is_none() {
                continue;
            }
            let telemetry_event = telemetry_event.unwrap();
            if !telemetry_event.is_ready(session) {
                continue;
            }
            let value = telemetry_event.get_event(session);
            let latest_value = self.latest_events.get(event);
            if !telemetry_event.is_forced()
                && !self.forced_events.contains(event)
                && latest_value.is_some()
                && latest_value.unwrap() == &value
            {
                continue;
            }
            channel.send(InvokeResponseBody::from(value.to_string()))?;
            self.latest_events.insert(event.to_string(), value);
            self.forced_events.remove(event);
        }

        Ok(())
    }

    pub fn register(&mut self, event: &str, channel: Channel) {
        if TelemetryEvent::from_str(event).is_err() {
            error!("Event {} is not supported", event);
            return;
        }
        self.registered_events.insert(event.to_owned(), channel);
        self.forced_events.insert(event.to_owned());
    }

    pub fn unregister(&mut self, event: &str) {
        match self.registered_events.remove(event) {
            Some(_) => {
                self.latest_events.remove(event);
                self.forced_events.remove(event);
            }
            None => {
                error!("Event {} is not registered", event);
            }
        }
    }

    pub fn reset(&mut self) {
        self.latest_events.clear();
        for event in TelemetryEvent::iter() {
            self.forced_events.insert(event.to_string());
        }
    }
}
