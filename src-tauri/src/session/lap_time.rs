use crate::util::signed_duration::SignedDuration;

#[derive(Default, Debug)]
pub struct LapTime {
    lap: u32,
    lap_time: SignedDuration,
}

impl LapTime {
    pub fn new(lap: u32, lap_time: SignedDuration) -> Self {
        Self { lap, lap_time }
    }

    pub fn lap(&self) -> u32 {
        self.lap
    }

    pub fn lap_time(&self) -> SignedDuration {
        self.lap_time
    }
}
