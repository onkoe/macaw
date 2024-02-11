use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub mod fps_counter;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    width: Val::Px(16_f32),
                    height: Val::Px(16_f32),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::RED),
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
