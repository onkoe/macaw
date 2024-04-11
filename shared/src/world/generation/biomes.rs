//! # Biomes
//!
//! The default implementation of the biome generator.

use libnoise::{Simplex, Worley};

use crate::{
    block::BlockType,
    world::{coordinates::GlobalCoordinate, MacawWorld},
};

#[derive(Clone, Debug)]
pub struct DefaultBiomeGenerator {
    /// A source of noise to map biomes onto.
    biomes_source: Simplex<2>,
    /// Noise that defines the temperature in a given area.
    temperature_source: Worley<2>,
    /// The seed that defines the noise of these sources.
    seed: u64,
}

impl DefaultBiomeGenerator {
    /// Creates a new DefaultBiomeGenerator from a given seed (u64).
    pub fn new(seed: u64) -> Self {
        Self {
            biomes_source: Simplex::new(seed),
            temperature_source: Worley::new(seed),
            seed,
        }
    }

    /// Given the coordinate of a block, gets the biome name at that point.
    pub fn get_biome_name(&self, coord: &GlobalCoordinate) -> String {
        todo!()
    }

    /// From a block's coordinate, finds and returns the temperature value [-1.0, 1.0]
    /// at that point.
    pub fn get_temperature(&self, coord: &GlobalCoordinate) -> f32 {
        todo!()
    }
}

impl PartialEq for DefaultBiomeGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.seed == other.seed
    }
}

/// A basic `BlockType` layout for a biome. This influences the basic blocks
/// used to generate a biome.
pub struct BiomeBlockLayout {
    grass: BlockType,
    dirt: BlockType,
    stone: BlockType,
}

impl BiomeBlockLayout {
    /// Creates a 'sandy' set of layers for biomes like beaches and deserts.
    pub fn sandy() -> Self {
        BiomeBlockLayout {
            grass: BlockType::Sand,
            dirt: BlockType::Sandstone,
            stone: BlockType::Stone,
        }
    }

    /// Creates a 'wet' set of layers for biomes like oceans, lakes, and rivers.
    pub fn wet() -> Self {
        Self {
            grass: BlockType::Water,
            dirt: BlockType::Water,
            stone: BlockType::Stone,
        }
    }

    pub fn cold_wet() -> Self {
        Self {
            grass: BlockType::Ice,
            dirt: BlockType::Water,
            stone: BlockType::Stone,
        }
    }
}

impl Default for BiomeBlockLayout {
    fn default() -> Self {
        Self {
            grass: BlockType::Grass,
            dirt: BlockType::Dirt,
            stone: BlockType::Stone,
        }
    }
}

/// A hard-coded list of possible biomes.
/// TODO: remove and make it dynamic
pub enum Biome {
    Forest,
    Plains,
    Taiga, // trees
    Tundra,
    Ocean,
    FrozenOcean,
    Lake,
    FrozenLake,
    River,
    FrozenRiver,
    Desert,
    Bog,   // (wetland)
    Beach, // pretty!!
}

impl Biome {
    /// Given a world to alter, this method will apply biome-specific transformations.
    pub fn create_features(&self, _world: &mut MacawWorld) {
        match self {
            Biome::Forest => {
                // add trees
            }
            Biome::Plains => {
                // add grass
            }
            _ => {
                todo!("Implement biome features")
            }
        }
    }

    /// Returns the layers (`BiomeBlockLayout`) of a biome.
    pub fn layers(&self) -> BiomeBlockLayout {
        match self {
            Biome::Forest | Biome::Plains | Biome::Taiga | Biome::Tundra | Biome::Bog => {
                BiomeBlockLayout::default()
            }
            Biome::Ocean | Biome::Lake | Biome::River => BiomeBlockLayout::wet(),
            Biome::FrozenOcean | Biome::FrozenLake | Biome::FrozenRiver => {
                BiomeBlockLayout::cold_wet()
            }
            Biome::Desert | Biome::Beach => BiomeBlockLayout::sandy(),
        }
    }

    /// Gets the temperature of this biome. range: [-1.0, 1.0]
    pub fn get_temperature(&self) -> f32 {
        match self {
            Biome::Forest => 0.7,
            Biome::Plains => 0.6,
            Biome::Taiga => 0.0,
            Biome::Tundra => -0.6,
            Biome::Ocean => 0.2,
            Biome::FrozenOcean => -0.7,
            Biome::Lake => 0.3,
            Biome::FrozenLake => -0.5,
            Biome::River => 0.5,
            Biome::FrozenRiver => -0.3,
            Biome::Desert => 1.0,
            Biome::Bog => 0.8,
            Biome::Beach => 0.9,
        }
    }
}
