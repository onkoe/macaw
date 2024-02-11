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
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[allow(unused)]
pub enum BlockType {
    Air,
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
