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

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, pbr::DirectionalLightShadowMap, prelude::*};

use macaw::{renderer::MacawRendererPlugin, ui::MacawUiPlugin};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()?
        .add_directive("mycrate=debug".parse()?);

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .finish()
        .init();

    let mut app = App::new();

    app.add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
        //.add_plugins(NoCameraPlayerPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins((MacawUiPlugin, MacawRendererPlugin))
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_systems(
            Startup,
            (macaw::player::setup, macaw::renderer::skybox::setup),
        )
        .add_systems(Update, macaw::player::player_input_system);

    app.run();

    Ok(())
}
