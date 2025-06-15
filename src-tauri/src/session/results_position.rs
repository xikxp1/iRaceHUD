use crate::util::signed_duration::SignedDuration;

#[derive(Default, Debug)]
pub struct ResultsPosition {
    pub car_id: u32,
    pub position: u32,
    pub class_position: u32,
    pub fastest_time: SignedDuration,
    pub last_time: SignedDuration,
    pub reason_out_id: u32,
}
