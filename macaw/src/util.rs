//! # util
//!
//! Some utilities that're used throughout Macaw.

/// Information about Cargo.toml.
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// Gets the package name, like "Macaw"
pub fn get_pkg_name() -> String {
    format!(
        "{}{}",
        (built_info::PKG_NAME[..1]).to_uppercase(),
        &built_info::PKG_NAME[1..]
    )
}

/// The full title of the game, like "Macaw Beta 1.7.3"
pub fn full_title() -> String {
    format!("{} Beta {}", get_pkg_name(), built_info::PKG_VERSION)
}

/// Gets the path of this crate's directory.
///
/// This may fail when used if distributed as a binary
/// (users won't have the path). Don't rely on this long-term.
pub const fn crate_dir() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

/// Gets a file path from the assets directory.
///
/// Again, this can fail. See `util::crate_dir()` for more info.
///
/// ```
/// let path = macaw::util::get_file("grass.png");
/// assert!(path.contains("macaw/assets/grass.png"));
/// ```
pub fn get_file(file: &str) -> String {
    format!("{}/assets/{file}", crate_dir())
}
