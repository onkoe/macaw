//! # Region
//!
//! A collection of chunks in a 32x32x32 area. Inspired by [the wonderful `McRegion`
//! format](https://tinyurl.com/mu3bfpkk)!

use std::collections::HashMap;

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{chunk::Chunk, coordinates::GlobalCoordinate};

/// A 'region' of 32x32x32 surrounding a collection of chunks.
/// Used to save these chunks to disk.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Region {
    /// The _region_ coordinate of this region. That's 32 times larger than
    /// chunks, and (16 * 32) times larger than blocks!
    ///
    /// Regions are named on disk according to their coordinates.
    coordinates: GlobalCoordinate,
    /// A list of chunks currently in this region. New chunks will be added.
    /// Empty chunks will be removed.
    ///
    /// Only chunks within the region's coordinates can be added.
    chunks: HashMap<GlobalCoordinate, Chunk>,
    /// The date/time when this region was last modified.
    /// Used to verify the world save.
    modification_date: DateTime<chrono::Utc>,
}

impl Region {
    pub const CHUNKS_PER_REGION: u8 = 32;

    /// Creates a new `Region`.
    pub fn new(coordinates: GlobalCoordinate) -> Self {
        Self {
            coordinates,
            chunks: HashMap::new(),
            modification_date: chrono::Utc::now(),
        }
    }

    pub fn write_to_disk(&self) -> Result<(), RegionError> {
        let s = bincode::serialize(&self.chunks)
            .map_err(|e| RegionError::ChunkSerializationFailed(e.to_string()))?;

        todo!()
    }

    /// Returns a copy of this region's coordinates.
    pub fn coordinates(&self) -> GlobalCoordinate {
        self.coordinates
    }

    /// Given a chunk's coordinates, finds the appropriate region coordinates.
    ///
    /// hint: This allows you to find regions on disk using chunks... :3
    pub fn find_region_coordinates(chunk_coordinates: GlobalCoordinate) -> GlobalCoordinate {
        chunk_coordinates * 16
    }

    /// Finds the minimum allowed chunk coordinate in a region, given its
    /// coordinates.
    pub fn minimum_chunk(rc: GlobalCoordinate) -> GlobalCoordinate {
        rc * 32
    }

    /// Finds the max. chunk coordinate in a region, given its coordinates.
    pub fn maximum_chunk(rc: GlobalCoordinate) -> GlobalCoordinate {
        // add minimum to be non-inclusive
        (rc * 32) + (Region::CHUNKS_PER_REGION - 1).into()
    }

    /// Tries to add a new chunk to the internal `chunks` list.
    ///
    /// This can fail if the chunk already exists or is out of bounds for this region.
    pub async fn add_chunk(
        &mut self,
        coordinates: GlobalCoordinate,
        chunk: Chunk,
    ) -> Result<(), RegionError> {
        // check if chunk is out-of-bounds
        if !self.can_contain_chunk(coordinates) {
            return Err(RegionError::WrongRegion {
                chunk: coordinates,
                region: self.coordinates(),
            });
        }

        // DANGEROUS: this will overwrite an existing chunk without checking
        // for the 'percentage' of modifications. It may be worth writing a
        // better public interface for this...

        self.chunks.insert(coordinates, chunk);
        self.modify();
        Ok(())
    }

    /// Checks to see if this `Region` should hold the given chunk.
    pub fn can_contain_chunk(&self, cc: GlobalCoordinate) -> bool {
        let min = self.coordinates() * 32;
        let max = min + 31;

        cc.x >= min.x
            && cc.x <= max.x
            && cc.y >= min.y
            && cc.y <= max.y
            && cc.z >= min.z
            && cc.z <= max.z
    }

    /// Changes the modification date of this region to the current time.
    ///
    /// Use this for *all* region modifications!
    fn modify(&mut self) {
        self.modification_date = chrono::Utc::now();
    }
}

#[derive(Clone, Debug, Error, PartialEq, PartialOrd, Hash)]
pub enum RegionError {
    #[error("Chunk at `{chunk}` is out of bounds for region at {region}")]
    WrongRegion {
        chunk: GlobalCoordinate,
        region: GlobalCoordinate,
    },
    #[error("Failed to serialize chunks to `bincode`: `{0}`")]
    ChunkSerializationFailed(String),
}
