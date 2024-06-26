use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use bevy::utils::Uuid;

use self::{
    coordinates::{BoundingBox, GlobalCoordinate},
    error::WorldError,
    generation::{generators::blank::BlankGenerator, Generator},
    loader::{WorldLoader, WorldLoadingError},
    metadata::WorldMetadata,
    save::WorldSave,
};

use super::block::Block;
use crate::{block::BlockSide, world::chunk::Chunk};

pub mod chunk;
pub mod coordinates;
pub mod error;
pub mod generation;
pub mod loader;
pub mod meshing;
pub mod metadata;
pub mod region;
pub mod save;

/// A representation of a game world. Holds game state and loaded chunks/entities.
#[derive(Debug)]
pub struct MacawWorld {
    /// The unique, user-given name of the world.
    metadata: Arc<WorldMetadata>,
    /// Loaded chunks, etc. in the world.
    /// This allows to save and load!
    loader: WorldLoader,
    /// The entities currently inhabiting this world.
    entities: HashSet<()>,
    /// Spawn location (for players).
    spawn_location: GlobalCoordinate,
}

impl MacawWorld {
    /// The name of this world.
    pub fn name(&self) -> String {
        self.metadata.name().to_string()
    }

    /// The metadata of this world.
    pub fn metadata(&self) -> Arc<WorldMetadata> {
        self.metadata.clone()
    }

    /// Saves the world, like I did when I was born.
    pub fn save(&mut self) -> Result<(), WorldLoadingError> {
        self.metadata.write_to_disk()?;
        self.loader.write_chunks()?;
        // TODO: write mobs/other world factors..?

        Ok(())
    }

    /// Loads a world from disk.
    pub fn load(metadata: Arc<WorldMetadata>) -> Result<Self, WorldLoadingError> {
        let save = WorldSave::new(metadata.clone())?;

        // TODO: remove hardcoded bounding box when the player can actually generate things
        let loader = WorldLoader::new_with_save(
            save,
            BoundingBox::new(
                GlobalCoordinate::new(-16, -16, -16),
                GlobalCoordinate::new(16, 16, 16),
            ),
        )?;

        Ok(MacawWorld {
            metadata,
            loader,
            entities: HashSet::new(),
            spawn_location: GlobalCoordinate::ORIGIN,
        })
    }

    /// When given a coordinate, this method will return a mutable chunk
    /// if that chunk is currently loaded in the world.
    ///
    /// The `coords` are for a chunk, not a block.
    pub fn chunk(&mut self, coords: GlobalCoordinate) -> Option<&mut Chunk> {
        self.loader.chunks_mut().get_mut(&coords)
    }

    /// Tries to place down a chunk at a given location.
    ///
    /// This can fail if there's already a chunk there. Try using `set_chunk()` instead.
    pub fn push_chunk(&mut self, chunk: Chunk, coords: GlobalCoordinate) -> Result<(), WorldError> {
        // early return if we already have those coords stored
        if self.loader.chunks_mut().contains_key(&coords) {
            return Err(WorldError::ChunkAlreadyExists(coords));
        }

        self.set_chunk(chunk, coords);
        Ok(())
    }

    /// Puts down a chunk at `coords`. This will overwrite anything currently there - be careful!
    pub fn set_chunk(&mut self, chunk: Chunk, coords: GlobalCoordinate) {
        self.loader.chunks_mut().insert(coords, chunk);
    }

    /// Given a block's global coordinates, this will find the chunk it's
    /// located in.
    ///
    /// ```
    /// # use macaw::{block::Block, world::{chunk::Chunk, coordinates::GlobalCoordinate, MacawWorld}};
    /// #
    /// let mut world = MacawWorld::generate();
    /// let chunk = world.chunk_from_block_coords(GlobalCoordinate::new(0, 0, 0)).unwrap();
    ///
    /// assert_eq!(chunk.coords(), GlobalCoordinate::new(0, 0, 0));
    /// ```
    pub fn chunk_from_block_coords(&mut self, coords: GlobalCoordinate) -> Option<&mut Chunk> {
        let chunk_coords = GlobalCoordinate {
            x: coords.x / 16,
            y: coords.y / 16,
            z: coords.z / 16,
        };

        self.chunk(chunk_coords)
    }

    /// Returns a reference to the internal chunks hashmap.
    pub fn chunks(&self) -> &HashMap<GlobalCoordinate, Chunk> {
        self.loader.chunks_ref()
    }

    /// Given global coordinates, returns a block if there's one present.
    pub fn block_from_coords(&mut self, coords: GlobalCoordinate) -> Option<&mut Block> {
        let chunk = self.chunk_from_block_coords(coords)?;
        chunk.block_from_global_coords(coords)
    }

    /// Checks if a block at the given coordinates has exposed sides. If it does,
    /// returns the sides.
    pub fn block_exposed_sides(&mut self, coords: GlobalCoordinate) -> Vec<BlockSide> {
        let adjacent_blocks = self.adjacent_blocks(coords);
        let mut v = Vec::new();

        for (block_side, block) in adjacent_blocks {
            if let Some(ref b) = block {
                if b.is_transparent() {
                    v.push(block_side);
                }
            } else {
                v.push(block_side);
            }
        }

        v
    }

    /// Returns a list of blocks adjacent to a block at the given coordinates
    /// along with the side the adjacent block is at.
    pub fn adjacent_blocks(&mut self, coords: GlobalCoordinate) -> Vec<(BlockSide, Option<Block>)> {
        let mut blocks = Vec::new();

        let movements: [(BlockSide, i64, i64, i64); 6] = [
            (BlockSide::PositiveX, 1, 0, 0),  // right
            (BlockSide::NegativeX, -1, 0, 0), // left
            (BlockSide::PositiveY, 0, 1, 0),  // up
            (BlockSide::NegativeY, 0, -1, 0), // down
            (BlockSide::PositiveZ, 0, 0, 1),  // forward
            (BlockSide::NegativeZ, 0, 0, -1), // back
        ];

        // for each adjacent block, push to blocks vec
        for (block_side, mx, my, mz) in movements {
            let (nx, ny, nz) = (coords.x + mx, coords.y + my, coords.z + mz);

            let neighbor = self.block_from_coords(GlobalCoordinate::new(nx, ny, nz));
            blocks.push((block_side, neighbor.cloned()));
        }

        blocks
    }

    fn default() -> Self {
        let metadata = Arc::new(WorldMetadata::new_now(
            Uuid::new_v4().to_string(),
            0,
            BlankGenerator.id(),
        ));

        let loader = WorldLoader::new(metadata.clone()).unwrap();

        MacawWorld {
            metadata,
            loader,
            entities: HashSet::new(),
            spawn_location: GlobalCoordinate::ORIGIN,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::world::generation::{generators::default::DefaultGenerator, Generator};
    use bevy::tasks::block_on;

    #[test]
    fn chunk_from_block_coords() -> anyhow::Result<()> {
        use crate::world::coordinates::GlobalCoordinate;

        let mut world = block_on(DefaultGenerator::new(0).pre_generate(0));
        let chunk = world
            .chunk_from_block_coords(GlobalCoordinate::new(47, 16, 15))
            .unwrap();

        assert_eq!(chunk.coords(), GlobalCoordinate::new(3, 1, 0));

        Ok(())
    }
}
