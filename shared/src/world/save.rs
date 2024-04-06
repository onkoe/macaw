//! # Save
//!
//! The different kinds of worlds in Macaw.

// there needs to be a way to make worlds without a generator
// for map/mod creators. also, custom generators are cool
// and i'll leave the option open for anyone who wants to give it
// a try!

use std::path::PathBuf;

pub const GAME_DIRECTORY: &str = "macaw";
pub const SAVES_DIRECTORY: &str = "saves";

/// Gets the path where all game saves are kept. This is currently
/// 'hard-coded', but should later take user configuration during launch.
pub fn get_saves_path() -> Result<PathBuf, ()> {
    let all_dirs = directories::ProjectDirs::from("", "", GAME_DIRECTORY).ok_or(())?;
    let dir = all_dirs.config_dir();

    let mut path = PathBuf::new();
    path.push(dir);
    path.push(SAVES_DIRECTORY);
    Ok(path)
}
