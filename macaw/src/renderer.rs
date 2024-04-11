use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use shared::world::{generation::generators::fixed::Generate, MacawWorld};

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

        // create a new world and save it
        let mut world = Generate::testing_world();
        world.save().expect("world should save");

        // load back this world
        let loaded_world = MacawWorld::load(world.metadata()).expect("would should load");

        // render the loaded world!
        meshing::render_clusters(
            &mut commands,
            meshes,
            &loaded_world,
            asset_server,
            materials,
        );

        world
            .save()
            .expect("world should be able to save onto its old self");
    }
}
