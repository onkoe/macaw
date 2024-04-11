use serde::{Deserialize, Serialize};

use crate::block::{Block, BlockSide, BlockType};

use super::{coordinates::BoundingBox, GlobalCoordinate};
use crate::world::coordinates::ChunkBlockCoordinate;

/// The height, width, and *length* of all chunks.
pub const CHUNK_LENGTH: u8 = 16;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Serialize, Deserialize)]
pub struct Chunk {
    /// A list of blocks within this loaded chunk.
    blocks: Vec<Block>,
    /// The global coordinates of this chunk. This is at 1/16th the scale of
    /// typical block coordinates.
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

    /// Returns the coordinates of the region that this chunk belongs to.
    pub fn region(&self, coords: GlobalCoordinate) -> GlobalCoordinate {
        (coords / 32) * 32
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

    /// Fills in blocks in the chunk given fill bounds.
    /// This overwrites existing blocks!
    pub fn fill(&mut self, block: Block, bounds: BoundingBox<ChunkBlockCoordinate>) {
        for coord in bounds.all_coordinates() {
            self.set_block(block.clone(), coord);
        }
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

    /// Finds the block next to the specified block in this chunk, should it exist.
    pub fn adjacent_block(&self, coord: &ChunkBlockCoordinate, side: BlockSide) -> Option<Block> {
        let (mut x, mut y, mut z) = side.position_offset();

        x += coord.x() as i8;
        y += coord.y() as i8;
        z += coord.z() as i8;

        if (0..=15).contains(&x) && (0..=15).contains(&y) && (0..=15).contains(&z) {
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
        // calculate how far from chunk's min the global coord is
        let relative_pos = |global_coord: i64, chunk_coord: i64| -> Option<u8> {
            let r: u8 = (global_coord - (chunk_coord * CHUNK_LENGTH as i64))
                .try_into()
                .ok()?;
            if (0..CHUNK_LENGTH).contains(&r) {
                Some(r)
            } else {
                None
            }
        };

        // see if new coord is in bounds
        if let (Some(x), Some(y), Some(z)) = (
            relative_pos(coords.x, self.coords.x),
            relative_pos(coords.y, self.coords.y),
            relative_pos(coords.z, self.coords.z),
        ) {
            let local_coord = ChunkBlockCoordinate::new(x, y, z);
            let index = self.block_index(&local_coord);
            self.blocks.get_mut(index)
        } else {
            None
        }
    }

    /// Given a **local coordinate** (i.e. within 16 x 16 x 16), this method
    /// gets the block from the internal `blocks` vector.
    ///
    /// If the block doesn't exist, you'll get `None` back instead.
    pub fn block(&self, local_coord: &ChunkBlockCoordinate) -> Option<Block> {
        self.blocks.get(self.block_index(local_coord)).cloned()
    }

    pub fn next_block(
        &self,
        coordinate: &ChunkBlockCoordinate,
        direction: BlockSide,
    ) -> Option<(Block, ChunkBlockCoordinate)> {
        let c = coordinate.next(&direction);
        if let Some(next_coordinate) = c {
            if let Some(next_block) = self.block(&next_coordinate) {
                return Some((next_block, next_coordinate));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        block::Block,
        world::coordinates::{ChunkBlockCoordinate, GlobalCoordinate},
    };

    use super::Chunk;

    #[test]
    fn check_coordinates() {
        let dirt = Block::new(crate::block::BlockType::Dirt, 0);

        let mut chunk = Chunk::new_filled(dirt.clone(), GlobalCoordinate::new(4, 4, 4));

        chunk.set_block(
            Block::new(crate::block::BlockType::Ice, 0),
            ChunkBlockCoordinate::new(0, 0, 0),
        );

        let sixty_four_g = chunk
            .block_from_global_coords(GlobalCoordinate::new(64, 64, 64))
            .cloned();

        let converted = chunk.global_block_coord(ChunkBlockCoordinate::new(0, 0, 0));
        let cb = chunk.block_from_global_coords(converted).cloned();
        assert_eq!(cb, sixty_four_g);

        let sixty_four_l = chunk.block(&ChunkBlockCoordinate::new(0, 0, 0));

        // we should get the same block back
        assert!(sixty_four_g.is_some());
        assert!(sixty_four_l.is_some());
        assert_eq!(sixty_four_g, sixty_four_l);

        // check block (15, 15, 15)'s global coords
        let b = chunk.global_block_coord(ChunkBlockCoordinate::new(15, 15, 15));
        assert_eq!(b, GlobalCoordinate::new(79, 79, 79));

        // ensure last block is vec's last element
        let c = chunk.block_index(&ChunkBlockCoordinate::new(15, 15, 15));
        assert_eq!(c, chunk.blocks().len() - 1);

        // a cbc will panic if out of bounds. let's generate a bunch with `.blocks()`
        let _ = chunk.blocks();

        // chunks should only contain blocks from [0, 15]. that makes (80, 80, 80) out of bounds!
        assert!(chunk
            .block_from_global_coords(GlobalCoordinate::new(80, 80, 80))
            .is_none());

        // the diagonal chunk over should start there, though!
        let mut next_chunk = Chunk::new_filled(dirt, GlobalCoordinate::new(5, 5, 5));
        assert!(next_chunk
            .block_from_global_coords(GlobalCoordinate::new(80, 80, 80))
            .is_some());
    }
}
