use thiserror::Error;

use super::coordinates::GlobalCoordinate;

#[derive(Clone, Copy, Debug, Error)]
pub enum WorldError {
    #[error("A chunk already exists at location: `{0}`")]
    ChunkAlreadyExists(GlobalCoordinate),
}
