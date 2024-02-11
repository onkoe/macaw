use std::collections::HashMap;

use self::coordinates::GlobalCoordinate;

use super::block::Block;
use crate::{block::BlockType, world::chunk::Chunk};

pub mod chunk;
pub mod coordinates;
pub mod generation;
pub mod meshing;

pub struct World {
    chunks: HashMap<GlobalCoordinate, Chunk>,
}

impl World {
    pub fn get_chunk(&self, coords: GlobalCoordinate) -> Option<Chunk> {
        self.chunks.get(&coords).cloned()
    }

    pub fn chunks(&self) -> &HashMap<GlobalCoordinate, Chunk> {
        &self.chunks
    }
}

/// Creates the damn world. I'm like god up in here
pub fn generate() -> World {
    // create 8 chunks at y = 0 and fill them with cobblestone

    let mut chunks = HashMap::new();

    for x in 0..3 {
        for z in 0..3 {
            let coords = GlobalCoordinate::new(x, 0, z);
            tracing::debug!("generating chunk at {:?}", coords);

            chunks.insert(
                coords,
                Chunk::new_filled(Block::new(BlockType::Stone, 0), coords),
            );
        }
    }

    World { chunks }
}
