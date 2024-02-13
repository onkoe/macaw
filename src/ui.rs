use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_editor_pls::EditorPlugin;

use crate::util::{built_info::PKG_VERSION, get_pkg_name};

mod fps_counter;
mod player_position;

pub struct MacawUiPlugin;

impl Plugin for MacawUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup,
                player_position::setup,
                fps_counter::setup_fps_counter,
            ),
        );

        app.add_systems(
            Update,
            (
                fps_counter::fps_text_update_system,
                fps_counter::fps_counter_showhide,
                player_position::position_update_system,
            ),
        );

        if cfg!(debug_assertions) {
            app.add_plugins(EditorPlugin::default());
        }
    }
}

/// Builds the major UI elements.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        camera: Camera {
            order: 1,
            ..Default::default()
        },
        ..Default::default()
    });

    // version info (top left)

    // capitalize game name no matter what :3

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Auto,
                    left: Val::ZERO,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(TextBundle {
                    text: Text::from_sections([TextSection {
                        value: format!("{} Beta {}", get_pkg_name(), PKG_VERSION),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    }]),
                    ..Default::default()
                });
            });
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        // crosshair (vertical)
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    width: Val::Px(2_f32),
                    height: Val::Px(24_f32),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::GRAY),
                ..default()
            });

            // crosshair (horizontal)
            p.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    width: Val::Px(24_f32),
                    height: Val::Px(2_f32),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::GRAY),
                ..default()
            });
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(400_f32),
                    height: Val::Px(48_f32),
                    border: UiRect::all(Val::Px(2_f32)),
                    align_self: AlignSelf::End,
                    ..default()
                },
                background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                ..default()
            });
        });
}
