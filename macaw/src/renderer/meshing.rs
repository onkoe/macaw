//! # Meshing
//!
//! Performs greedy meshing for clusters of blocks found in a chunk.

use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    block::{Block, BlockSide, BlockType},
    util::get_file,
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
            translation: ((self.bounding_box.larger().to_vec3()
                + self.bounding_box.smaller().to_vec3())
                / 2.0),
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
        let mut selected_block = starting_block.clone();
        let mut selected_coordinate = *starting_coordinate;

        // use positive only; we're starting from the lowest coordinate
        let directions = [
            BlockSide::PositiveX,
            BlockSide::PositiveY,
            BlockSide::PositiveZ,
        ];

        // check in all directions
        for direction in directions {
            if direction == BlockSide::PositiveZ {
                completed_blocks.insert(selected_coordinate);
            }

            while let Some((neighbor_block, neighbor_coordinate)) =
                self.next_block(&selected_coordinate, direction)
            {
                if selected_block.same_kind_as(&neighbor_block)
                    && completed_blocks.get(&neighbor_coordinate).is_none()
                {
                    tracing::debug!(
                        "Found neighbor: {neighbor_block:?} at {neighbor_coordinate}..."
                    );

                    // TODO: optimize by only checking the 'new' blocks
                    // created by the 'potential' extension
                    //
                    // we can also avoid cloning by having a method that
                    // does logic instead
                    let is_valid_extension = {
                        let mut c = cluster.clone();
                        c.extend(neighbor_coordinate);
                        c
                    }
                    .bounding_box
                    .all_coordinates()
                    .iter()
                    .map(|c| self.block(c))
                    .all(|b| match b {
                        Some(block) => block.block_type == cluster.block_type,
                        None => false,
                    });

                    if is_valid_extension {
                        cluster.extend(neighbor_coordinate);
                        completed_blocks.insert(neighbor_coordinate);
                        completed_blocks.insert(selected_coordinate);

                        selected_block = neighbor_block.clone();
                        selected_coordinate = neighbor_coordinate;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        // TODO: test this method and make sure it returns None when it should
        for coord in cluster.bounding_box.all_coordinates() {
            completed_blocks.insert(coord);
        }
        completed_blocks.insert(selected_coordinate);

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

        for c in &clusters {
            for cc in &clusters {
                if c != cc && !c.bounding_box.is_point() && cc.bounding_box.is_point() {
                    let cc_coords = cc.bounding_box.all_coordinates();

                    for c_coord in c.bounding_box.all_coordinates() {
                        if cc_coords.contains(&c_coord) {
                            tracing::error!("duplicate coordinates in clusters!\n\n Cluster 1: {c:#?}\n\n Cluster 2: {cc:#?}\n\n coordinate: {c_coord}");
                        }
                    }
                }
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
pub fn render_clusters(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    world: crate::world::MacawWorld,
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
            material: get_texture_from_block_type(&block_type, &mut materials, &asset_server),
            ..Default::default()
        });
    }
    // render things
}

fn get_texture_from_block_type(
    block_type: &BlockType,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
) -> Handle<StandardMaterial> {
    fn handle_builder(assets: &Res<AssetServer>, path: &str) -> Handle<Image> {
        assets.load(path.to_owned())
    }

    fn material_builder(
        materials: &mut ResMut<'_, Assets<StandardMaterial>>,
        image_handle: Handle<Image>,
    ) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color_texture: Some(image_handle),
            reflectance: 1.0,
            metallic: 0.0,
            ..Default::default()
        })
    }

    let stone_handle: Handle<Image> = handle_builder(asset_server, &get_file("stone.png"));
    let grass_handle: Handle<Image> = handle_builder(asset_server, &get_file("grass.png"));
    let dirt_handle: Handle<Image> = handle_builder(asset_server, &get_file("dirt.png"));

    let stone_material = material_builder(materials, stone_handle);
    let dirt_material = material_builder(materials, dirt_handle);
    let grass_material = material_builder(materials, grass_handle);

    match block_type {
        BlockType::Grass => grass_material.clone(),
        BlockType::Dirt => dirt_material.clone(),
        _ => stone_material.clone(),
    }
}
