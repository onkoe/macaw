//! # Bounding Box
//!
//! Helps define an area in the world of Macaw.

use super::{ChunkBlockCoordinate, GlobalCoordinate};

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
    ///
    ///
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
        if bound.smallest() < self.smaller.smallest() {
            // combine the lowest of both coords
            self.smaller = GlobalCoordinate::new(
                bound.x.min(self.smaller.x),
                bound.y.min(self.smaller.y),
                bound.z.min(self.smaller.z),
            );
        } else if bound.largest() > self.larger.largest() {
            // combine the highest of both coords
            self.larger = GlobalCoordinate::new(
                bound.x.max(self.larger.x),
                bound.y.max(self.larger.y),
                bound.z.max(self.larger.z),
            );
        }
    }
}

impl BoundingBox<ChunkBlockCoordinate> {
    /// Extend the bounding box to include a new bound.
    ///
    /// ```
    /// # use macaw::world::coordinates::{BoundingBox, LocalChunkCoordinate};
    /// #
    /// let (low_coord, high_coord) = (LocalChunkCoordinate::new(0, -20, 0), LocalChunkCoordinate::new(0, 4, 0));
    /// let mut bounds = BoundingBox::new_point(LocalChunkCoordinate::ORIGIN);
    ///
    /// bounds.extend(low_coord);
    /// bounds.extend(high_coord);
    /// assert_eq!(bounds.bounds(), (low_coord, high_coord));
    ///
    /// bounds.extend(LocalChunkCoordinate::ORIGIN);
    /// assert_eq!(bounds.bounds(), (low_coord, high_coord));

    /// ```
    pub fn extend(&mut self, bound: ChunkBlockCoordinate) {
        if bound.smallest() < self.smaller.smallest() {
            // combine the lowest of both coords
            self.smaller = ChunkBlockCoordinate::new(
                bound.x().min(self.smaller.x()),
                bound.y().min(self.smaller.y()),
                bound.z().min(self.smaller.z()),
            );
        } else if bound.largest() > self.larger.largest() {
            // combine the highest of both coords
            self.larger = ChunkBlockCoordinate::new(
                bound.x().max(self.larger.x()),
                bound.y().max(self.larger.y()),
                bound.z().max(self.larger.z()),
            );
        }
    }
}
