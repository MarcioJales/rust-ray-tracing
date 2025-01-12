use std::f64::{INFINITY, NEG_INFINITY};

#[derive(Copy, Clone)]
pub struct Interval(pub f64, pub f64);

const EMPTY: Interval = Interval(INFINITY, NEG_INFINITY);

const UNIVERSE: Interval = Interval(NEG_INFINITY, INFINITY);

impl Interval {
    pub fn min(self) -> f64 {
        self.0
    }

    pub fn max(self) -> f64 {
        self.1
    }

    pub fn size(self) -> f64 {
        self.1 - self.0
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.0 < x && x < self.1
    }

    pub fn contains(&self, x: f64) -> bool {
        self.0 <= x && x <= self.1
    }
}

impl Default for Interval {
    fn default() -> Self {
        EMPTY
    }
}