//! # Save
//!
//! The different kinds of worlds in Macaw.

// there needs to be a way to make worlds without a generator
// for map/mod creators. also, custom generators are cool
// and i'll leave the option open for anyone who wants to give it
// a try!

use std::{
    collections::HashMap,
    env::temp_dir,
    fs::File,
    io::Write as _,
    path::{Path, PathBuf},
    sync::Arc,
};

use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

use super::{
    chunk::Chunk,
    coordinates::GlobalCoordinate,
    generation::{generators::blank::BlankGenerator, Generator as _},
    loader::WorldLoadingError,
    metadata::WorldMetadata,
};

pub const GAME_DIRECTORY: &str = "macaw";
pub const SAVES_DIRECTORY: &str = "saves";

/// A representation of the world's actual save files.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldSave {
    /// The name of the world being saved. Used to find paths.
    metadata: Arc<WorldMetadata>,
    /// The location at which all this world's data is saved.
    save_path: PathBuf,
}

impl WorldSave {
    /// Loads an existing save, if it exists, or attempts to create a new save.
    pub async fn new(world_metadata: Arc<WorldMetadata>) -> Result<Self, WorldLoadingError> {
        let save_path = Self::get_path(Arc::new(world_metadata.name().to_owned())).await?;

        // check if the save exists
        if let Some(save) = Self::try_load(world_metadata.clone(), save_path.clone()).await {
            Ok(save)
        } else {
            // check if we can create a new save
            Ok(Self::try_new(world_metadata, save_path).await?)
        }
    }

    /// Creates a temporary world save that won't stick around.
    pub async fn temp() -> Self {
        Self {
            metadata: Arc::new(WorldMetadata::new_now(
                Uuid::new_v4().to_string(),
                0,
                BlankGenerator.id(),
            )),
            save_path: temp_dir(),
        }
    }

    /// Writes metadata to disk.
    pub async fn write_metadata(&self) -> Result<(), WorldLoadingError> {
        // serialize self to string
        let s = toml::to_string_pretty(&self)
            .map_err(|e| WorldLoadingError::MetadataWriteFailed(e.to_string()))?;

        // create a file
        let mut file = File::create(self.save_path.clone())
            .map_err(|e| WorldLoadingError::MetadataWriteFailed(e.to_string()))?;

        // write the string'd self to that file
        file.write_all(s.as_bytes())
            .map_err(|e| WorldLoadingError::MetadataWriteFailed(e.to_string()))?;

        Ok(())
    }

    /// Writes chunks to disk.
    pub async fn write_chunks(
        &self,
        chunks: &HashMap<GlobalCoordinate, Chunk>,
    ) -> Result<(), WorldLoadingError> {
        todo!("loader.rs: write_chunks()... we should write to a region first. which means we need regions!!")
    }

    /// The metadata of the world being saved.
    pub async fn metadata(&self) -> Arc<WorldMetadata> {
        self.metadata.clone()
    }

    /// The name of the world being saved.
    pub async fn name(&self) -> Arc<String> {
        Arc::new(self.metadata.name().to_owned())
    }

    /// Attempts to check if this world save is on disk.
    async fn try_load(metadata: Arc<WorldMetadata>, save_path: Arc<String>) -> Option<Self> {
        let res = Path::new(save_path.as_str()).try_exists();

        if let Ok(true) = res {
            // return a Self
            Some(Self {
                metadata,
                save_path: {
                    let mut p = PathBuf::new();
                    p.push(save_path.as_str());
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
        save_path: Arc<String>,
    ) -> Result<Self, WorldLoadingError> {
        // attempt to create world folder
        std::fs::create_dir(Path::new(save_path.as_str()))
            .map_err(|_| WorldLoadingError::WorldNameTaken)?;

        // if we're still here, it worked! let's try to create a world metadata file
        let s = Self {
            metadata: world_metadata.clone(),
            save_path: {
                let mut p = PathBuf::new();
                p.push(save_path.as_str());
                p
            },
        };

        // write metadata to disk
        s.write_metadata().await?;

        // let's return the `WorldSave`
        Ok(s)
    }

    /// Gets the path of the save, given the name of the world.
    async fn get_path(world_name: Arc<String>) -> Result<Arc<String>, WorldLoadingError> {
        let saves_folder = get_saves_path();

        let save = format!(
            "{}/{}",
            saves_folder
                .to_str()
                .ok_or(WorldLoadingError::WorldNameWackFormatting)?,
            world_name
        );

        // urlencoded to discourage nonsense :3
        let save_folder_path = urlencoding::encode(&save).to_string();

        Ok(Arc::new(save_folder_path))
    }
}

/// Gets the path where all game saves are kept. This is currently
/// 'hard-coded', but should later take user configuration during launch.
pub fn get_saves_path() -> PathBuf {
    let all_dirs = directories::ProjectDirs::from("", "", GAME_DIRECTORY)
        .expect("Failed to get `saves/` directory.");
    let dir = all_dirs.config_dir();

    let mut path = PathBuf::new();
    path.push(dir);
    path.push(SAVES_DIRECTORY);
    path
}
