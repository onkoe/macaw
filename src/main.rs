//! # Macaw
//!
//! A voxel game aiming to target functionality matching that of Minecraft
//! b1.7.3, alongside some quality of life features, fixes, and performance
//! increases.
//!
//! Who knows if it'll ever do that. But it'd be fun!
//!
//! ## Planned Features
//!
//! This isn't final, but here are some potential features and discussion
//! around them:
//!
//! 1. FoV: yup!
//! 1. Textures/Sounds: Re-use CC0'd Minecraft texture packs of the era.
//!    I'd like to allow texture pack usage from beta 1.7.3, but I'd need to
//!    be careful when whipping up a parser.
//! 1. Creative/Infinite Building Mode: yup!
//! 1. Mods: I'd like to support mods using something like `bevy_wasm`.
//!    For a closer form of integration, something like [rhai](https://crates.io/crates/rhai)
//!    might be nice!
//!
//!
//! ## Unplanned Features
//!
//! While there's a lot to do, it's also important to clarify what Macaw isn't
//! intending to be!
//!
//! It isn't:
//!
//! 1. a modern Minecraft replacement. Macaw won't aim to become an advanced
//!    voxel game. Instead, it's a passion project intended for pre-b1.8 users
//!    who prefer the game in its raw form. There won't be goals, extensive
//!    logic circuits, or vibrant, dense worlds.
//! 1. for-profit. I don't care about making money off this game - it's just
//!    a way to access a peaceful, serene form of gameplay. It's one that
//!    doesn't require sweeping game modifications or old software.
//! 1. adventurous. Most popular voxel games have the player embark on some
//!    quest across the world. This won't have RPG or streamlined adventure
//!    elements.
//! 1. for a community. The only audience I'm targeting here is me. If you want
//!    to change something that isn't in the purview of b1.7.3 feature parity,
//!    I suggest using mods or forking this project.
//! 1. optimized to the bone. I'll happily use a lot of libraries, high-level
//!    Rust features, and safe wrappers! My only target is to match my
//!    monitor's specs (2160p144).
//! 1. with warranty.

use bevy::{
    pbr::DirectionalLightShadowMap,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use bevy_flycam::NoCameraPlayerPlugin;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};
use world::ChunkBlockCoordinate;

use world::meshing::Meshing;

use crate::ui::fps_counter::{fps_counter_showhide, fps_text_update_system, setup_fps_counter};

mod block;
mod player;
mod ui;
mod world;

fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()?
        .add_directive("mycrate=debug".parse()?);

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .finish()
        .init();

    tracing::warn!("hello world!");

    App::new()
        //.add_plugins((DefaultPlugins, player::controls::ControlsPlugin))
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_systems(
            Startup,
            (setup, player::setup, ui::setup, setup_fps_counter),
        )
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        //.add_systems(Update, player::player_input_system)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // enable mouse lock
    let mut window = window_query.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Locked;

    // create a light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(5.0, 8.0, 5.0),
        ..Default::default()
    });

    // make the block
    let cube_mesh = Mesh::from(shape::Cube { size: 2.0 }); // size of the cube
    let green_material = materials.add(Color::rgb(0.0, 1.0, 0.0).into());

    // spawn that mf
    commands.spawn(PbrBundle {
        mesh: meshes.add(cube_mesh),
        material: green_material.clone(),
        ..Default::default()
    });

    let world = world::generate();

    for (chunk_location, chunk) in world.chunks() {
        tracing::debug!("chunk: `{chunk_location:?}`");

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block_coords = ChunkBlockCoordinate::new(x, y, z);
                    let block = chunk.get_local_block(&block_coords);

                    if let Some(ref b) = block {
                        tracing::debug!("block is found! {:?}", &block);

                        if let Some(b_coords) = chunk.global_block_coords(block_coords.clone()) {
                            if chunk.is_visible(&block_coords) {
                                tracing::debug!(
                                    "placing block {0:?} at coords: `{1:?}`",
                                    b,
                                    b_coords
                                );

                                commands.spawn(PbrBundle {
                                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1_f32 })),
                                    transform: Transform {
                                        translation: b_coords.to_vec3(),
                                        ..Default::default()
                                    },
                                    material: green_material.clone(),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    tracing::info!("done 'rendering' world");
}
