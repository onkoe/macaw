use std::collections::HashSet;

use bevy::{prelude::*, render::mesh::shape::Quad};

use crate::{
    block::{Block, BlockSide, BlockType},
    world::{
        chunk::Chunk,
        coordinates::{BoundingBox, ChunkBlockCoordinate},
    },
};

/// A cluster of related blocks.
pub struct Cluster {
    /// This cluster's single block kind.
    block_type: BlockType,
    /// The coordinates defining the borders of the cluster.
    bounding_box: BoundingBox<ChunkBlockCoordinate>,
}

impl Cluster {
    pub fn new(block_type: BlockType, bounding_box: BoundingBox<ChunkBlockCoordinate>) -> Self {
        Self {
            block_type,
            bounding_box,
        }
    }

    /// Extends the internal bounding box to include `coord`.
    pub fn extend(&mut self, coord: ChunkBlockCoordinate) {
        self.bounding_box.extend(coord);
    }

    /// Provides the positions (corner points) for this cluster.
    pub fn positions(&self) -> Vec<Vec3> {
        // ...
        todo!()
    }

    /// Returns the normals (lighting hints) for this cluster.
    pub fn normals(&self) -> Vec<Vec3> {
        // ...
        todo!()
    }

    /// Returns the UVs (texture hints) for this cluster.
    pub fn uvs(&self) -> Vec<Vec2> {
        // ...
        todo!()
    }

    /// Calculates the indices for this cluster. Indices indicate where the
    /// triangles for each quad are in a 3D model.
    ///
    /// This is fine to be static for now, but will need to be calculated
    /// when blocks like stairs are added.
    pub fn indices(&self) -> Vec<Quad> {
        // try to use Bevy's quads
        // otherwise, can just use the `triangle` module i made last time
        todo!()
    }

    pub fn build(self) -> (BlockType, Transform, Mesh) {
        // ...
        todo!()
    }
}

// in another module (file)!
// this can use the previous neighbor-chasing algo from the first attempt
impl Chunk {
    // ...

    /// Given a block, this method chases all of its neighbors until there are
    /// none of the same type.
    fn chase_neighbors(
        &self,
        starting_block: &Block,
        starting_coordinate: &ChunkBlockCoordinate,
        checked_blocks: &mut HashSet<ChunkBlockCoordinate>,
    ) -> Option<Cluster> {
        let mut cluster = Cluster::new(
            starting_block.block_type,
            BoundingBox::new_point(*starting_coordinate), // a bounding box that has no area
        );

        // these can change, though only locally
        let mut starting_block = starting_block.clone();
        let mut starting_coordinate = *starting_coordinate;

        // use positive only; we're starting from the lowest coordinate
        let directions = [
            BlockSide::PositiveX,
            BlockSide::PositiveY,
            BlockSide::PositiveZ,
        ];

        // check in all directions
        for direction in directions {
            while let Some((neighbor_block, neighbor_coordinate)) =
                self.next_block(&starting_block, &starting_coordinate, direction)
            {
                if starting_block.same_kind_as(&neighbor_block)
                    && !checked_blocks.contains(&neighbor_coordinate)
                {
                    cluster.extend(neighbor_coordinate);
                    checked_blocks.insert(neighbor_coordinate);
                    // blocks_to_check.remove(self.block_index(&neighbor_coordinate)); // TODO: check this shit

                    starting_block = neighbor_block.clone();
                    starting_coordinate = neighbor_coordinate;
                } else {
                    return None;
                }
            }
        }

        // TODO: test this method and make sure it returns None when it should
        Some(cluster)
    }

    /// Creates a list of the chunk's block `Cluster`s to be rendered.
    pub fn cluster(&self) -> Vec<Cluster> {
        let mut blocks_to_check = self.blocks();
        let mut completed_blocks = HashSet::new();
        let mut clusters = Vec::new();

        for (coordinate, block) in blocks_to_check.iter_mut() {
            // don't check blocks we've finished
            if completed_blocks.contains(coordinate) {
                continue;
            }

            if let Some(cluster) = self.chase_neighbors(block, coordinate, &mut completed_blocks) {
                clusters.push(cluster);
            }
        }

        clusters
    }

    // ...
}

/// A function called by Bevy's renderer when it's collected a `World`'s `Cluster`s.
///
/// This will eventually take a cluster to render (i.e. chunk coordinate) so
/// we don't update the whole world.
pub fn render_clusters(commands: &mut Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let world = crate::world::World::generate();

    let clusters = world
        .chunks()
        .iter()
        .flat_map(|(_, f)| f.cluster())
        .map(|c| c.build())
        .collect::<Vec<_>>();

    for (_block_type, transform, mesh) in clusters {
        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            transform,
            // material: `get_texture_from_block_type(block_type)`,
            ..Default::default()
        });
    }
    // render things
}
