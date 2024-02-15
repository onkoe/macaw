use crate::block::{Block, BlockSide, BlockType};

use super::GlobalCoordinate;
use crate::{world::coordinates::ChunkBlockCoordinate, world::coordinates::GlobalCoordinate2D};

/// The height, width, and *length* of all chunks.
pub const CHUNK_LENGTH: u8 = 16;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct Chunk {
    blocks: Vec<Block>,
    coords: GlobalCoordinate,
}

impl Chunk {
    /// Creates a new, 'empty' chunk (where all elements are air).
    pub fn new(coords: GlobalCoordinate) -> Self {
        Chunk {
            blocks: vec![Block::default(); 16 * 16 * 16],
            coords,
        }
    }

    /// Creates a new chunk that's filled entirely with the given block.
    pub fn new_filled(block: Block, coords: GlobalCoordinate) -> Self {
        Chunk {
            blocks: vec![block; 16 * 16 * 16],
            coords,
        }
    }

    /// Returns the coordinates of the Chunk.
    pub fn coords(&self) -> GlobalCoordinate {
        self.coords
    }

    /// Gives out a list of blocks in the chunk with their coordinates.
    ///
    /// This doesn't include `None` 'blocks' or Air blocks.
    pub fn blocks(&self) -> Vec<(ChunkBlockCoordinate, Block)> {
        let mut blocks = Vec::new();

        for x in 0..CHUNK_LENGTH {
            for y in 0..CHUNK_LENGTH {
                for z in 0..CHUNK_LENGTH {
                    let coord = ChunkBlockCoordinate::new(x, y, z);

                    if let Some(b) = self.block(&coord) {
                        if b.block_type != BlockType::Air {
                            blocks.push((coord, b));
                        }
                    }
                }
            }
        }

        blocks
    }

    /// Sets the `Block` at the given `ChunkBlockCoordinate`.
    pub fn set_block(&mut self, block: Block, coord: ChunkBlockCoordinate) {
        let index = self.block_index(&coord);
        self.blocks[index] = block;
    }

    /// Given a local coordinate, this method returns a list of blocks that
    /// are surrounding the given block.
    pub fn adjacent_blocks(&self, coord: &ChunkBlockCoordinate) -> Vec<Block> {
        let mut blocks = Vec::new();

        use BlockSide::*; // this is code smell but it makes my eyes happier

        let movements = [
            PositiveX, NegativeX, PositiveY, NegativeY, PositiveZ, NegativeZ,
        ];

        // for each adjacent block, push to blocks vec
        for side in movements {
            if let Some(block) = self.adjacent_block(coord, side) {
                blocks.push(block);
            }
        }

        blocks
    }

    /// Finds the block next to the specified block in this chunk, should it
    /// exist.
    pub fn adjacent_block(&self, coord: &ChunkBlockCoordinate, side: BlockSide) -> Option<Block> {
        let (mut x, mut y, mut z) = side.position_offset();

        x += coord.x() as i8;
        y += coord.y() as i8;
        z += coord.z() as i8;

        if (0..16).contains(&x) && (0..16).contains(&y) && (0..16).contains(&z) {
            if let Some(block) = self.block(&ChunkBlockCoordinate::new(x as u8, y as u8, z as u8)) {
                return Some(block);
            }
        }
        None
    }

    /// Given a local block coordinate, returns the block's index in the
    /// `blocks` vector.
    pub(crate) fn block_index(&self, local_coord: &ChunkBlockCoordinate) -> usize {
        (local_coord.z() as usize) * (16 * 16)
            + (local_coord.y() as usize) * 16
            + (local_coord.x() as usize)
    }

    /// Given a local block coordinate, returns that block's global coordinate.
    pub fn global_block_coord(&self, local_coord: ChunkBlockCoordinate) -> GlobalCoordinate {
        let local_to_global = |cc: i64, l: u8| -> i64 { cc * 16 + l as i64 };

        GlobalCoordinate {
            x: local_to_global(self.coords.x, local_coord.x()),
            y: local_to_global(self.coords.y, local_coord.y()),
            z: local_to_global(self.coords.z, local_coord.z()),
        }
    }

    /// Given a global block coordinate, returns that block.
    pub fn block_from_global_coords(&mut self, coords: GlobalCoordinate) -> Option<&mut Block> {
        // global to local value
        let gtl = |coords: i64| -> u8 {
            coords
                .rem_euclid(16)
                .try_into()
                .expect("dividing by 16 will never have a remainder above 15")
        };

        let index = self.block_index(&ChunkBlockCoordinate::new(
            gtl(coords.x),
            gtl(coords.y),
            gtl(coords.z),
        ));

        self.blocks.get_mut(index)
    }

    /// Given a **local coordinate** (i.e. within 16 x 16 x 16), this method
    /// gets the block from the internal `blocks` vector.
    ///
    /// If the block doesn't exist, you'll get `None` back instead.
    pub fn block(&self, local_coord: &ChunkBlockCoordinate) -> Option<Block> {
        self.blocks.get(self.block_index(local_coord)).cloned()
    }

    /// Returns all of the global 2D block coordinates within this chunk. (x, z)
    pub fn all_global_block_coordinates(&self) -> Vec<GlobalCoordinate2D> {
        let mut v = Vec::new();

        for x in 0..16 {
            for z in 0..16 {
                v.push(GlobalCoordinate2D { x, z });
            }
        }

        v
    }
}
