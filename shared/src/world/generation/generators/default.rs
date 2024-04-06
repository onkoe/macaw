//! # Default Generator
//!
//! The default generator to create normal, mellow worlds.

use async_trait::async_trait;

use crate::{
    block::BlockSide,
    world::{
        chunk::Chunk,
        coordinates::GlobalCoordinate,
        generation::{biomes::DefaultBiomeGenerator, Generator},
        MacawWorld,
    },
};

/// An instance of the default world generator.
///
/// Create this before generating a world. Save it alongside the chunks.
#[derive(Clone, Debug, PartialEq)]
pub struct DefaultGenerator {
    seed: u64,
    biome_generator: DefaultBiomeGenerator,
    // TODO: put adjustable values here
}

impl DefaultGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            biome_generator: DefaultBiomeGenerator::new(seed),
        }
    }

    fn biome_map(&self) {
        let temperature_generator = libnoise::Worley::<2>::new(self.seed);
        // create temperature from noise
        // create biome layout from temperature + more noise :3

        // forest, plains, taiga (has trees), tundra (no trees), ocean, frozen ocean, lake, frozen lake, river, frozen river,
        // desert, bog (wetland), beach (pretty!!),

        //let x = noise::
    }
}

#[async_trait]
impl Generator for DefaultGenerator {
    fn name(&self) -> &'static str {
        "DefaultGenerator"
    }

    fn description(&self) -> &'static str {
        "Macaw's default world generator. Creates a normal, mellow world."
    }

    async fn pre_generate(&mut self, _seed: u64) -> MacawWorld {
        todo!()
    }

    async fn generate(
        &mut self,
        _world: &mut MacawWorld,
        _chunks: (
            crate::world::coordinates::ChunkBlockCoordinate,
            crate::world::coordinates::ChunkBlockCoordinate,
        ),
    ) {
        todo!()
    }
}

impl DefaultGenerator {
    /// Generates the first bits (48 chunks in all directions) of a world.
    fn generate(&mut self, seed: u64) -> MacawWorld {
        /*
        let mut world = MacawWorld::new(seed);

        for x in -24..=24 {
            // 16 * 16 = 256 blocks from bedrock
            for y in 0..=15 {
                for z in -24..=24 {
                    let coordinate = GlobalCoordinate::new(x, y, z);
                    let chunk = Chunk::new(coordinate);
                    world.chunks.insert(coordinate, chunk);
                }
            }
        }

        // let v = Vec::new();
        // for block in chunk.all_global_block_coordinates() {
        //      let noise = noise_generator.get(block.to_f32_array());
        //      v.push((block, block_noise));
        // }
        // let _noise = noise_generator.get(chunk_coords);

        // step 2: create a height map from the noise

        */
        todo!()
    }
}
