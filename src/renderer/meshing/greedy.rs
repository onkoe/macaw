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
    ] {
        for (coord, block) in block_list.iter_mut() {
            // avoid making multiple meshes per block
            if checked_blocks.contains(coord) {
                // tracing::warn!("block at {coord} already checked!");
                continue;
            } else {
                let mut offset: u32 = 1; // each mesh needs to add to this offset to show how many faces we got!

                if let Some(mesh_construct) = expand_and_combine(
                    block,
                    coord,
                    chunk,
                    &mut checked_blocks,
                    direction,
                    &mut offset,
                ) {
                    mesh_constructs.push(mesh_construct);
                }
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
) -> Option<MeshConstruct> {
    tracing::info!("expand and combine! on block {block_coord:?}");

    let mut root_coordinate = *block_coord;

    // mark first block as checked once we've been through all directions
    if direction == BlockSide::PositiveZ {
        checked_blocks.insert(root_coordinate);
    }

    let mut mcon = MeshConstruct::from_block(block, root_coordinate, chunk)?;

    while let Some(neighbor_coordinate) = root_coordinate.next(&direction) {
        if let Some(neighbor) = chunk.adjacent_block(&root_coordinate, direction) {
            if block.same_kind_as(&neighbor) && !checked_blocks.contains(&neighbor_coordinate) {
                // combine and mark the neighbor block
                if let Some(neighbor_mesh_construct) =
                    MeshConstruct::from_block(&neighbor, neighbor_coordinate, chunk)
                {
                    mcon.combine(
                        neighbor_mesh_construct,
                        block_coord.min(&neighbor_coordinate).to_vec3(),
                        *offset,
                    );
                    *offset += 1;
                    checked_blocks.insert(neighbor_coordinate);

                    root_coordinate = neighbor_coordinate;
                }
            } else {
                // give up if we've already hit this block. or we just cant
                break;
            }
        } else {
            break;
        }
    }

    Some(mcon)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use tracing_subscriber::util::SubscriberInitExt;

    use super::expand_and_combine;
    use crate::{
        block::*,
        world::{chunk::Chunk, coordinates::*},
    };

    fn setup_logger() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .finish()
            .init();
    }

    #[test]
    fn expand_and_combine_empty() {
        setup_logger();
        let chunk = Chunk::new_filled(Block::new(BlockType::Air, 0), GlobalCoordinate::ORIGIN);

        let origin_coord = ChunkBlockCoordinate::ORIGIN;
        let origin_block = chunk.block(&origin_coord).unwrap();

        let mut checked_blocks = HashSet::new();
        let mut offset = 0;

        let mesh = expand_and_combine(
            &origin_block,
            &origin_coord,
            &chunk,
            &mut checked_blocks,
            BlockSide::PositiveX,
            &mut offset,
        );

        assert!(mesh.is_none());
    }

    #[test]
    fn expand_and_combine_all_stone() {
        setup_logger();
        let chunk = Chunk::new_filled(Block::new(BlockType::Stone, 0), GlobalCoordinate::ORIGIN);

        let origin_coord = ChunkBlockCoordinate::ORIGIN;
        let origin_block = chunk.block(&origin_coord).unwrap();

        let mut checked_blocks = HashSet::new();
        let mut offset = 0;

        let mesh = expand_and_combine(
            &origin_block,
            &origin_coord,
            &chunk,
            &mut checked_blocks,
            BlockSide::PositiveX,
            &mut offset,
        )
        .expect("should get back one mesh")
        .build();

        assert_eq!(
            mesh.indices().unwrap().len(),
            origin_block.indices(0).len() * 16_usize.pow(2)
        );
    }
}
