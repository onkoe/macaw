//! # Greedy
//!
//! A greedy meshing implementation for Macaw.

use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    block::{Block, BlockSide},
    renderer::meshing::MeshConstruct,
    world::{chunk::Chunk, coordinates::ChunkBlockCoordinate},
};

pub fn meshing(chunk: &Chunk) -> Vec<(Transform, Mesh)> {
    let mut block_list = chunk.blocks().clone();
    let mut mesh_constructs = Vec::<MeshConstruct>::new();
    let mut checked_blocks = HashSet::<ChunkBlockCoordinate>::new();
    let mut final_meshes = Vec::<(Transform, Mesh)>::new();

    for direction in [
        BlockSide::PositiveX,
        BlockSide::PositiveY,
        BlockSide::PositiveZ,
        BlockSide::NegativeX,
        BlockSide::NegativeY,
        BlockSide::NegativeZ,
    ] {
        for (coord, block) in block_list.iter_mut() {
            // avoid making multiple meshes per block
            if checked_blocks.contains(coord) {
                tracing::warn!("block at {coord} already checked!");
                continue;
            } else {
                let mut offset: u32 = 1; // each mesh needs to add to this offset to show how many faces we got!

                let mesh_construct = expand_and_combine(
                    block,
                    coord,
                    chunk,
                    &mut checked_blocks,
                    direction,
                    &mut offset,
                );
                mesh_constructs.push(mesh_construct);
            }
        }
    }

    // finishing up
    for construct in mesh_constructs {
        final_meshes.push((
            Transform {
                translation: chunk.global_block_coord(construct.transform).to_vec3(),
                ..Default::default()
            },
            construct.build(),
        ));
    }

    final_meshes
}

/// Starting at the given `block`, this function checks all of its neighbors to create a MeshConstruct.
///
/// It will then append the blocks it was able to capture to the `checked_blocks` HashSet.
fn expand_and_combine(
    block: &Block,
    block_coord: &ChunkBlockCoordinate,
    chunk: &Chunk,
    checked_blocks: &mut HashSet<ChunkBlockCoordinate>,
    direction: BlockSide,
    offset: &mut u32,
) -> MeshConstruct {
    tracing::info!("expand and combine! on block {block_coord:?}");

    let mut selected_coordinate = *block_coord;

    if direction == BlockSide::PositiveZ {
        // mark first block as checked, no matter what
        checked_blocks.insert(selected_coordinate);
    }

    let mut mcon = MeshConstruct::from_block(block, selected_coordinate, chunk);

    while let Some(next_coordinate) = selected_coordinate.next(&direction) {
        if let Some(neighbor) = chunk.adjacent_block(&selected_coordinate, direction) {
            if block.same_kind_as(&neighbor) && !checked_blocks.contains(&next_coordinate) {
                // combine and mark the neighbor block
                mcon.combine(
                    MeshConstruct::from_block(&neighbor, next_coordinate, chunk),
                    block_coord.min(&next_coordinate).to_vec3(),
                    *offset,
                );
                *offset += 1;
                checked_blocks.insert(next_coordinate);

                selected_coordinate = next_coordinate;
            } else {
                // give up if we've already hit this block. or we just cant
                tracing::warn!(
                    "couldn't combine blocks: {:#?} at coords {} \n \n and \n \n {:#?}@{}",
                    block,
                    block_coord,
                    neighbor,
                    selected_coordinate
                );
                break;
            }
        } else {
            break;
        }
    }

    mcon
}
