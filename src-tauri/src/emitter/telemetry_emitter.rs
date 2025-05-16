use std::{
    collections::{HashMap, HashSet}, str::FromStr, sync::Arc
};

use eyre::Result;
use log::{error, info};
use serde::Serialize;
use strum::IntoEnumIterator;
use tokio::sync::OnceCell;

use crate::{
    emitter::emittable_event::{EmittableEvent, TelemetryEvent, EmittableValue},
    session::session_data::SessionData,
    websocket::WebSocketServer,
};

static WS_SERVER: OnceCell<Arc<WebSocketServer>> = OnceCell::const_new();

#[derive(Serialize)]
struct WsEvent<'a> {
    event: &'a str,
    data: &'a dyn EmittableValue,
}

#[derive(Default)]
pub struct TelemetryEmitter {
    latest_events: HashMap<String, Box<dyn EmittableValue>>,
    registered_events: HashSet<String>,
    forced_events: HashSet<String>,
}

impl TelemetryEmitter {
    pub async fn init() {
        let server = Arc::new(WebSocketServer::new());
        let server_clone = server.clone();

        tokio::spawn(async move {
            server_clone.run("127.0.0.1:8384").await;
        });

        if let Err(e) = WS_SERVER.set(server) {
            error!("Failed to initialize WebSocket server: {:?}", e);
        } else {
            info!("WebSocket server initialized");
        }
    }

    pub fn emit_all(&mut self, session: &SessionData) -> Result<()> {
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
                if let Some(ws_server) = WS_SERVER.get() {
                    let ws_event = WsEvent {
                        event: event.as_str(),
                        data: event_data.as_ref(),
                    };
                    ws_server.broadcast(&ws_event);
                }

                self.latest_events.insert(event.to_string(), event_data);
                self.forced_events.remove(event);
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
}
