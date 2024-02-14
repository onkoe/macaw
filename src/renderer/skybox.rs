use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

use crate::player::PlayerCamera;

pub fn add_skybox(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Has<PlayerCamera>>,
    mut images: ResMut<Assets<Image>>,
) {
    let skybox_handle = asset_server.load::<Image>("skybox/skybox.png");

    // the assets MUST be loaded by now, but we must check
    if asset_server.get_load_state(skybox_handle.clone().id()) == Some(LoadState::Loaded) {
        if let Some(image) = images.get_mut(skybox_handle.clone()) {
            for x in query.iter() {
                //
                // fix png skybox (pngs don't have valid metadata on cubemapping)
                if image.texture_descriptor.array_layer_count() == 1 {
                    image.reinterpret_stacked_2d_as_array(image.height() / image.width());
                    image.texture_view_descriptor = Some(TextureViewDescriptor {
                        dimension: Some(TextureViewDimension::Cube),
                        ..default()
                    });
                }

                commands.entity(x).insert(Skybox(skybox_handle.clone()));
            }
        }
    }
}
