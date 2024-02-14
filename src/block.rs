use bevy::prelude::*;

#[derive(Clone, Component, Debug, Default, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Block {
    pub block_type: BlockType,
    state: u32, // TODO
}

impl Block {
    pub fn new(block_type: BlockType, state: u32) -> Self {
        Self { block_type, state }
    }

    /// If a block type is transparent, like air, liquids, or glass, this
    /// method will let you know.
    pub fn is_transparent(&self) -> bool {
        // if it's one of these, then yes.
        // otherwise, no.
        matches!(self.block_type, BlockType::Air | BlockType::Water)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
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
    pub fn position_offset(&self) -> (i8, i8, i8) {
        match self {
            BlockSide::PositiveX => (1, 0, 0),
            BlockSide::NegativeX => (-1, 0, 0),
            BlockSide::PositiveY => (0, 1, 0),
            BlockSide::NegativeY => (0, -1, 0),
            BlockSide::PositiveZ => (0, 0, 1),
            BlockSide::NegativeZ => (0, 0, -1),
        }
    }
}
