use crate::util::signed_duration::SignedDuration;

#[derive(Clone, Default)]
pub struct Driver {
    pub car_id: u32,
    pub position: u32,
    pub class_position: u32,
    pub laps_completed: u32,
    pub lap_dist_pct: f32,
    pub total_completed: f32,
    pub best_lap_time: SignedDuration,
    pub last_lap_time: SignedDuration,
    pub estimated: SignedDuration,
    pub leader_gap_laps: i32,
    pub leader_gap: SignedDuration,
    pub player_gap_laps: i32,
    pub player_gap: SignedDuration,
    pub player_relative_gap: SignedDuration,
    pub user_name: String,
    pub car_number: String,
    pub car_class_id: u32,
    pub irating: u32,
    pub lic_string: String,
    pub is_player: bool,
    pub is_leader: bool,
    pub is_in_pits: bool,
    pub is_off_track: bool,
    pub is_off_world: bool,
    pub car_class_est_lap_time: SignedDuration,
}

impl Driver {
    pub fn new(
        car_id: u32,
        user_name: String,
        car_number: String,
        car_class_id: u32,
        irating: u32,
        lic_string: String,
        car_class_est_lap_time: SignedDuration,
    ) -> Self {
        Self {
            car_id,
            user_name,
            car_number,
            car_class_id,
            irating,
            lic_string,
            car_class_est_lap_time,
            ..Default::default()
        }
    }
}
