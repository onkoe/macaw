use std::fmt::Display;

use bevy::math::Vec3;

/// A coordinate found in the world - globally.
///
/// These can represent a block or even a chunk!
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct GlobalCoordinate {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl GlobalCoordinate {
    /// An origin coordinate, at (0, 0, 0).
    pub const ORIGIN: GlobalCoordinate = GlobalCoordinate::new(0, 0, 0);

    /// Creates a `GlobalCoordinate`.
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn to_vec3(self) -> Vec3 {
        Vec3 {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        }
    }

    /// Returns the largest coordinate value.
    ///
    /// ```
    /// # use macaw::world::coordinates::GlobalCoordinate;
    /// #
    /// let coord = GlobalCoordinate::new(-1, -2, -3);
    /// assert_eq!(coord.largest(), -1);
    pub fn largest(&self) -> i64 {
        self.x.max(self.y).max(self.z)
    }

    /// Returns the smallest coordinate value.
    ///
    ///```
    /// # use macaw::world::coordinates::GlobalCoordinate;
    /// #
    /// let coord = GlobalCoordinate::new(3, 2, 1);
    /// assert_eq!(coord.smallest(), 1);
    pub fn smallest(&self) -> i64 {
        self.x.min(self.y).min(self.z)
    }

    /// Destructures self into (x, y, z) coordinates.
    pub fn free(self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }

    /// If any of the contained coordinates match a given closure's comparison,
    /// this function will return true.
    ///
    /// Usage:
    ///
    /// ```
    /// # use macaw::world::coordinates::GlobalCoordinate;
    /// #
    /// let coord = GlobalCoordinate::new(1, 2, 3);
    ///
    /// assert!(coord.any(|a| a == 3));
    /// ```
    pub fn any<F>(&self, compare_to: F) -> bool
    where
        F: Fn(i64) -> bool,
    {
        compare_to(self.x) || compare_to(self.y) || compare_to(self.z)
    }
}

impl Display for GlobalCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = format!("{}, {}, {}", self.x, self.y, self.z);
        f.write_str(&string)
    }
}

impl super::Coordinate for GlobalCoordinate {
    type Value = i64;

    fn x(&self) -> Self::Value {
        self.x
    }

    fn y(&self) -> Self::Value {
        self.y
    }

    fn z(&self) -> Self::Value {
        self.z
    }
}
