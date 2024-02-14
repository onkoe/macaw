use std::collections::HashMap;

use crate::{
    block::{Block, BlockType},
    world::{
        chunk::{Chunk, CHUNK_LENGTH},
        coordinates::ChunkBlockCoordinate,
        World,
    },
};
use noise::Perlin;
use rand::distributions::{Distribution, Uniform};

use super::GlobalCoordinate;

const _MAX_GEN_HEIGHT: i32 = 256;

pub struct Generate;

impl Generate {
    pub fn testing_world() -> World {
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

        World {
            chunks,
            _entities: (),
            ..Default::default()
        }
    }
}

/// Some kind of world generator.
pub trait Generator {
    fn generate(seed: u64) -> World;
}

pub struct DefaultGenerator;

impl DefaultGenerator {}

impl Generator for DefaultGenerator {
    /// The default world generator.
    fn generate(seed: u64) -> World {
        // let chunk_coords: [f64; 2] = [chunk.x as f64, chunk.z as f64];

        // step 1: create noise
        let _noise_generator = Perlin::new(seed as u32);

        // let v = Vec::new();
        // for block in chunk.all_global_block_coordinates() {
        //      let noise = noise_generator.get(block.to_f32_array());
        //      v.push((block, block_noise));
        // }
        // let _noise = noise_generator.get(chunk_coords);

        // step 2: create a height map from the noise

        todo!()
    }
}
