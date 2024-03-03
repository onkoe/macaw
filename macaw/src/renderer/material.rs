//! # Material
//!
//! A module that aids in creating materials for each block.

use bevy::{prelude::*, render::render_resource::AsBindGroup};

/// A material extension for greedy meshes that respects their block counts.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GreedyMaterialExtension {
    #[uniform(0)]
    pub block_count: Vec2,
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,
    // alpha_mode: TODO?,
}

impl Material for GreedyMaterialExtension {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/blocks/greedy.wgsl".into()
    }

    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/blocks/greedy.wgsl".into()
    }
}
