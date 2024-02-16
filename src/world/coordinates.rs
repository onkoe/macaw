mod bounding_box;
mod global;
mod local;

pub use global::GlobalCoordinate;
pub use local::ChunkBlockCoordinate;

/// A marker trait that indicates a type of coordinate.
pub trait Coordinate {}
