use crate::block::Block;

use super::GlobalCoordinate;
use crate::{world::coordinates::ChunkBlockCoordinate, world::coordinates::GlobalCoordinate2D};

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

    fn local_block_index(&self, local_coord: &ChunkBlockCoordinate) -> usize {
        (local_coord.z() as usize) * (16 * 16)
            + (local_coord.y() as usize) * 16
            + (local_coord.x() as usize)
    }

    /// Given a **local coordinate** (i.e. within 16 x 16 x 16), this method
    /// returns the block from the internal `blocks` vector.
    ///
    /// If the block doesn't exist, you'll get `None` back instead.
    pub fn get_local_block(&self, local_coord: &ChunkBlockCoordinate) -> Option<Block> {
        // TODO: return a result instead of option lmao
        self.blocks
            .get(self.local_block_index(local_coord))
            .cloned()
    }

    /// Given a local coordinate, this method returns a list of blocks that
    /// are surrounding the given block.
    pub fn adjacent_blocks(&self, coord: &ChunkBlockCoordinate) -> Vec<Block> {
        let mut blocks = Vec::new();

        let movements: [(i8, i8, i8); 6] = [
            (1, 0, 0),  // right
            (-1, 0, 0), // left
            (0, 1, 0),  // up
            (0, -1, 0), // down
            (0, 0, 1),  // forward
            (0, 0, -1), // back
        ];

        // for each adjacent block, push to blocks vec
        for (mx, my, mz) in movements {
            let (nx, ny, nz) = (
                coord.x() as i8 + mx,
                coord.y() as i8 + my,
                coord.z() as i8 + mz,
            );

            if (0..16).contains(&nx) && (0..16).contains(&ny) && (0..16).contains(&nz) {
                if let Some(block) =
                    self.get_local_block(&ChunkBlockCoordinate::new(nx as u8, ny as u8, nz as u8))
                {
                    blocks.push(block);
                }
            }
        }

        blocks
    }

    /// Given a local block coordinate, returns that block's global coordinate.
    pub fn global_block_coord(&self, local_coord: ChunkBlockCoordinate) -> GlobalCoordinate {
        GlobalCoordinate {
            x: self.coords.x * 16 + local_coord.x() as u64,
            y: self.coords.y * 16 + local_coord.y() as u64,
            z: self.coords.z * 16 + local_coord.z() as u64,
        }
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
