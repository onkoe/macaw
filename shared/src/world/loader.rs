//! # Loader
//!
//! A module that saves/loads the world on disk.

use super::{chunk::Chunk, coordinates::GlobalCoordinate, region::RegionError, save::WorldSave};
use crate::world::metadata::WorldMetadata;
use bevy::tasks::futures_lite::future::zip;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;

/// Manages the world's operations to/from disk.
#[derive(Debug)]
pub struct WorldLoader {
    /// The name of the loaded world.
    metadata: Arc<WorldMetadata>,
    /// All currently-loaded chunks in the world.
    loaded: HashMap<GlobalCoordinate, Chunk>,
    /// The file the world is being saved into.
    save: WorldSave,
}

impl WorldLoader {
    pub async fn new(world_metadata: Arc<WorldMetadata>) -> Result<Self, WorldLoadingError> {
        // attempt to create a new save

        Self {
            metadata: world_metadata.clone(),
            loaded: HashMap::new(),
            save: WorldSave::new(world_metadata).await?,
        };

        todo!()
    }

    /// Creates a temporary world that isn't saved.
    pub async fn temp(chunks: HashMap<GlobalCoordinate, Chunk>) -> Self {
        let save = WorldSave::temp().await;

        Self {
            metadata: save.metadata().await,
            loaded: chunks,
            save,
        }
    }

    /// Saves the world, like I did when I was born.
    pub async fn push_to_disk(&self) -> Result<(), WorldLoadingError> {
        let (metadata, chunks) = zip(
            self.save.write_metadata(),
            self.save.write_chunks(&self.loaded),
        )
        .await;

        metadata?;
        chunks?;
        // TODO: write mobs/other world factors..?

        Ok(())
    }

    /// The currently-loaded chunks, as of calling. Be careful!
    ///
    /// TODO: having this not be borrowed may cause bugs. Either document
    /// well or change something!
    pub(crate) fn chunks(&self) -> HashMap<GlobalCoordinate, Chunk> {
        self.loaded.clone()
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
    #[error("This world name is already taken. Please choose another name.")]
    WorldNameTaken,
    #[error("Failed to find `saves/` directory.")]
    NoSaveDirectory,
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
