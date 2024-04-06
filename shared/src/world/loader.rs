//! # Loader
//!
//! A module that saves/loads the world on disk.

use std::{
    borrow::Cow,
    collections::HashMap,
    env::temp_dir,
    fs::File,
    path::{Path, PathBuf},
    sync::Arc,
};

use bevy::utils::Uuid;
use thiserror::Error;

use crate::world::save;

use super::{chunk::Chunk, coordinates::GlobalCoordinate};

/// Manages the world's operations to/from disk.
#[derive(Debug)]
pub struct WorldLoader {
    /// The name of the loaded world.
    name: Arc<String>,
    /// All currently-loaded chunks in the world.
    loaded: HashMap<GlobalCoordinate, Chunk>,
    /// The file the world is being saved into.
    save: WorldSave,
}

impl WorldLoader {
    pub async fn new(world_name: String) -> Result<Self, WorldLoadingError> {
        // attempt to create a new save

        Self {
            name: Arc::new(world_name.clone()),
            loaded: HashMap::new(),
            save: WorldSave::new(Arc::new(world_name)).await?,
        };

        todo!()
    }

    /// Creates a temporary world that isn't saved.
    pub async fn temp(chunks: HashMap<GlobalCoordinate, Chunk>) -> Self {
        let save = WorldSave::temp().await;

        Self {
            name: save.name().await,
            loaded: chunks,
            save,
        }
    }

    pub async fn push_to_disk(&self) -> Result<(), ()> {
        todo!()
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

/// A representation of the world's actual save files.
#[derive(Debug)]
pub struct WorldSave {
    /// The name of the world being saved. Used to find paths.
    name: Arc<String>,
    /// The location at which all this world's data is saved.
    save_path: PathBuf,
}

impl WorldSave {
    /// Loads an existing save, if it exists, or attempts to create a new save.
    pub async fn new(world_name: Arc<String>) -> Result<Self, WorldLoadingError> {
        let save_path = Self::get_path(world_name.clone()).await?;

        // check if the save exists
        if let Some(save) = Self::try_load(world_name, save_path).await {
            return Ok(save);
        } else {
            // check if we can create a new save
        }

        todo!()
    }

    async fn try_load(world_name: Arc<String>, save_path: Arc<String>) -> Option<Self> {
        let res = Path::new(save_path.as_str()).try_exists();

        if let Ok(true) = res {
            // return a Self
            Some(Self {
                name: world_name,
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

    async fn attempt_new(
        world_name: Arc<String>,
        save_path: Arc<String>,
    ) -> Result<Self, WorldLoadingError> {
        // attempt to create world folder
        std::fs::create_dir(Path::new(save_path.as_str()))
            .map_err(|_| WorldLoadingError::WorldNameTaken)?;

        // if we're still here, it worked! let's try to create a world metadata file
        todo!();

        // let's create the `WorldSave`
        Ok(Self {
            name: world_name,
            save_path: {
                let mut p = PathBuf::new();
                p.push(save_path.as_str());
                p
            },
        })
    }

    /// Gets the path of the save, given the name of the world.
    async fn get_path(world_name: Arc<String>) -> Result<Arc<String>, WorldLoadingError> {
        let saves_folder =
            save::get_saves_path().map_err(|_| WorldLoadingError::NoSaveDirectory)?;

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

    /// Creates a temporary world save that won't stick around.
    pub async fn temp() -> Self {
        Self {
            name: Arc::new(Uuid::new_v4().to_string()),
            save_path: temp_dir(),
        }
    }

    pub async fn name(&self) -> Arc<String> {
        self.name.to_owned()
    }
}

/// A world-loading error.
#[derive(Clone, Copy, Debug, Error, PartialEq, PartialOrd, Hash)]
pub enum WorldLoadingError {
    #[error("This world name is already taken. Please choose another name.")]
    WorldNameTaken,
    #[error("Failed to find `saves/` directory.")]
    NoSaveDirectory,
    #[error("Given world name isn't valid UTF-8.")]
    WorldNameWackFormatting,
    #[error("Requested world doesn't exist.")]
    WorldDoesntExist,
}
