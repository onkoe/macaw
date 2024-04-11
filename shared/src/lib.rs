//! # Shared
//!
//! All the shared types between the Macaw client and server.
//!
//! For example, a `GlobalCoordinate` is useful when doing world generation and
//! rendering. You need the two sides to agree! 😄

pub mod block;
pub mod time;
pub mod util;
pub mod world;
