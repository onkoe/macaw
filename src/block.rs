use bevy::prelude::*;

use crate::renderer::meshing::triangle::{Face, Triangle};

#[derive(Clone, Component, Debug, Default, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Block {
    pub block_type: BlockType,
    pub state: u32, // TODO
}

impl Block {
    pub const fn new(block_type: BlockType, state: u32) -> Self {
        Self { block_type, state }
    }

    /// If a block type is transparent, like air, liquids, or glass, this
    /// method will let you know.
    pub const fn is_transparent(&self) -> bool {
        // if it's one of these, then yes.
        // otherwise, no.
        matches!(self.block_type, BlockType::Air | BlockType::Water)
    }

    /// Checks to see if this block has the same kind and state as another block.
    pub fn same_kind_as(&self, other: &Block) -> bool {
        self.block_type == other.block_type && self.state == other.state
    }

    /// Provides the positions (corner points) of the block.
    ///
    /// These are currently static, but may be altered with additional block
    /// shapes, like stairs or slabs.
    #[rustfmt::skip]
    pub fn positions(&self) -> Vec<Vec3> {
    vec![
        // Front face
        Vec3::new(-0.5, -0.5,  0.5), Vec3::new(0.5, -0.5,  0.5), Vec3::new(0.5,  0.5,  0.5), Vec3::new(-0.5,  0.5, 0.5),
        // Back face
        Vec3::new(-0.5, -0.5, -0.5), Vec3::new(-0.5,  0.5, -0.5), Vec3::new(0.5,  0.5, -0.5), Vec3::new(0.5, -0.5, -0.5),
        // Top face
        Vec3::new(-0.5,  0.5, -0.5), Vec3::new(-0.5,  0.5,  0.5),  Vec3::new(0.5,  0.5,  0.5), Vec3::new(0.5,  0.5, -0.5),
        // Bottom face
        Vec3::new(-0.5, -0.5, -0.5), Vec3::new(0.5, -0.5, -0.5), Vec3::new(0.5, -0.5,  0.5), Vec3::new(-0.5, -0.5,  0.5),
        // Right face
        Vec3::new(0.5, -0.5, -0.5), Vec3::new( 0.5,  0.5, -0.5), Vec3::new(0.5,  0.5,  0.5), Vec3::new(0.5, -0.5,  0.5),
        // Left face
        Vec3::new(-0.5, -0.5, -0.5), Vec3::new(-0.5, -0.5,  0.5), Vec3::new(-0.5,  0.5,  0.5), Vec3::new(-0.5,  0.5, -0.5),
    ]
    }

    /// Returns the normals (lighting hints) for this block.
    ///
    /// These are currently static. See `positions()` for more.
    pub fn normals(&self) -> Vec<Vec3> {
        BlockSide::all_normals()
    }

    /// Returns the UVs (texture bounds) for this block.
    pub fn uvs(&self) -> Vec<Vec2> {
        let mut uvs = vec![];

        // for each face, add uvs
        for _ in 0..6 {
            uvs.extend([
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 1.0),
            ]);
        }
        uvs
    }

    /// Gives the indices (reusable verticies) for this block.
    ///
    /// These can be combined with other block's indices, filtering out
    /// verticies that appear on the same edge as others that are futher apart.
    #[rustfmt::skip]
    pub fn indices(&self, index_offset: u32) -> Vec<Face> {
        // TODO: this'll need presets for different kinds of blocks, like stairs

        vec![
            // Front face
            Face::new(Triangle::new(0, 1, 2), Triangle::new(0, 2, 3), index_offset),
            // Back face
            Face::new(Triangle::new(4, 5, 6), Triangle::new(4, 6, 7), index_offset),
            // Top face
            Face::new(Triangle::new(8, 9, 10), Triangle::new(8, 10, 11), index_offset),
            // Bottom face
            Face::new(Triangle::new(12, 13, 14), Triangle::new(12, 14, 15), index_offset),
            // Right face
            Face::new(Triangle::new(16, 17, 18), Triangle::new(16, 18, 19), index_offset),
            // Left face
            Face::new(Triangle::new(20, 21, 22), Triangle::new(20, 22, 23), index_offset),
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[allow(unused)]
pub enum BlockType {
    Air,
    Water,
    Stone,
    Log,
    Dirt,
    Grass,
}

impl Default for BlockType {
    fn default() -> Self {
        Self::Air
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum BlockSide {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}

impl BlockSide {
    /// Returns the position offset of a block's neighbor on this side.
    ///
    /// For example, a block at (x: 5, y: 68, z: 70) looking at the blockside
    /// of their +X neighbor will get back (6, 68, 70).
    pub const fn position_offset(&self) -> (i8, i8, i8) {
        match self {
            BlockSide::PositiveX => (1, 0, 0),
            BlockSide::NegativeX => (-1, 0, 0),
            BlockSide::PositiveY => (0, 1, 0),
            BlockSide::NegativeY => (0, -1, 0),
            BlockSide::PositiveZ => (0, 0, 1),
            BlockSide::NegativeZ => (0, 0, -1),
        }
    }

    /// Returns all BlockSides.
    pub const fn all_sides() -> [BlockSide; 6] {
        [
            BlockSide::PositiveX,
            BlockSide::NegativeX,
            BlockSide::PositiveY,
            BlockSide::NegativeY,
            BlockSide::PositiveZ,
            BlockSide::NegativeZ,
        ]
    }

    /// Returns all normals from each BlockSide.
    pub fn all_normals() -> Vec<Vec3> {
        BlockSide::all_sides()
            .iter()
            .flat_map(|side| vec![side.normal(); 6]) // 6 normals for each side
            .collect()
    }

    /// Returns all normals from each BlockSide.
    pub fn normals(&self) -> Vec<Vec3> {
        // FIXME: this won't work for all kinds of blocks.
        // some blocks may need more normals

        [
            Vec3::new(1.0, 0.0, 0.0),  // PositiveX
            Vec3::new(-1.0, 0.0, 0.0), // NegativeX
            Vec3::new(0.0, 1.0, 0.0),  // PositiveY
            Vec3::new(0.0, -1.0, 0.0), // NegativeY
            Vec3::new(0.0, 0.0, 1.0),  // PositiveZ
            Vec3::new(0.0, 0.0, -1.0), // NegativeZ
        ]
        .iter()
        .flat_map(|&normal| vec![normal; 6])
        .collect() // 6 vertices per face
    }

    /// Returns the matching normal (lighting hint) for a given BlockSide.
    pub const fn normal(&self) -> Vec3 {
        match self {
            BlockSide::PositiveX => Vec3::new(1.0, 0.0, 0.0),
            BlockSide::NegativeX => Vec3::new(-1.0, 0.0, 0.0),
            BlockSide::PositiveY => Vec3::new(0.0, 1.0, 0.0),
            BlockSide::NegativeY => Vec3::new(0.0, -1.0, 0.0),
            BlockSide::PositiveZ => Vec3::new(0.0, 0.0, 1.0),
            BlockSide::NegativeZ => Vec3::new(0.0, 0.0, -1.0),
        }
    }
}
