//! # Local
//!
//! Utilities for local block coordinates within a chunk.

use std::fmt::Display;

use bevy::math::Vec3;

use crate::{block::BlockSide, world::chunk::CHUNK_LENGTH};

/// A coordinate in a chunk. Chunks are 16x16x16, so all values must be in the
/// range [0, 15].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct ChunkBlockCoordinate {
    x: u8,
    y: u8,
    z: u8,
}

impl ChunkBlockCoordinate {
    /// The first block in a chunk, at (0, 0, 0).
    pub const ORIGIN: ChunkBlockCoordinate = ChunkBlockCoordinate { x: 0, y: 0, z: 0 };

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

    /// Checks to see if a potential 'next' block is a neighbor of this one.
    pub fn is_directional_neighbor(
        &self,
        next: &ChunkBlockCoordinate,
        direction: &BlockSide,
    ) -> bool {
        match direction {
            BlockSide::PositiveX => self.x + 1 == next.x,
            BlockSide::NegativeX => self.x - 1 == next.x,
            BlockSide::PositiveY => self.y + 1 == next.y,
            BlockSide::NegativeY => self.y - 1 == next.y,
            BlockSide::PositiveZ => self.z + 1 == next.z,
            BlockSide::NegativeZ => self.z - 1 == next.z,
        }
    }

    /// Given a direction to move in, this method returns the 'next' block in that direction.
    pub fn next(&self, direction: &BlockSide) -> Option<ChunkBlockCoordinate> {
        let mut x = self.x();
        let mut y = self.y();
        let mut z = self.z();

        match direction {
            BlockSide::PositiveX => x += 1,
            BlockSide::NegativeX => x -= 1,
            BlockSide::PositiveY => y += 1,
            BlockSide::NegativeY => y -= 1,
            BlockSide::PositiveZ => z += 1,
            BlockSide::NegativeZ => z -= 1,
        };

        let n = Self { x, y, z };

        if n.any(|v| v >= CHUNK_LENGTH) {
            None
        } else {
            Some(n)
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        }
    }

    /// Returns the largest coordinate value.
    ///
    /// ```
    /// # use macaw::world::coordinates::ChunkBlockCoordinate;
    /// #
    /// let coord = ChunkBlockCoordinate::new(1, 2, 3);
    /// assert_eq!(coord.largest(), 3);
    pub fn largest(&self) -> u8 {
        self.x.max(self.y).max(self.z)
    }

    /// Returns the smallest coordinate value.
    ///
    ///```
    /// # use macaw::world::coordinates::ChunkBlockCoordinate;
    /// #
    /// let coord = ChunkBlockCoordinate::new(3, 2, 1);
    /// assert_eq!(coord.smallest(), 1);
    pub fn smallest(&self) -> u8 {
        self.x.min(self.y).min(self.z)
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

impl Display for ChunkBlockCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "ChunkBlockCoordinate(x: {}, y: {}, z: {})",
            self.x, self.y, self.z
        ))
    }
}
impl super::Coordinate for ChunkBlockCoordinate {}
