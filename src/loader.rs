use bevy::{asset::LoadState, prelude::*};

/// A representation of whether or not the project has completed the loading
/// process for filesystem resources.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, States)]
pub enum LoaderState {
    /// The project hasn't finished loading all resources.
    #[default]
    Loading,
    /// All resources are loaded!
    Complete,
}

/// A list of assets that the loader will handle.
///
/// This list just stores the handles of these assets, checking to see
/// when they're all loaded.
#[derive(Resource)]
pub struct LoaderAssets {
    handles: Vec<UntypedHandle>,
}

impl LoaderAssets {
    /// From a given vector of handles, returns a new `LoaderAssets`.
    fn new(handles: Vec<UntypedHandle>) -> Self {
        Self { handles }
    }
}

/// A plugin that loads the project's filesystem resources.
pub struct MacawLoaderPlugin;

impl MacawLoaderPlugin {
    /// Begins loading resources.
    fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
        let loader_assets = LoaderAssets::new(vec![asset_server
            .load::<Image>("skybox/skybox.png")
            .untyped()]);

        // add our asset list to global resources
        commands.insert_resource(loader_assets);
    }

    /// Checks if the loader state should become `LoaderState::Complete`.
    /// If so, it'll set the project state to Complete.
    fn check_progress(
        mut state: ResMut<NextState<LoaderState>>,
        loader_assets: Res<LoaderAssets>,
        asset_server: Res<AssetServer>,
    ) {
        if loader_assets
            .handles
            .iter()
            .all(|h| asset_server.get_load_state(h.id()) == Some(LoadState::Loaded))
        {
            tracing::info!("All assets have completed loading!");
            state.0 = Some(LoaderState::Complete);
        }
    }
}

impl Plugin for MacawLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LoaderState>();
        app.add_systems(Startup, MacawLoaderPlugin::load);
        app.add_systems(
            Update,
            MacawLoaderPlugin::check_progress.run_if(state_exists_and_equals(LoaderState::Loading)),
        );
    }
}
