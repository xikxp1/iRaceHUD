use log::error;

use crate::WS_SERVER;

use super::ws_event::WsEvent;

pub fn emit_settings_update(overlay_name: &str) {
    let ws_server = match WS_SERVER.get() {
        Some(ws_server) => ws_server,
        None => {
            error!("WebSocket server not initialized");
            return;
        }
    };
    let event_name = format!("{}_overlay_settings_changed", overlay_name);
    let ws_event = WsEvent {
        event: event_name.as_str(),
        data: &Box::new(event_name.clone()),
    };
    ws_server.broadcast(&ws_event);
}
