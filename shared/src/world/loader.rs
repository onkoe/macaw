//! # Loader
//!
//! A module that saves/loads the world on disk.

use super::{
    chunk::Chunk,
    coordinates::GlobalCoordinate,
    region::{Region, RegionError},
    save::WorldSave,
};

use crate::world::metadata::WorldMetadata;
use bevy::tasks::block_on;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;

/// Manages the world's operations to/from disk.
#[derive(Debug)]
pub struct WorldLoader {
    /// All currently-loaded chunks in the world.
    loaded: HashMap<GlobalCoordinate, Chunk>,
    /// The file the world is being saved into.
    save: WorldSave,
}

impl WorldLoader {
    /// Creates a new `WorldLoader` given a `WorldMetadata`.
    pub fn new(world_metadata: Arc<WorldMetadata>) -> Result<Self, WorldLoadingError> {
        Ok(Self {
            loaded: HashMap::new(),
            save: block_on(WorldSave::new(world_metadata))?,
        })
    }

    /// Creates a new `WorldLoader` given a `WorldSave` and `WorldMetadata`.
    pub fn new_with_save(save: WorldSave) -> Self {
        let _ = Self {
            save,
            loaded: HashMap::new(),
        };

        // load from disk
        todo!()
    }

    /// Loads the world from disk.
    pub fn load_from_disk(&mut self) -> Result<(), WorldLoadingError> {
        todo!()
    }

    /// Saves the world, like I did when I was born.
    pub async fn push_to_disk(&self) -> Result<(), WorldLoadingError> {
        // get all regions
        let regions = self.regions();

        // write all to disk
        self.save.write_chunks(&regions).await?;
        self.save.write_metadata().await?;
        // TODO: write mobs/other world factors..?

        Ok(())
    }

    /// Gets all regions for these loaded chunks.
    pub fn regions(&self) -> Vec<Region> {
        let mut v = HashMap::new();

        for (chunk_coordinates, chunk) in self.loaded.iter() {
            // add region to hashmap with new chunk if it's not there.
            // otherwise, just add the chunk to the existing region.
            let region_coordinates = chunk.region(chunk.coords());

            v.entry(region_coordinates)
                .or_insert_with(|| {
                    let mut region = Region::new(region_coordinates, self.save.metadata());
                    region
                        .add_chunk(*chunk_coordinates, chunk.clone())
                        .expect("region contains chunk");
                    region
                })
                .add_chunk(*chunk_coordinates, chunk.clone())
                .expect("region contains chunk");
        }

        v.into_values().collect()
    }

    pub fn get_save(&self) -> Result<WorldSave, WorldLoadingError> {
        Ok(self.save.clone())
    }

    /// The currently-loaded chunks in a mutable form.
    pub(crate) fn chunks_mut(&mut self) -> &mut HashMap<GlobalCoordinate, Chunk> {
        &mut self.loaded
    }

    /// The currently-loaded chunks in a referenced form.
    pub(crate) fn chunks_ref(&self) -> &HashMap<GlobalCoordinate, Chunk> {
        &self.loaded
    }
}

/// A world-loading error.
#[derive(Clone, Debug, Error, PartialEq, PartialOrd, Hash)]
pub enum WorldLoadingError {
    #[error("Failed to create a new save path: `{0}`")]
    SavePathCreationFailure(String),
    #[error("Given world name isn't valid UTF-8.")]
    WorldNameWackFormatting,
    #[error("Requested world doesn't exist.")]
    WorldDoesntExist,
    #[error("Error while writing metadata: `{0}`.")]
    MetadataWriteFailed(String),
    #[error("Couldn't write chunks to `bincode`: `{0}`")]
    ChunkSerializationFailed(String),
    #[error("Failed to write chunks to region: `{0}`.")]
    RegionWriteFailed(#[from] RegionError),
    #[error("World write failed: `{0}`.")]
    WorldWriteFailed(String),
}
