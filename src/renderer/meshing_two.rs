//! the plan:
//! - create a list of blocks (`Cluster`) for each block type in the chunk
//!     - clusters store:
//!         - a `BlockType`
//!         - a `BoundingBox` representing the area they exist in (two coordinates)
//!     - this can literally be a method on Chunk. like `pub fn cluster(&self) -> Vec<Cluster>;`
//!         - `cluster()` should basically do the meshing part for us
//!     - `Clusters` can easily calculate their positions, normals, uvs, and indices from the
//!       properties of their held `BlockType` and their size.
//!         - this part can (and SHOULD) be mathematically proven
//! - given this list of `Cluster`s, we can easily iterate over them + create their meshes
//!
//!

use std::{collections::HashSet, thread::sleep, time::Duration};

use bevy::prelude::*;

use crate::{
    block::{Block, BlockSide, BlockType},
    world::{
        chunk::Chunk,
        coordinates::{BoundingBox, ChunkBlockCoordinate},
    },
};

/// A cluster of related blocks.
#[derive(Clone, Hash, PartialEq, PartialOrd)]
pub struct Cluster<'a> {
    /// This cluster's single block kind.
    block_type: BlockType,
    /// The coordinates defining the borders of the cluster.
    bounding_box: BoundingBox<ChunkBlockCoordinate>,
    /// A reference to the chunk that the cluster's blocks live in.
    chunk: &'a Chunk,
}

impl<'a> Cluster<'a> {
    pub fn new(
        block_type: BlockType,
        bounding_box: BoundingBox<ChunkBlockCoordinate>,
        chunk: &'a Chunk,
    ) -> Self {
        Self {
            block_type,
            bounding_box,
            chunk,
        }
    }

    /// Extends the internal bounding box to include `coord`.
    pub fn extend(&mut self, coord: ChunkBlockCoordinate) {
        self.bounding_box.extend(coord);
    }

    pub fn build(self) -> (BlockType, Transform, Mesh) {
        let bb = self.bounding_box.to_global(self.chunk);

        let mesh = bb.as_cuboid().mesh();
        let transform = Transform {
            translation: (self.bounding_box.larger().to_vec3()
                + self.bounding_box.smaller().to_vec3())
                / 2.0,
            ..Default::default()
        };

        (self.block_type, transform, mesh)
    }
}

impl<'a> core::fmt::Debug for Cluster<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cluster")
            .field("block_type", &self.block_type)
            .field("bounding_box", &self.bounding_box)
            .field("chunk", &self.chunk.coords())
            .finish()
    }
    //
}

// in another module (file)!
// this can use the previous neighbor-chasing algo from the first attempt
impl Chunk {
    // ...

    /// Given a block, this method chases all of its neighbors until there are
    /// none of the same type.
    pub(crate) fn chase_neighbors(
        &self,
        starting_block: &Block,
        starting_coordinate: &ChunkBlockCoordinate,
        completed_blocks: &mut HashSet<ChunkBlockCoordinate>,
    ) -> Option<Cluster> {
        tracing::debug!("Chasing neighbors: {starting_block:?} at {starting_coordinate}...");

        let mut cluster = Cluster::new(
            starting_block.block_type,
            BoundingBox::new_point(*starting_coordinate), // a bounding box that has no area
            self,
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
                self.next_block(&starting_coordinate, direction)
            {
                if starting_block.same_kind_as(&neighbor_block)
                    && completed_blocks.get(&neighbor_coordinate).is_none()
                {
                    tracing::debug!(
                        "Found neighbor: {neighbor_block:?} at {neighbor_coordinate}..."
                    );

                    cluster.extend(neighbor_coordinate);
                    completed_blocks.insert(neighbor_coordinate);
                    completed_blocks.insert(starting_coordinate);

                    starting_block = neighbor_block.clone();
                    starting_coordinate = neighbor_coordinate;
                } else {
                    break;
                }
            }
        }

        // TODO: test this method and make sure it returns None when it should
        for coord in cluster.bounding_box.all_coordinates() {
            completed_blocks.insert(coord);
        }

        Some(cluster)
    }

    /// Creates a list of the chunk's block `Cluster`s to be rendered.
    pub fn cluster(&self) -> Vec<Cluster> {
        let mut blocks_to_check = self.blocks();
        let mut completed_blocks = HashSet::new();
        let mut clusters = Vec::new();

        for (coordinate, block) in blocks_to_check
            .iter_mut() // don't check air
            .filter(|(_, block)| block.block_type != BlockType::Air)
        {
            // don't check blocks we've finished
            if completed_blocks.contains(coordinate) {
                continue;
            }

            if let Some(cluster) = self.chase_neighbors(block, coordinate, &mut completed_blocks) {
                clusters.push(cluster);
            }
        }

        tracing::debug!("Found all these clusters: {clusters:#?} \n \n");

        // let new_clusters: Vec<Cluster>;

        // for c in clusters {
        //     clusters.windows(2).filter(|a| a[0] > a[1]);
        // }

        clusters
    }

    // ...
}

/// A function called by Bevy's renderer when it's collected a `World`'s `Cluster`s.
///
/// This will eventually take a cluster to render (i.e. chunk coordinate) so
/// we don't update the whole world.
pub fn render_clusters(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    world: crate::world::World,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let clusters = world
        .chunks()
        .iter()
        .flat_map(|(_, f)| f.cluster())
        .map(|c| c.build())
        .collect::<Vec<_>>();

    for (block_type, transform, mesh) in clusters {
        tracing::debug!(
            "Rendering cluster of block type: `{block_type:?}` at `{}`",
            transform.translation
        );

        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            transform,
            // material: `get_texture_from_block_type(block_type)`,
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                base_color_texture: Some(
                    asset_server.load("/home/barrett/Documents/macaw/assets/stone.png"),
                ),
                reflectance: 1.0,
                metallic: 0.1,
                ..Default::default()
            }),
            ..Default::default()
        });
    }
    // render things
}

/// Attempts to spawn a cluster. If it does, it'll sleep for one second.
pub fn debug_cluster_rendering<'a>(
    starting: (Block, ChunkBlockCoordinate),
    completed_blocks: &mut HashSet<ChunkBlockCoordinate>,
    chunk: &'a Chunk,
) -> Option<Cluster<'a>> {
    if let Some(cluster) = chunk.chase_neighbors(&starting.0, &starting.1, completed_blocks) {
        sleep(Duration::from_secs(1));
        return Some(cluster);
    }
    None
}
