//! # Meshing
//!
//! Some utilities to help with creating meshes in Macaw.

use crate::block::{Block, BlockType};
use crate::world::chunk::Chunk;
use crate::world::coordinates::ChunkBlockCoordinate;
use bevy::math::{Vec2, Vec3};
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;

use self::triangle::Face;

pub mod greedy;
pub mod triangle;

/// An in-progress mesh.
#[derive(Clone, Debug, Default, PartialEq)]
struct MeshConstruct {
    /// Location of mesh placement in the chunk.
    /// This is the closest point to (0, 0, 0) of all the meshes.
    transform: ChunkBlockCoordinate,
    /// Corners of the mesh. These indicate mesh positions.
    /// Use the existing block corners for visible parts of the mesh, removing
    /// those that are between two
    positions: Vec<Vec3>,
    /// Vectors pointing away from the mesh (for lighting calculations).
    normals: Vec<Vec3>,
    /// Texture mapping coordinates. UVs help to loop textures.
    /// Voxels need four of these per face.
    uvs: Vec<Vec2>,
    /// Shared vertices (corners) of a mesh. These are encapsulated by faces and their triangles.
    /// A cube has eight of these.
    indices: Vec<Face>,
}

impl MeshConstruct {
    /// Given `self` and some `other` `MeshConstruct`, this method will
    /// combine the two.
    fn combine(&mut self, mut other: MeshConstruct, transform: Vec3, offset: u32) {
        // positions
        let transformed_positions = other
            .positions
            .iter()
            .map(|pos| *pos + transform)
            .collect::<Vec<_>>();
        self.positions.extend(transformed_positions);

        // normals
        self.normals.extend(other.normals);

        // ok here are the UVs
        // TODO: find rectangles and make them into two triangles. this means we gotta clean up 'extra' uvs
        self.uvs.extend(other.uvs);

        // Calculate the offset for the new vertices
        let vertex_offset = self.positions.len() as u32;

        // Update indices by adding the vertex offset
        for face in &mut other.indices {
            face.offset(vertex_offset);
        }

        // Continue handling indices
        self.indices.extend(other.indices);
    }

    /// Given two MeshConstructs, find the Vec3 that best represents their combined coordinates.
    fn calculate_transform(&self, other: MeshConstruct) -> Vec3 {
        //let

        todo!()
    }

    /// Takes the various components of a block, along with an index offset, and makes a MeshConstruct.
    fn from_block(block: &Block, position: ChunkBlockCoordinate, chunk: &Chunk) -> Option<Self> {
        // TODO: for stairs and other blocks, the offset may not be * 8 (the number of vertices)
        let offset = (chunk.block_index(&position) as u32) * 8;

        if block.block_type == BlockType::Air {
            return None;
        }

        tracing::info!(
            "Making MeshConstruct from block: indices: {}, uvs: {} \n",
            &block
                .indices(offset)
                .iter()
                .map(|ind| ind.count())
                .sum::<usize>(),
            &block.uvs().len()
        );

        Some(Self {
            transform: position,
            positions: block.positions(),
            normals: block.normals(),
            uvs: block.uvs(),
            indices: block.indices(offset),
        })
    }

    /// Creates a `Mesh` from `self`.
    fn build(self) -> Mesh {
        tracing::info!(
            "Building Mesh from MeshConstruct with following components: indices: {}, uvs: {} \n",
            &self.indices.iter().map(|ind| ind.count()).sum::<usize>(),
            &self.uvs.len()
        );

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh.set_indices(Some(Indices::U32(
            self.indices
                .iter()
                .flat_map(|face| face.to_u32_list())
                .collect::<Vec<u32>>(),
        )));
        mesh
    }
}
