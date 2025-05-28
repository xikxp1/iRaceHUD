use std::time::Duration;

#[derive(Copy, Clone, Default)]
pub struct SignedDuration {
    is_positive: bool,
    duration: Duration,
}

impl SignedDuration {
    pub const ZERO: SignedDuration = SignedDuration {
        is_positive: true,
        duration: Duration::ZERO,
    };

    pub const MAX: SignedDuration = SignedDuration {
        is_positive: true,
        duration: Duration::MAX,
    };

    pub fn from_secs_f64(secs: f64) -> Self {
        Self {
            is_positive: secs >= 0.0,
            duration: Duration::from_secs_f64(secs.abs()),
        }
    }

    pub fn from_secs_f32(secs: f32) -> Self {
        Self {
            is_positive: secs >= 0.0,
            duration: Duration::from_secs_f32(secs.abs()),
        }
    }

    pub fn as_secs_f32(&self) -> f32 {
        if self.is_positive {
            self.duration.as_secs_f32()
        } else {
            -self.duration.as_secs_f32()
        }
    }

    pub fn as_abs_secs_f32(&self) -> f32 {
        self.duration.as_secs_f32()
    }

    pub fn as_secs_f64(&self) -> f64 {
        if self.is_positive {
            self.duration.as_secs_f64()
        } else {
            -self.duration.as_secs_f64()
        }
    }

    pub fn as_secs(&self) -> i64 {
        if self.is_positive {
            self.duration.as_secs() as i64
        } else {
            -(self.duration.as_secs() as i64)
        }
    }

    pub fn subsec_millis(&self) -> u32 {
        self.duration.subsec_millis()
    }

    pub fn is_positive(&self) -> bool {
        self.is_positive
    }

    pub fn is_zero(&self) -> bool {
        self.duration == Duration::ZERO
    }
}

impl std::fmt::Debug for SignedDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}.{:03}",
            if self.is_positive { "+" } else { "-" },
            self.as_secs(),
            self.subsec_millis()
        )
    }
}

impl std::fmt::Display for SignedDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}.{:03}",
            if self.is_positive { "+" } else { "-" },
            self.as_secs(),
            self.subsec_millis()
        )
    }
}

impl std::ops::Add for SignedDuration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let value = self.as_secs_f32() + rhs.as_secs_f32();
        Self {
            is_positive: value >= 0.0,
            duration: Duration::from_secs_f32(value.abs()),
        }
    }
}

impl std::ops::Sub for SignedDuration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let value = self.as_secs_f32() - rhs.as_secs_f32();
        Self {
            is_positive: value >= 0.0,
            duration: Duration::from_secs_f32(value.abs()),
        }
    }
}

impl std::ops::Mul for SignedDuration {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = self.as_secs_f32() * rhs.as_secs_f32();
        Self {
            is_positive: value >= 0.0,
            duration: Duration::from_secs_f32(value.abs()),
        }
    }
}

impl std::ops::Mul<f32> for SignedDuration {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let value = self.as_secs_f32() * rhs;
        Self {
            is_positive: value >= 0.0,
            duration: Duration::from_secs_f32(value.abs()),
        }
    }
}

impl std::cmp::PartialEq for SignedDuration {
    fn eq(&self, other: &Self) -> bool {
        self.as_secs_f32() == other.as_secs_f32()
    }
}

impl std::ops::Neg for SignedDuration {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            is_positive: !self.is_positive,
            duration: self.duration,
        }
    }
}

impl std::cmp::PartialOrd for SignedDuration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_secs_f32().partial_cmp(&other.as_secs_f32())
    }
}
