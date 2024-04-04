use super::Chunk;
use crate::world::coordinates::ChunkBlockCoordinate;

pub trait Meshing {
    /// Given a location, returns whether or not the thing at that location
    /// should be visible for the mesh.
    ///
    /// In other words, this detects if it's on a chunk border or has air
    /// around it.
    fn is_visible(&self, block: &ChunkBlockCoordinate) -> bool;
}

impl Meshing for Chunk {
    fn is_visible(&self, block: &ChunkBlockCoordinate) -> bool {
        // show blocks on chunk borders
        // FIXME: this marks blocks that are on chunk borders, but can't be seen!
        if block.any(|v| v == 15) || block.any(|v| v == 0) {
            return true;
        }

        // show blocks with air around them
        for adj in self.adjacent_blocks(block) {
            if adj.is_transparent() {
                return true;
            }
        }

        false
    }
}
