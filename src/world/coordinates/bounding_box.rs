//! # Bounding Box
//!
//! Helps define an area in the world of Macaw.

use bevy::math::primitives::Cuboid;
use std::collections::HashSet;

use super::{ChunkBlockCoordinate, GlobalCoordinate};
use crate::{block::BlockSide, world::chunk::Chunk};

/// A bounding box around two `Coordinate`s.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct BoundingBox<T: super::Coordinate> {
    smaller: T,
    larger: T,
}

impl<T: super::Coordinate + Ord + Copy> BoundingBox<T> {
    /// Creates a new `BoundingBox` from the two given bounds.
    ///
    /// ```
    /// # use macaw::world::coordinates::{BoundingBox, ChunkBlockCoordinate};
    /// let (bound_a, bound_b) = (ChunkBlockCoordinate::new(2, 2, 2), ChunkBlockCoordinate::new(1, 1, 1));
    /// let bb = BoundingBox::new(bound_a, bound_b);
    ///
    /// assert_eq!(bb.bounds(), (bound_b, bound_a));
    /// ```
    pub fn new(bound_a: T, bound_b: T) -> Self {
        Self {
            smaller: bound_a.min(bound_b),
            larger: bound_a.max(bound_b),
        }
    }

    /// Given a point, creates a new `BoundingBox` with both bounds set at this point.
    pub fn new_point(bound: T) -> Self {
        Self {
            smaller: bound,
            larger: bound,
        }
    }

    /// Shows whether or not the BoundingBox represents a single point.
    ///
    /// ```
    /// # use macaw::world::coordinates::{BoundingBox, ChunkBlockCoordinate};
    /// #
    /// let bounds = BoundingBox::new_point(ChunkBlockCoordinate::ORIGIN);
    /// assert!(bounds.is_point());
    /// ```
    pub fn is_point(&self) -> bool {
        self.smaller == self.larger
    }

    /// Returns the smaller of the two bounds.
    pub fn smaller(&self) -> T {
        self.smaller
    }

    /// Returns the larger of the two bounds.
    pub fn larger(&self) -> T {
        self.larger
    }

    /// Returns a tuple (smaller, larger) of the two bounds.
    pub fn bounds(&self) -> (T, T) {
        (self.smaller, self.larger)
    }
}

impl BoundingBox<GlobalCoordinate> {
    /// Extend the bounding box to include a new bound.
    ///
    /// ```
    /// # use macaw::world::coordinates::{BoundingBox, GlobalCoordinate};
    /// #
    /// let (low_coord, high_coord) = (GlobalCoordinate::new(0, -20, 0), GlobalCoordinate::new(0, 4, 0));
    /// let mut bounds = BoundingBox::new_point(GlobalCoordinate::ORIGIN);
    ///
    /// bounds.extend(low_coord);
    /// bounds.extend(high_coord);
    /// assert_eq!(bounds.bounds(), (low_coord, high_coord));
    ///
    /// bounds.extend(GlobalCoordinate::ORIGIN);
    /// assert_eq!(bounds.bounds(), (low_coord, high_coord));
    /// ```
    pub fn extend(&mut self, bound: GlobalCoordinate) {
        self.smaller = GlobalCoordinate::new(
            self.smaller.x.min(bound.x),
            self.smaller.y.min(bound.y),
            self.smaller.z.min(bound.z),
        );
        self.larger = GlobalCoordinate::new(
            self.larger.x.max(bound.x),
            self.larger.y.max(bound.y),
            self.larger.z.max(bound.z),
        );
    }

    /// Creates a cuboid from self given the amount of blocks within.
    pub fn as_cuboid(&self) -> Cuboid {
        Cuboid::new(
            ((self.larger.x - self.smaller.x) + 1) as f32,
            ((self.larger.y - self.smaller.y) + 1) as f32,
            ((self.larger.z - self.smaller.z) + 1) as f32,
        )
    }
}

impl BoundingBox<ChunkBlockCoordinate> {
    /// Extend the bounding box to include a new bound.
    ///
    /// ```
    /// # use macaw::world::coordinates::{BoundingBox, ChunkBlockCoordinate};
    /// #
    /// let (low_coord, high_coord) = (, ChunkBlockCoordinate::new(0, 4, 0));
    /// let mut bounds = BoundingBox::new_point(ChunkBlockCoordinate::ORIGIN);
    ///
    /// bounds.extend(low_coord);
    /// bounds.extend(high_coord);
    /// assert_eq!(bounds.bounds(), (low_coord, high_coord));
    ///
    /// bounds.extend(ChunkBlockCoordinate::ORIGIN);
    /// assert_eq!(bounds.bounds(), (low_coord, high_coord));

    /// ```
    pub fn extend(&mut self, bound: ChunkBlockCoordinate) {
        self.smaller = ChunkBlockCoordinate::new(
            self.smaller.x().min(bound.x()),
            self.smaller.y().min(bound.y()),
            self.smaller.z().min(bound.z()),
        );
        self.larger = ChunkBlockCoordinate::new(
            self.larger.x().max(bound.x()),
            self.larger.y().max(bound.y()),
            self.larger.z().max(bound.z()),
        );
    }

    /// Returns the length of a given side. Ignores signedness of `direction`.
    pub fn length(&self, direction: &BlockSide) -> u8 {
        match direction {
            BlockSide::PositiveX | BlockSide::NegativeX => self.larger.x() - self.smaller.x(),
            BlockSide::PositiveY | BlockSide::NegativeY => self.larger.y() - self.smaller.y(),
            BlockSide::PositiveZ | BlockSide::NegativeZ => self.larger.z() - self.smaller.z(),
        }
    }

    /// Given a chunk, this method creates a bounding box with global coordinates
    /// instead of chunk-local ones.
    pub fn to_global(&self, chunk: &Chunk) -> BoundingBox<GlobalCoordinate> {
        let bound_1 = chunk.global_block_coord(self.smaller);
        let bound_2 = chunk.global_block_coord(self.larger);

        BoundingBox::new(bound_1, bound_2)
    }

    /// Creates a cuboid from self given the amount of blocks within.
    pub fn as_cuboid(&self) -> Cuboid {
        Cuboid::new(
            (self.larger.x() - self.smaller.x()) as f32,
            (self.larger.y() - self.smaller.y()) as f32,
            (self.larger.z() - self.smaller.z()) as f32,
        )
    }

    /// Returns a set of all coordinates within the box.
    pub fn all_coordinates(&self) -> HashSet<ChunkBlockCoordinate> {
        let mut set = HashSet::new();

        for x in self.smaller.x()..=self.larger.x() {
            for y in self.smaller.y()..=self.larger.y() {
                for z in self.smaller.z()..=self.larger.z() {
                    set.insert(ChunkBlockCoordinate::new(x, y, z));
                }
            }
        }

        set
    }
}
