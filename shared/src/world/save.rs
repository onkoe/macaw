//! # Save
//!
//! The different kinds of worlds in Macaw.

// there needs to be a way to make worlds without a generator
// for map/mod creators. also, custom generators are cool
// and i'll leave the option open for anyone who wants to give it
// a try!

use std::{
    collections::HashMap,
    fs::File,
    io::Write as _,
    path::{Path, PathBuf},
    sync::Arc,
};

use bevy::tasks::block_on;
use serde::{Deserialize, Serialize};

use super::{
    chunk::Chunk, coordinates::GlobalCoordinate, loader::WorldLoadingError,
    metadata::WorldMetadata, region::Region,
};

pub const GAME_DIRECTORY: &str = "macaw";
pub const SAVES_DIRECTORY: &str = "saves";

/// A representation of the world's actual save files.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldSave {
    /// The name of the world being saved. Used to find paths.
    metadata: Arc<WorldMetadata>,
    /// The location at which all this world's data is saved.
    save_path: PathBuf,
}

impl WorldSave {
    /// Loads an existing save, if it exists, or attempts to create a new save.
    pub async fn new(world_metadata: Arc<WorldMetadata>) -> Result<Self, WorldLoadingError> {
        let save_path = Self::get_path(world_metadata.clone()).await;

        // check if the save exists
        if let Some(save) = Self::try_load(world_metadata.clone(), save_path.clone()).await {
            Ok(save)
        } else {
            // check if we can create a new save
            Ok(Self::try_new(world_metadata, save_path).await?)
        }
    }

    /// Writes metadata to disk.
    pub async fn write_metadata(&self) -> Result<(), WorldLoadingError> {
        // serialize self to string
        let s = toml::to_string_pretty(&self).map_err(|e| {
            WorldLoadingError::MetadataWriteFailed(format!(
                "failed to serialize metadata to toml: {}",
                e
            ))
        })?;

        // create a file
        let metadata_path = self.save_path.join("save.toml");
        let mut file = File::create(metadata_path).map_err(|e| {
            WorldLoadingError::MetadataWriteFailed(format!("failed to create metadata file: {}", e))
        })?;

        // write the string'd self to that file
        file.write_all(s.as_bytes()).map_err(|e| {
            WorldLoadingError::MetadataWriteFailed(format!(
                "failed to write stringy metadata: {}",
                e
            ))
        })?;

        Ok(())
    }

    /// Writes chunks to disk.
    pub async fn write_chunks(&self, regions: &[Region]) -> Result<(), WorldLoadingError> {
        // write all to disk
        for r in regions {
            r.write()?;
        }

        Ok(())
    }

    /// The metadata of the world being saved.
    pub fn metadata(&self) -> Arc<WorldMetadata> {
        self.metadata.clone()
    }

    /// The name of the world being saved.
    pub async fn name(&self) -> Arc<String> {
        Arc::new(self.metadata.name().to_owned())
    }

    /// Attempts to check if this world save is on disk.
    async fn try_load(metadata: Arc<WorldMetadata>, save_path: Arc<PathBuf>) -> Option<Self> {
        let res = Path::new(save_path.as_ref()).try_exists();

        if let Ok(true) = res {
            // return a Self
            Some(Self {
                metadata,
                save_path: {
                    let mut p = PathBuf::new();
                    p.push(save_path.as_ref());
                    p
                },
            })
        } else {
            // return nothing lol
            None
        }
    }

    /// Attempts to create a new world save on disk.
    async fn try_new(
        world_metadata: Arc<WorldMetadata>,
        save_path: Arc<PathBuf>,
    ) -> Result<Self, WorldLoadingError> {
        // attempt to create world folder
        std::fs::create_dir_all(Path::new(save_path.as_ref())).map_err(|e| {
            WorldLoadingError::SavePathCreationFailure(format!(
                "{} at {}",
                e,
                save_path.to_string_lossy()
            ))
        })?;

        // if we're still here, it worked! let's try to create a world metadata file
        let s = Self {
            metadata: world_metadata.clone(),
            save_path: {
                let mut p = PathBuf::new();
                p.push(save_path.as_ref());
                p
            },
        };

        // write metadata to disk
        s.write_metadata().await?;

        // let's return the `WorldSave`
        Ok(s)
    }

    /// Gets the path of the save, given the name of the world.
    async fn get_path(world_metadata: Arc<WorldMetadata>) -> Arc<PathBuf> {
        world_metadata.save_path()
    }
}

/// Gets the path where all game saves are kept. This is currently
/// 'hard-coded', but should later take user configuration during launch.
/// ```
/// # use shared::world::save::get_saves_path;
///
/// let path = get_saves_path(SavePath::User);
/// assert!(path.contains("macaw/saves"));
/// ```
pub fn get_saves_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(
        directories::BaseDirs::new()
            .expect("OS should have a home directory..?")
            .config_dir(),
    );

    path.push(GAME_DIRECTORY);
    path.push(SAVES_DIRECTORY);
    path
}
