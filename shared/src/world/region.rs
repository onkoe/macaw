//! # Region
//!
//! A collection of chunks in a 32x32x32 area. Inspired by [the wonderful `McRegion`
//! format](https://tinyurl.com/mu3bfpkk)!

use std::{collections::HashMap, path::PathBuf};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::world::save::get_saves_path;

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

    /// Loads a region from disk.
    pub fn load(coordinates: GlobalCoordinate) -> Result<Self, RegionError> {
        // attempt to find region file
        let path = Region::path(&coordinates);

        if !path.exists() {
            return Err(RegionError::RegionReadFailed(
                path.to_string_lossy().to_string(),
            ));
        }

        // read region into a buffer
        let buf = std::fs::read(path).map_err(|e| RegionError::RegionReadFailed(e.to_string()))?;

        // deserialize that buffer into a region
        let s = bincode::deserialize(&buf)
            .map_err(|e| RegionError::RegionWriteFailed(e.to_string()))?;

        Ok(s)
    }

    /// Writes this region to disk.
    pub fn write(&self) -> Result<(), RegionError> {
        // serialize this region into bincode
        let s = bincode::serialize(&self)
            .map_err(|e| RegionError::ChunkSerializationFailed(e.to_string()))?;

        // write to disk
        let path = Region::path(&self.coordinates);
        std::fs::write(path, s).map_err(|e| RegionError::RegionWriteFailed(e.to_string()))?;

        Ok(())
    }

    /// This region's filename.
    pub fn filename(coordinates: &GlobalCoordinate) -> String {
        let (x, y, z) = coordinates.free();
        format!("{}_{}_{}.region", x, y, z)
    }

    /// Gets this region's path on disk. Yes, even if it doesn't exist yet.
    pub fn path(coordinates: &GlobalCoordinate) -> PathBuf {
        let mut p = PathBuf::from(get_saves_path());
        p.push(Self::filename(coordinates));
        p
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
    #[error("Failed to write region to disk: `{0}`")]
    RegionWriteFailed(String),
    #[error("Failed to read region from disk: `{0}`")]
    RegionReadFailed(String),
}
