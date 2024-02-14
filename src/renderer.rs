use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{
    block::{self, BlockSide},
    world::{
        self,
        coordinates::{ChunkBlockCoordinate, GlobalCoordinate},
    },
};

pub mod skybox;

pub struct MacawRendererPlugin;

impl Plugin for MacawRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, Self::setup);
    }
}

impl MacawRendererPlugin {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut window_query: Query<&mut Window, With<PrimaryWindow>>,
        assets: Res<AssetServer>,
    ) {
        // enable mouse lock
        let mut window = window_query.single_mut();
        window.cursor.grab_mode = CursorGrabMode::Locked;
        let world = world::World::generate_test_chunk();

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

        let stone_handle: Handle<Image> =
            handle_builder(&assets, "/home/barrett/Documents/macaw/assets/stone.png");
        let grass_handle: Handle<Image> =
            handle_builder(&assets, "/home/barrett/Documents/macaw/assets/grass.png");
        let dirt_handle: Handle<Image> =
            handle_builder(&assets, "/home/barrett/Documents/macaw/assets/dirt.png");

        let stone_material = material_builder(&mut materials, stone_handle);
        let dirt_material = material_builder(&mut materials, dirt_handle);
        let grass_material = material_builder(&mut materials, grass_handle);

        for (chunk_location, chunk) in world.chunks().clone() {
            tracing::debug!("chunk: `{chunk_location:?}`");

            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let block =
                            chunk.block_from_local_coords(&ChunkBlockCoordinate::new(x, y, z));

                        if let Some(ref b) = block {
                            let block_coordinates =
                                chunk.global_block_coord(ChunkBlockCoordinate::new(x, y, z));

                            // for face on block, render that mf
                            /*
                            for face in world.block_exposed_sides(block_coordinates) {
                                render_block_side(
                                    &mut commands,
                                    &mut meshes,
                                    face,
                                    block_coordinates,
                                    match b.block_type {
                                        block::BlockType::Grass => grass_material.clone(),
                                        block::BlockType::Dirt => dirt_material.clone(),
                                        _ => stone_material.clone(),
                                    },
                                )
                            }*/

                            commands.spawn(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 1_f32 })),
                                transform: Transform {
                                    translation: block_coordinates.to_vec3(),
                                    ..Default::default()
                                },
                                material: match b.block_type {
                                    block::BlockType::Grass => grass_material.clone(),
                                    block::BlockType::Dirt => dirt_material.clone(),
                                    _ => stone_material.clone(),
                                },
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }

        tracing::info!("done 'rendering' world");
    }

    /// Given a BlockSide, renders a plane in its place.
    fn render_block_side(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        block_side: BlockSide,
        coords: GlobalCoordinate,
        texture: Handle<StandardMaterial>,
    ) {
        tracing::debug!("Rendering block side: {block_side:?} at {coords}");

        struct PlaneTransform {
            rotation: Quat,
            offset: Vec3,
        }

        // how much to shift + rotate the plane by
        // FIXME: i messed with this a lot and the planes are wonky as hell
        // probably gonna end up spinning my own meshes >:3
        let adjustment = match block_side {
            BlockSide::PositiveX => PlaneTransform {
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                offset: Vec3::new(0.5, 0.0, 0.0),
            },
            BlockSide::NegativeX => PlaneTransform {
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_1_SQRT_2),
                offset: Vec3::new(-0.5, 0.0, 0.0),
            },
            BlockSide::PositiveY => PlaneTransform {
                rotation: Quat::IDENTITY,
                offset: Vec3::new(0.0, 0.5, 0.0),
            },
            BlockSide::NegativeY => PlaneTransform {
                rotation: Quat::from_rotation_y(std::f32::consts::E),
                offset: Vec3::new(0.0, -0.5, 0.0),
            },
            BlockSide::PositiveZ => PlaneTransform {
                rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                offset: Vec3::new(0.0, 0.0, 0.5),
            },
            BlockSide::NegativeZ => PlaneTransform {
                rotation: Quat::from_rotation_x(-(std::f32::consts::FRAC_PI_2)),
                offset: Vec3::new(0.0, 0.0, -0.5),
            },
        };

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 1_f32,
                ..Default::default()
            })),
            transform: Transform {
                rotation: adjustment.rotation,
                translation: coords.to_vec3() + adjustment.offset,
                ..Default::default()
            },
            material: texture,
            ..Default::default()
        });
    }
}
