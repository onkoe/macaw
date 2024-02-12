use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::world::{self, coordinates::ChunkBlockCoordinate, meshing::Meshing};

pub mod skybox;

pub struct MacawRendererPlugin;

impl Plugin for MacawRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}

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
    let world = world::generate();

    let stone_handle: Handle<Image> =
        assets.load("/home/barrett/Documents/mythic_mining/assets/stone.png");

    let stone_material = materials.add(StandardMaterial {
        base_color: Color::RED,
        base_color_texture: Some(stone_handle),

        ..Default::default()
    });

    for (chunk_location, chunk) in world.chunks() {
        tracing::debug!("chunk: `{chunk_location:?}`");

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block_coords = ChunkBlockCoordinate::new(x, y, z);
                    let block = chunk.get_local_block(&block_coords);

                    if let Some(ref b) = block {
                        tracing::debug!("block is found! {:?}", &block);

                        let block_coordinates = chunk.global_block_coord(block_coords.clone());
                        if chunk.is_visible(&block_coords) {
                            tracing::debug!(
                                "placing block {b:?} at coords: `{block_coordinates:?}`",
                            );

                            commands.spawn(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 1_f32 })),
                                transform: Transform {
                                    translation: block_coordinates.to_vec3(),
                                    ..Default::default()
                                },
                                material: stone_material.clone(),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
    }

    tracing::info!("done 'rendering' world");
}
