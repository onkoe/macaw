//! # Tick
//!
//! A module that handles game ticks.

use std::ops::Add;

use super::day::Day;

/// A game tick. These power various game systems.
#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct GameTick {
    /// The current tick value. There are 20 ticks per second, and 24000 ticks
    /// per day.
    id: u16,
    /// The day value this tick belongs to. Each day lasts 20 minutes.
    day: Day,
}

impl GameTick {
    const TICKS_PER_DAY: u16 = 24_000;
    const TICKS_PER_SECOND: u8 = 20;

    /// Creates a new `GameTick` with the given tick and day values.
    pub fn new(id: u16, day: u64) -> Self {
        Self {
            id,
            day: Day::new(day),
        }
    }

    /// Returns the current tick value.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the current day.
    pub fn day(&self) -> Day {
        self.day
    }
}

impl Add<u16> for GameTick {
    type Output = Self;

    /// Adds to the tick's value, wrapping around to another day when necessary.
    /// ```
    /// # use shared::time::{Day, GameTick};
    /// #
    /// let mut tick = GameTick::new(0, 0);
    /// tick += 49_000;
    ///
    /// assert_eq!(tick.id(), 1_000);
    /// assert_eq!(tick.day(), Day::new(2));
    /// ```
    fn add(self, rhs: u16) -> Self::Output {
        let id = self.id + rhs;

        Self {
            id: id % 24000,
            day: Day::new((id / 24000) as u64),
        }
    }
}
