use serde::Serialize;
use specta::Type;

use crate::emitter::emittable_event::EmittableEvent;
use crate::session::session_data::SessionData;

#[derive(Default, Type, Serialize)]
pub struct SessionTime(String);

impl EmittableEvent for SessionTime {
    fn get_event(&self, session: &SessionData) -> Vec<u8> {
        let ss = session.session_time.as_secs();
        let (hh, ss) = (ss / 3600, ss % 3600);
        let (mm, ss) = (ss / 60, ss % 60);
        rmp_serde::to_vec(&format!("{:0>2}:{:02}:{:02}", hh, mm, ss)).unwrap()
    }
}
