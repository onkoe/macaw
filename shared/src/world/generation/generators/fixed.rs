//! # Fixed Generator
//!
//! An included generator to create static worlds.

use std::collections::HashMap;

use rand::distributions::{Distribution as _, Uniform};

use crate::{
    block::{Block, BlockType},
    world::{
        chunk::{Chunk, CHUNK_LENGTH},
        coordinates::{ChunkBlockCoordinate, GlobalCoordinate},
        loader::WorldLoader,
        MacawWorld,
    },
};

pub struct Generate;

impl Generate {
    pub async fn testing_world() -> MacawWorld {
        let mut chunks = HashMap::new();

        // -----------------------
        // create the random chunk
        // -----------------------
        let mut rng = rand::thread_rng();
        let rand_chunk_coord = GlobalCoordinate::new(4, 4, 4);
        let mut random_chunk = Chunk::new(rand_chunk_coord);

        for x in 0..CHUNK_LENGTH {
            for y in 0..CHUNK_LENGTH {
                for z in 0..CHUNK_LENGTH {
                    let coord = ChunkBlockCoordinate::new(x, y, z);

                    let block_type = match Uniform::from(0..=2).sample(&mut rng) {
                        0 => BlockType::Stone,
                        1 => BlockType::Air,
                        2 => BlockType::Grass,
                        _ => {
                            unreachable!("rng only gens from 0 to 2")
                        }
                    };

                    random_chunk.set_block(Block::new(block_type, 0), coord);
                }
            }
        }

        chunks.insert(rand_chunk_coord, random_chunk);

        // ----------------------------
        // create two block testing area
        // -----------------------------
        let tbc_coord = GlobalCoordinate::new(0, 0, 0);
        let mut two_block_chunk = Chunk::new(tbc_coord);
        two_block_chunk.set_block(
            Block::new(BlockType::Stone, 0),
            ChunkBlockCoordinate::new(0, 0, 0),
        );
        two_block_chunk.set_block(
            Block::new(BlockType::Stone, 0),
            ChunkBlockCoordinate::new(1, 0, 0),
        );

        chunks.insert(tbc_coord, two_block_chunk);

        MacawWorld {
            loader: WorldLoader::temp(chunks).await,
            ..MacawWorld::default().await
        }
    }
}
