use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct LongDuration {
    seconds: u64
}

impl LongDuration {
    pub fn new_minutes(minutes: u64) -> Self {
        Self { seconds: minutes*60 }
    }

    pub fn new_seconds(seconds: u64) -> Self {
        Self { seconds }
    }

    pub fn minutes(&self) -> u64 {
        self.seconds / 60
    }

    pub fn seconds(&self) -> u64 {
        self.seconds
    }
}

impl Add for &LongDuration {
    type Output = LongDuration;

    fn add(self, rhs: Self) -> Self::Output {
        LongDuration::new_seconds(self.seconds() + rhs.seconds())
    }
}

impl Sub for &LongDuration {
    type Output = LongDuration;

    fn sub(self, rhs: Self) -> Self::Output {
        LongDuration::new_seconds(self.seconds() - rhs.seconds())
    }
}

