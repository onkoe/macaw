//! # Region
//!
//! A collection of chunks in a 32x32x32 area. Inspired by [the wonderful `McRegion`
//! format](https://tinyurl.com/mu3bfpkk)!

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{chunk::Chunk, coordinates::GlobalCoordinate, metadata::WorldMetadata};

/// A 'region' of 32x32x32 surrounding a collection of chunks.
/// Used to save these chunks to disk.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Region {
    /// The world metadata.
    metadata: Arc<WorldMetadata>,
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
    pub fn new(coordinates: GlobalCoordinate, metadata: Arc<WorldMetadata>) -> Self {
        Self {
            metadata,
            coordinates,
            chunks: HashMap::new(),
            modification_date: chrono::Utc::now(),
        }
    }

    /// Gets a region from disk, if it exists. Otherwise, it'll create a new one,
    /// write it to disk, and return it.
    pub fn get(
        coordinates: GlobalCoordinate,
        metadata: Arc<WorldMetadata>,
    ) -> Result<Self, RegionError> {
        if let Ok(region) = Self::load(coordinates, metadata.clone()) {
            Ok(region)
        } else {
            // let's create one!
            let region = Self::new(coordinates, metadata);
            region.write()?;
            Ok(region)
        }
    }

    /// Loads a region from disk.
    ///
    /// This can fail if the region isn't found on disk or is corrupted.
    /// TODO: handle corruption!
    pub fn load(
        coordinates: GlobalCoordinate,
        world_metadata: Arc<WorldMetadata>,
    ) -> Result<Self, RegionError> {
        // attempt to find region file
        let path = Region::path_from_coordinates(&coordinates, world_metadata);

        if !path.exists() {
            return Err(RegionError::RegionReadFailed(
                path.to_string_lossy().to_string(),
            ));
        }

        // read region into a buffer
        let buf = std::fs::read(path.as_ref())
            .map_err(|e| RegionError::RegionReadFailed(e.to_string()))?;

        // deserialize that buffer into a region
        let s: Region = bincode::deserialize(&buf).map_err(|e| {
            RegionError::RegionWriteFailed(format!(
                "failed to deserialize region from bincode: `{}`",
                e
            ))
        })?;

        tracing::warn!("read region from disk: {:?}", &s.chunks.len());
        tracing::error!("chunk: {:?}", &s.chunks.get(&GlobalCoordinate::ORIGIN));

        Ok(s)
    }

    /// Writes this region to disk.
    pub fn write(&self) -> Result<(), RegionError> {
        tracing::debug!("Writing region at {:?}", self.coordinates());

        // serialize this region into bincode
        let s = bincode::serialize(&self)
            .map_err(|e| RegionError::ChunkSerializationFailed(e.to_string()))?;

        tracing::debug!("Serialized region to bincode: {:?}", self.coordinates());

        // write to disk
        let path = Region::path(self);
        tracing::debug!("Writing region to disk: {:?}", &path);

        std::fs::write(path.as_ref(), s).map_err(|e| {
            RegionError::RegionWriteFailed(format!(
                "failed to actually write region to disk at `{}`: {}",
                path.to_string_lossy(),
                e
            ))
        })?;

        Ok(())
    }

    /// This region's filename.
    pub fn filename(coordinates: &GlobalCoordinate) -> String {
        let (x, y, z) = coordinates.free();
        format!("{}_{}_{}.region", x, y, z)
    }

    /// Gets this region's path on disk, even if that path doesn't exist yet.
    pub fn path(&self) -> Arc<PathBuf> {
        Self::path_from_coordinates(&self.coordinates, self.metadata.clone())
    }

    /// Gets a region's path on disk from its coordinates.
    pub fn path_from_coordinates(
        coordinates: &GlobalCoordinate,
        world_metadata: Arc<WorldMetadata>,
    ) -> Arc<PathBuf> {
        let p = world_metadata.save_path().join(Self::filename(coordinates));
        Arc::new(p)
    }

    /// Returns a copy of this region's coordinates.
    pub fn coordinates(&self) -> GlobalCoordinate {
        self.coordinates
    }

    pub fn chunks(&self) -> &HashMap<GlobalCoordinate, Chunk> {
        &self.chunks
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
    pub fn add_chunk(
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
