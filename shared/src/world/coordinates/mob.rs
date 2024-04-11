//! # Mob
//!
//! A coordinate type having to do with mobs. Holds exact coordinates for
//! proper mob location.

use bevy::math::Vec3;
use fraction::Decimal;

use super::Coordinate;

/// A coordinate holding fractional values for mobs.
struct MobCoordinate {
    x: Decimal,
    y: Decimal,
    z: Decimal,
}

impl MobCoordinate {
    /// Creates a new `MobCoordinate` from given `Decimal`s.
    pub fn new(x: Decimal, y: Decimal, z: Decimal) -> Self {
        Self { x, y, z }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x.try_into().unwrap(),
            y: self.y.try_into().unwrap(),
            z: self.z.try_into().unwrap(),
        }
    }
}

impl Coordinate for MobCoordinate {
    type Value = Decimal;

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
