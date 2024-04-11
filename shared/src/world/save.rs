//! # Save
//!
//! The different kinds of worlds in Macaw.

// there needs to be a way to make worlds without a generator
// for map/mod creators. also, custom generators are cool
// and i'll leave the option open for anyone who wants to give it
// a try!

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use super::{
    coordinates::GlobalCoordinate,
    loader::WorldLoadingError,
    metadata::WorldMetadata,
    region::{Region, RegionError},
};

pub const GAME_DIRECTORY: &str = "macaw";
pub const SAVES_DIRECTORY: &str = "saves";

/// A representation of the world's actual save files.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldSave {
    /// The name of the world being saved. Used to find paths.
    metadata: Arc<WorldMetadata>,
}

impl WorldSave {
    /// Loads an existing save, if it exists, or attempts to create a new save.
    pub fn new(world_metadata: Arc<WorldMetadata>) -> Result<Self, WorldLoadingError> {
        let save_path = world_metadata.save_path();

        // check if the save exists already
        if let Some(save) = Self::try_load(world_metadata.clone()) {
            return Ok(save);
        } else {
            // no save yet. let's try to create one!

            // make the folder
            tracing::debug!("Attempting to create new save folder at {:?}", save_path);
            std::fs::create_dir_all(save_path.as_ref()).map_err(|e| {
                WorldLoadingError::SavePathCreationFailure(format!(
                    "error: `{}` at `{}`",
                    e,
                    save_path.to_string_lossy()
                ))
            })?;

            // try to write metadata
            world_metadata.write_to_disk()?;
        }

        // all good! here's the save
        Ok(Self {
            metadata: world_metadata,
        })
    }

    /// The metadata of the world being saved.
    pub fn metadata(&self) -> Arc<WorldMetadata> {
        self.metadata.clone()
    }

    /// The name of the world being saved.
    pub fn name(&self) -> Arc<String> {
        Arc::new(self.metadata.name().to_owned())
    }

    /// Attempts to load a world save from disk given its metadata.
    fn try_load(metadata: Arc<WorldMetadata>) -> Option<Self> {
        let res = Path::new(metadata.save_path().as_ref()).try_exists();

        // if we found the save, return it
        if let Ok(true) = res {
            Some(Self { metadata })
        } else {
            None
        }
    }

    pub fn load_region(
        &self,
        region_coordinate: GlobalCoordinate,
    ) -> Result<Region, WorldLoadingError> {
        let region_path = Region::path_from_coordinates(&region_coordinate, self.metadata.clone());

        if !region_path.exists() {
            return Err(WorldLoadingError::RegionReadFailed(
                region_path.to_string_lossy().to_string(),
            ));
        }

        // read region into a buffer
        let buf = std::fs::read(region_path.as_ref())
            .map_err(|e| WorldLoadingError::RegionReadFailed(e.to_string()))?;

        // deserialize that buffer into a region
        let s: Region = bincode::deserialize(&buf).map_err(|e| {
            RegionError::RegionReadFailed(format!(
                "failed to deserialize region from bincode: `{}`",
                e
            ))
        })?;

        Ok(s)
    }
}

/// Gets the path where all game saves are kept. This is currently
/// 'hard-coded', but should later take user configuration during launch.
/// ```
/// # use shared::world::save::get_saves_path;
///
/// let path = get_saves_path();
/// assert!(path.to_string_lossy().contains("macaw/saves"));
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
