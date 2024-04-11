//! # Day
//!
//! Utilities for handling days in Macaw.

use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A day. This is the amount of days since a save was created.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Day(u64);

impl Day {
    /// Creates a new `Day` with the given value.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the current day value.
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Add<u64> for Day {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u64> for Day {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl Sub<u64> for Day {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<u64> for Day {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}
