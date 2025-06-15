use crate::util::signed_duration::SignedDuration;

#[derive(Clone, Default, Debug)]
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
    pub is_player_class: bool,
    pub car_class_color: u32,
    pub team_name: String,
    pub is_out: bool,
    pub result_position: Option<u32>,
    pub result_class_position: Option<u32>,
    pub ahead_behind: i32,
}

#[derive(Default)]
pub struct DriverBuilder {
    car_id: u32,
    user_name: String,
    car_number: String,
    car_class_id: u32,
    irating: u32,
    lic_string: String,
    car_class_est_lap_time: SignedDuration,
    is_player_class: bool,
    car_class_color: u32,
    team_name: String,
}

impl DriverBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn car_id(mut self, car_id: u32) -> Self {
        self.car_id = car_id;
        self
    }

    pub fn user_name(mut self, user_name: String) -> Self {
        self.user_name = user_name;
        self
    }

    pub fn car_number(mut self, car_number: String) -> Self {
        self.car_number = car_number;
        self
    }

    pub fn car_class_id(mut self, car_class_id: u32) -> Self {
        self.car_class_id = car_class_id;
        self
    }

    pub fn irating(mut self, irating: u32) -> Self {
        self.irating = irating;
        self
    }

    pub fn lic_string(mut self, lic_string: String) -> Self {
        self.lic_string = lic_string;
        self
    }

    pub fn car_class_est_lap_time(mut self, car_class_est_lap_time: SignedDuration) -> Self {
        self.car_class_est_lap_time = car_class_est_lap_time;
        self
    }

    pub fn is_player_class(mut self, is_player_class: bool) -> Self {
        self.is_player_class = is_player_class;
        self
    }

    pub fn car_class_color(mut self, car_class_color: u32) -> Self {
        self.car_class_color = car_class_color;
        self
    }

    pub fn team_name(mut self, team_name: String) -> Self {
        self.team_name = team_name;
        self
    }

    pub fn build(self) -> Driver {
        Driver {
            car_id: self.car_id,
            user_name: self.user_name,
            car_number: self.car_number,
            car_class_id: self.car_class_id,
            irating: self.irating,
            lic_string: self.lic_string,
            car_class_est_lap_time: self.car_class_est_lap_time,
            is_player_class: self.is_player_class,
            car_class_color: self.car_class_color,
            team_name: self.team_name,
            ..Default::default()
        }
    }
}

impl Driver {
    pub fn builder() -> DriverBuilder {
        DriverBuilder::new()
    }
}
