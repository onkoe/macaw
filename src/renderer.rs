use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub mod meshing_two;
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
        asset_server: Res<AssetServer>,
    ) {
        // enable mouse lock
        let mut window = window_query.single_mut();
        window.cursor.grab_mode = CursorGrabMode::Locked;

        let world = crate::world::World::generate_test_chunk();

        let mesh = Cuboid::new(1.0, 1.0, 1.0).mesh();

        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            transform: Transform {
                translation: Vec3::new(100.0, 0.0, 0.0),
                ..Default::default()
            },
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

        meshing_two::render_clusters(&mut commands, meshes, world, asset_server, materials);
    }
}
