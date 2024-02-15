use std::collections::HashMap;

use self::{
    coordinates::{ChunkBlockCoordinate, GlobalCoordinate},
    error::WorldError,
};

use super::block::Block;
use crate::{
    block::{BlockSide, BlockType},
    world::chunk::Chunk,
};

pub mod chunk;
pub mod coordinates;
pub mod error;
pub mod generation;
pub mod meshing;

/// A representation of a game world. Holds game state and loaded chunks/entities.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct World {
    /// Loaded chunks in the world.
    chunks: HashMap<GlobalCoordinate, Chunk>,
    /// Loaded entities.
    _entities: (),
    /// Spawn location (for players).
    spawn_location: (f32, f32, f32),
}

impl World {
    /// Creates the damn world. I'm like god up in here
    pub fn generate() -> World {
        // create 8 chunks at y = 0 and fill them with cobblestone

        let mut chunks = HashMap::new();

        for x in -3..=3 {
            for z in -3..=3 {
                let coords = GlobalCoordinate::new(x, 0, z);
                tracing::debug!("generating chunk at {:?}", coords);

                chunks.insert(
                    coords,
                    Chunk::new_filled(Block::new(BlockType::Stone, 0), coords),
                );
            }
        }

        World {
            chunks,
            _entities: (),
            spawn_location: (0.0, 18.0, 0.0),
        }
    }

    pub fn one_test_block() -> World {
        let chunk_coordinate = GlobalCoordinate::new(0, 0, 0);
        let mut chunk = Chunk::new(chunk_coordinate);
        chunk.set_block(
            Block::new(BlockType::Grass, 0),
            ChunkBlockCoordinate::new(0, 0, 0),
        );

        let mut map = HashMap::new();
        map.insert(chunk_coordinate, chunk);

        World {
            chunks: map,
            _entities: (),
            spawn_location: (0.0, 0.0, 0.0),
        }
    }

    pub fn generate_test_chunk() -> World {
        let mut chunks = HashMap::new();

        let mut chunk = Chunk::new_filled(Block::new(BlockType::Stone, 0), coordinates::ORIGIN);

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

        chunks.insert(coordinates::ORIGIN, chunk);

        World {
            chunks,
            _entities: (),
            spawn_location: (0.0, 18.0, 0.0),
        }
    }

    /// When given a coordinate, this method will return a mutable a chunk
    /// if that chunk is currently loaded in the world.
    ///
    /// The `coords` are for a chunk, not a block.
    pub fn chunk(&mut self, coords: GlobalCoordinate) -> Option<&mut Chunk> {
        self.chunks.get_mut(&coords)
    }

    /// Tries to place down a chunk at a given location.
    ///
    /// This can fail if there's already a chunk there. Try using `set_chunk()` instead.
    pub fn push_chunk(&mut self, chunk: Chunk, coords: GlobalCoordinate) -> Result<(), WorldError> {
        // early return if we already have those coords stored
        if self.chunks.contains_key(&coords) {
            return Err(WorldError::ChunkAlreadyExists(coords));
        }

        self.set_chunk(chunk, coords);
        Ok(())
    }

    /// Puts down a chunk at `coords`. This will overwrite anything currently there - be careful!
    pub fn set_chunk(&mut self, chunk: Chunk, coords: GlobalCoordinate) {
        self.chunks.insert(coords, chunk);
    }

    /// Given a block's global coordinates, this will find the chunk it's
    /// located in.
    ///
    /// ```
    /// # use macaw::{block::Block, world::{chunk::Chunk, coordinates::GlobalCoordinate, World}};
    /// #
    /// let mut world = World::generate();
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
        &self.chunks
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
                    tracing::warn!(
                        "pushed block side {block_side:#?} with block: {:#?}",
                        &block
                    );
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn chunk_from_block_coords() -> anyhow::Result<()> {
        use crate::world::{coordinates::GlobalCoordinate, World};

        let mut world = World::generate();
        let chunk = world
            .chunk_from_block_coords(GlobalCoordinate::new(47, 16, 15))
            .unwrap();

        assert_eq!(chunk.coords(), GlobalCoordinate::new(3, 1, 0));

        Ok(())
    }
}
