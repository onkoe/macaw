use std::fmt::Display;

use bevy::math::Vec3;

/// A coordinate in a chunk. Chunks are 16x16x16, so all values must be in the
/// range [0, 15].
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct ChunkBlockCoordinate {
    x: u8,
    y: u8,
    z: u8,
}

impl ChunkBlockCoordinate {
    /// A checked constructor for `ChunkBlockCoordinate`.
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        assert!((0..16).contains(&x));
        assert!((0..16).contains(&y));
        assert!((0..16).contains(&z));

        Self { x, y, z }
    }

    pub fn new_from_tuple(xyz: (u8, u8, u8)) -> Self {
        Self::new(xyz.0, xyz.1, xyz.2)
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }

    pub fn z(&self) -> u8 {
        self.z
    }

    /// If any of the contained coordinates match a given closure's comparison,
    /// this function will return true.
    ///
    /// Usage:
    ///
    /// ```
    /// # use macaw::world::coordinates::ChunkBlockCoordinate;
    /// #
    /// let coord = ChunkBlockCoordinate::new(1, 2, 3);
    ///
    /// assert!(coord.any(|a| a == 3));
    /// ```
    pub fn any<F>(&self, compare_to: F) -> bool
    where
        F: Fn(u8) -> bool,
    {
        compare_to(self.x) || compare_to(self.y) || compare_to(self.z)
    }

    /// Destructures self into (x, y, z) coordinates.
    pub fn free(self) -> (u8, u8, u8) {
        (self.x, self.y, self.z)
    }
}

/// A coordinate found in the world - globally.
///
/// These can represent a block or even a chunk!
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct GlobalCoordinate {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl GlobalCoordinate {
    /// Creates a `GlobalCoordinate`.
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn to_vec3(self) -> Vec3 {
        Vec3 {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        }
    }
}

impl Display for GlobalCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = format!("{}, {}, {}", self.x, self.y, self.z);
        f.write_str(&string)
    }
}

pub struct GlobalCoordinate2D {
    pub x: i64,
    pub z: i64,
}

impl GlobalCoordinate2D {
    pub fn to_array(&self) -> [i64; 2] {
        [self.x, self.z]
    }

    pub fn to_f32_array(&self) -> [f32; 2] {
        [self.x as f32, self.z as f32]
    }
}

impl From<GlobalCoordinate> for GlobalCoordinate2D {
    fn from(value: GlobalCoordinate) -> Self {
        Self {
            x: value.x,
            z: value.z,
        }
    }
}
