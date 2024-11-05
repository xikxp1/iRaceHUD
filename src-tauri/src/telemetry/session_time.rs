use serde::Serialize;
use serde_json::Value;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct SessionTime(String);

impl EmittableEvent for SessionTime {
    fn get_event(&self, session: &SessionData) -> Value {
        let ss = session.session_time.as_secs();
        let (hh, ss) = (ss / 3600, ss % 3600);
        let (mm, ss) = (ss / 60, ss % 60);
        Value::String(format!("{:0>2}:{:02}:{:02}", hh, mm, ss))
    }
}
