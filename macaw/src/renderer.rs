use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub mod meshing;
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
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
        mut window_query: Query<&mut Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
    ) {
        // enable mouse lock
        let mut window = window_query.single_mut();
        window.cursor.grab_mode = CursorGrabMode::Locked;

        let world = shared::world::generation::generators::fixed::Generate::testing_world();

        meshing::render_clusters(&mut commands, meshes, world, asset_server, materials);
    }
}
