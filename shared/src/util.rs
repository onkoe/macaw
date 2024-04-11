//! # Util
//!
//! Some utilities that are (still) used throughout Macaw.
//!
//! This is like the other module at `/macaw/src/util.rs`, but it outputs
//! different results. That's mostly due to the `shared` crate's name.

/// Information about Cargo.toml.
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
