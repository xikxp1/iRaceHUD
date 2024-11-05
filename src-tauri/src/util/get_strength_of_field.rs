use std::f32::consts::LN_2;

use crate::session::session_data::SessionData;

static BR1: f32 = 1600. / LN_2;

pub fn get_strength_of_field(session: &SessionData) -> u32 {
    let sum_of_exp = session
        .drivers
        .values()
        .map(|driver| (-(driver.irating as f32) / BR1).exp())
        .sum::<f32>();
    (BR1 * (session.positions_total as f32 / sum_of_exp).ln()) as u32
}
