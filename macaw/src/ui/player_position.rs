use bevy::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct PositionRoot;

#[derive(Component)]
pub struct PositionText;

pub fn setup(mut commands: Commands) {
    let root = commands
        .spawn((
            PositionRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.),
                    top: Val::Percent(10.),
                    bottom: Val::Auto,
                    left: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let position_text = commands
        .spawn((
            PositionText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "Position: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: " (?, ?, ?)".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[position_text]);
}

pub fn position_update_system(
    mut query: Query<&mut Text, With<PositionText>>,
    player_position: Query<&GlobalTransform, With<Player>>,
) {
    for t in player_position.iter() {
        let pos = t.translation();

        for mut text in query.iter_mut() {
            text.sections[1].value = format!("({:>6.2}, {:>6.2}, {:>6.2})", pos.x, pos.y, pos.z);
        }
    }
}
