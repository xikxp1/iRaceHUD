use log::error;

use crate::WS_SERVER;

use super::ws_event::WsEvent;

pub fn emit_overlay_locked_unlocked(locked: bool) {
    let ws_server = match WS_SERVER.get() {
        Some(ws_server) => ws_server,
        None => {
            error!("WebSocket server not initialized");
            return;
        }
    };
    let event_name = "overlay_locked_unlocked";
    let ws_event = WsEvent {
        event: event_name,
        data: &Box::new(locked),
    };
    ws_server.broadcast(&ws_event);
}
