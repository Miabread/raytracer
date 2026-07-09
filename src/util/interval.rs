use std::ops::Add;

use rand::{RngExt, rng};

pub const fn interval(min: f64, max: f64) -> Interval {
    Interval::new(min, max)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Interval = interval(f64::INFINITY, f64::NEG_INFINITY);
    pub const FULL: Interval = interval(f64::NEG_INFINITY, f64::INFINITY);
    pub const UNIT: Interval = interval(0.0, 1.0);
    pub const HALF: Interval = interval(0.0, 0.5);
    pub const DIAM: Interval = interval(-1.0, 1.0);

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    pub fn join(&self, rhs: Interval) -> Self {
        Self::new(
            if self.min <= rhs.min {
                self.min
            } else {
                rhs.min
            },
            // Here lied a bug that took an hour
            if self.max >= rhs.max {
                self.max
            } else {
                rhs.max
            },
        )
    }

    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    pub const fn contains(&self, a: f64) -> bool {
        self.min <= a && a <= self.max
    }

    pub const fn surrounds(&self, a: f64) -> bool {
        self.min < a && a < self.max
    }

    pub fn clamp(&self, a: f64) -> f64 {
        if a < self.min {
            self.min
        } else if a > self.max {
            self.max
        } else {
            a
        }
    }

    pub fn random_double(&self) -> f64 {
        rng().random_range(self.min..=self.max)
    }

    pub fn random_integer(&self) -> usize {
        rng().random_range(self.min..=self.max).floor() as usize
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Add<f64> for Interval {
    type Output = Interval;

    fn add(self, rhs: f64) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self
    }
}
