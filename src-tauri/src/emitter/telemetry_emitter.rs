use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use eyre::Result;
use log::{error, info};
use strum::IntoEnumIterator;

use crate::{
    WS_SERVER,
    emitter::emittable_event::{EmittableEvent, EmittableValue, TelemetryEvent},
    session::session_data::SessionData,
};

use super::ws_event::WsEvent;

const TELEMETRY_RECORDING_START_DISTANCE: u32 = 400; // 4m

#[derive(Default, PartialEq, Clone, Copy)]
pub enum TelemetryRecordingState {
    #[default]
    NotRecording,
    WaitingForStart,
    InProgress,
    Finished,
}

#[derive(Default)]
pub struct TelemetryEmitter {
    latest_events: HashMap<String, Box<dyn EmittableValue>>,
    registered_events: HashSet<String>,
    forced_events: HashSet<String>,
    recording_state: TelemetryRecordingState,
    recording_last_lap_dist: u32,
}

impl TelemetryEmitter {
    pub async fn emit_all(&mut self, session: &SessionData) -> Result<()> {
        let mut should_start_recording = false;
        let mut should_record = false;
        let mut should_stop_recording = false;
        if self.recording_state == TelemetryRecordingState::WaitingForStart
            && session.lap_dist <= TELEMETRY_RECORDING_START_DISTANCE
        {
            self.recording_state = TelemetryRecordingState::InProgress;
            self.recording_last_lap_dist = session.lap_dist;
            should_start_recording = true;
            info!("Recording telemetry started");
        } else if self.recording_state == TelemetryRecordingState::InProgress
            && session.lap_dist <= TELEMETRY_RECORDING_START_DISTANCE
            && session.lap_dist < self.recording_last_lap_dist
        {
            self.recording_state = TelemetryRecordingState::Finished;
            should_stop_recording = true;
            info!("Recording telemetry finished");
        } else if self.recording_state == TelemetryRecordingState::InProgress {
            self.recording_last_lap_dist = session.lap_dist;
            should_record = true;
        }
        let ws_server = match WS_SERVER.get() {
            Some(ws_server) => ws_server,
            None => {
                error!("WebSocket server not initialized");
                return Ok(());
            }
        };
        for event in &self.registered_events {
            let telemetry_event = TelemetryEvent::from_str(event).ok();
            if telemetry_event.is_none() {
                continue;
            }
            let telemetry_event = telemetry_event.unwrap();
            if !telemetry_event.is_ready(session) {
                continue;
            }
            let event_data = telemetry_event.get_event(session);
            let latest_value = self.latest_events.get(event);

            let should_emit = telemetry_event.is_forced()
                || self.forced_events.contains(event)
                || latest_value.is_none()
                || !latest_value.is_some_and(|v| v.equals(event_data.as_ref()));

            if should_emit {
                // Emit via WebSocket if available
                let ws_event = WsEvent {
                    event: event.as_str(),
                    data: event_data.as_ref(),
                };
                ws_server.broadcast(&ws_event);

                self.latest_events.insert(event.to_string(), event_data);
                self.forced_events.remove(event);
            }

            if should_start_recording {
                telemetry_event.start_recording(session);
                telemetry_event.record(session);
            } else if should_record {
                telemetry_event.record(session);
            } else if should_stop_recording {
                telemetry_event.stop_recording(session).await;
            }
        }

        Ok(())
    }

    pub fn register(&mut self, event: &str) {
        if TelemetryEvent::from_str(event).is_err() {
            error!("Event {} is not supported", event);
            return;
        }
        self.registered_events.insert(event.to_owned());
        self.forced_events.insert(event.to_owned());
    }

    pub fn unregister(&mut self, event: &str) {
        match self.registered_events.remove(event) {
            true => {
                self.latest_events.remove(event);
                self.forced_events.remove(event);
            }
            false => {
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

    pub fn enable_recording(&mut self) {
        self.recording_state = TelemetryRecordingState::WaitingForStart;
    }

    pub fn get_recording_state(&self) -> TelemetryRecordingState {
        self.recording_state
    }
}
