use bevy::math::Vec3;
use std::collections::HashMap;

use super::block::Block;
use crate::block::BlockType;

pub mod meshing;

pub struct World {
    chunks: HashMap<GlobalCoordinate, Chunk>,
}

impl World {
    pub fn get_chunk(&self, coords: GlobalCoordinate) -> Option<Chunk> {
        self.chunks.get(&coords).cloned()
    }

    pub fn chunks(&self) -> &HashMap<GlobalCoordinate, Chunk> {
        &self.chunks
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]

pub struct Chunk {
    blocks: Vec<Block>,
    coords: GlobalCoordinate,
}

/// A coordinate in a chunk. Chunks are 16x16x16, so all values must be in the
/// range [0, 15].
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct ChunkBlockCoordinate {
    x: u8,
    y: u8,
    z: u8,
}

impl ChunkBlockCoordinate {
    /// A checked constructor for `ChunkBlockCoordinate`.
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        assert!((0..16).contains(&x));
        assert!((0..16).contains(&y));
        assert!((0..16).contains(&z));

        Self { x, y, z }
    }

    pub fn new_from_tuple(xyz: (u8, u8, u8)) -> Self {
        Self::new(xyz.0, xyz.1, xyz.2)
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }

    pub fn z(&self) -> u8 {
        self.z
    }

    /// If any of the contained coordinates match a given closure's comparison,
    /// this function will return true.
    ///
    /// Usage:
    ///
    /// ```
    /// # use macaw::world::ChunkBlockCoordinate;
    /// #
    /// let coord = ChunkBlockCoordinate::new(1, 2, 3);
    ///
    /// assert!(coord.any(|a| a == 3));
    /// ```
    pub fn any<F>(&self, compare_to: F) -> bool
    where
        F: Fn(u8) -> bool,
    {
        compare_to(self.x) || compare_to(self.y) || compare_to(self.z)
    }

    /// Destructures self into (x, y, z) coordinates.
    pub fn free(self) -> (u8, u8, u8) {
        (self.x, self.y, self.z)
    }
}

/// A coordinate found in the world - globally.
///
/// These can represent a block or even a chunk!
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct GlobalCoordinate {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl GlobalCoordinate {
    /// Creates a `GlobalCoordinate`.
    pub fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }

    pub fn to_vec3(self) -> Vec3 {
        Vec3 {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        }
    }
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
        (local_coord.z as usize) * (16 * 16)
            + (local_coord.y as usize) * 16
            + (local_coord.x as usize)
    }

    /// Given a **local coordinate** (i.e. within 16 x 16 x 16), this method
    /// returns the block from the internal `blocks` vector.
    ///
    /// If the block doesn't exist, you'll get `None` back instead.
    pub(crate) fn get_local_block(&self, local_coord: &ChunkBlockCoordinate) -> Option<Block> {
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
            let (nx, ny, nz) = (coord.x as i8 + mx, coord.y as i8 + my, coord.z as i8 + mz);

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

    pub fn global_block_coords(
        &self,
        local_coord: ChunkBlockCoordinate,
    ) -> Option<GlobalCoordinate> {
        Some(GlobalCoordinate {
            x: self.coords.x * 16 + local_coord.x as u64,
            y: self.coords.y * 16 + local_coord.y as u64,
            z: self.coords.z * 16 + local_coord.z as u64,
        })
    }
}

/// Creates the damn world. I'm like god up in here
pub fn generate() -> World {
    // create 8 chunks at y = 0 and fill them with cobblestone

    let mut chunks = HashMap::new();

    for x in 0..3 {
        for z in 0..3 {
            let coords = GlobalCoordinate::new(x, 0, z);
            tracing::debug!("generating chunk at {:?}", coords);

            chunks.insert(
                coords,
                Chunk::new_filled(Block::new(BlockType::Stone, 0), coords),
            );
        }
    }

    World { chunks }
}
