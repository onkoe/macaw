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
