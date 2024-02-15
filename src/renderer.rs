use bevy::{
    prelude::*,
    render::mesh::Indices,
    window::{CursorGrabMode, PrimaryWindow},
};

use self::meshing::triangle::{Face, Triangle};

pub mod meshing;
pub mod skybox;

pub struct MacawRendererPlugin;

impl Plugin for MacawRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, Self::setup);
    }
}

impl MacawRendererPlugin {
    #[allow(unused)]
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

        let world = crate::world::World::generate();

        fn handle_builder(assets: &Res<AssetServer>, path: &str) -> Handle<Image> {
            assets.load(path.to_owned())
        }

        fn material_builder(
            materials: &mut ResMut<'_, Assets<StandardMaterial>>,
            image_handle: Handle<Image>,
        ) -> Handle<StandardMaterial> {
            materials.add(StandardMaterial {
                base_color: Color::RED,
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

        let _stone_material = material_builder(&mut materials, stone_handle);
        let _dirt_material = material_builder(&mut materials, dirt_handle);
        let _grass_material = material_builder(&mut materials, grass_handle);

        for (chunk_location, chunk) in world.chunks().clone() {
            tracing::debug!("chunk: `{chunk_location:?}`");

            let calculated_meshes = meshing::greedy::meshing(&chunk);

            for (transform, mesh) in calculated_meshes {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(mesh),
                    transform,
                    //material: stone_material.clone(),
                    /*match b.block_type {
                        block::BlockType::Grass => grass_material.clone(),
                        block::BlockType::Dirt => dirt_material.clone(),
                        _ => stone_material.clone(),
                    },*/
                    ..Default::default()
                });
            }
        }

        tracing::info!("done 'rendering' world");
    }
}
