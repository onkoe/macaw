//! # Fixed Generator
//!
//! An included generator to create static worlds.

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use rand::distributions::{Distribution as _, Uniform};

use crate::{
    block::{Block, BlockType},
    world::{
        chunk::{Chunk, CHUNK_LENGTH},
        coordinates::{ChunkBlockCoordinate, GlobalCoordinate},
        generation::{Generator as _, GeneratorWrapper},
        loader::WorldLoader,
        metadata::{self, WorldMetadata},
        MacawWorld,
    },
};

use super::blank::BlankGenerator;

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

        let metadata = Arc::new(metadata::WorldMetadata::new_now(
            "Test Chunk".into(),
            0,
            BlankGenerator.id(),
        ));

        let mut loader = WorldLoader::new(metadata).expect("failed to load testing world");

        loader.chunks_mut().insert(tbc_coord, two_block_chunk);

        MacawWorld {
            loader,
            ..MacawWorld::default()
        }
    }

    pub fn one_test_block() -> MacawWorld {
        let chunk_coordinate = GlobalCoordinate::new(0, 0, 0);
        let mut chunk = Chunk::new(chunk_coordinate);
        chunk.set_block(
            Block::new(BlockType::Grass, 0),
            ChunkBlockCoordinate::new(0, 0, 0),
        );

        let metadata = Arc::new(WorldMetadata::new_now(
            "One Block".into(),
            0,
            BlankGenerator.id(),
        ));

        let mut loader = WorldLoader::new(metadata.clone()).unwrap();
        loader.chunks_mut().insert(chunk_coordinate, chunk);

        MacawWorld {
            metadata,
            loader,
            entities: HashSet::new(),
            spawn_location: GlobalCoordinate::ORIGIN,
        }
    }

    pub async fn generate_test_chunk() -> MacawWorld {
        let mut chunk =
            Chunk::new_filled(Block::new(BlockType::Stone, 0), GlobalCoordinate::ORIGIN);

        // set first layer as grass
        for x in 0..16 {
            for z in 0..16 {
                chunk.set_block(
                    Block::new(BlockType::Grass, 0),
                    ChunkBlockCoordinate::new(x, 15, z),
                )
            }
        }

        // set 2nd-4th layers as dirt
        for x in 0..16 {
            for y in 12..=14 {
                for z in 0..16 {
                    chunk.set_block(
                        Block::new(BlockType::Dirt, 0),
                        ChunkBlockCoordinate::new(x, y, z),
                    )
                }
            }
        }

        let metadata = Arc::new(WorldMetadata::new_now(
            "Test Chunk".into(),
            0,
            BlankGenerator.id(),
        ));

        let mut loader = WorldLoader::new(metadata.clone()).expect("failed to load testing world");

        loader.chunks_mut().insert(GlobalCoordinate::ORIGIN, chunk);

        MacawWorld {
            metadata,
            loader,
            entities: HashSet::new(),
            spawn_location: GlobalCoordinate::new(0, 18, 0),
        }
    }
}
