//! # Main Menu
//!
//! The main menu in Macaw.

use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, States)]
pub enum MenuState {
    #[default]
    Menu,
    Options,
    Game,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, MainMenuPlugin::render_menu);
        app.add_systems(
            Update,
            MainMenuPlugin::perform_menu_actions.run_if(in_state(MenuState::Menu)),
        );
        app.init_state::<MenuState>();
    }
}

#[derive(Component)]
struct MainMenuRoot;

#[derive(Component)]
struct GenerateButton;

impl MainMenuPlugin {
    /// Creates the main menu.
    fn render_menu(mut commands: Commands) {
        commands
            .spawn((
                MainMenuRoot,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    background_color: Color::MIDNIGHT_BLUE.into(),
                    z_index: ZIndex::Global(0),
                    ..Default::default()
                },
            ))
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(0.5),
                        top: Val::Percent(0.5),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                crate::util::full_title(),
                                TextStyle {
                                    font_size: 14.0,
                                    ..Default::default()
                                },
                            )],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            })
            .with_children(|p| {
                p.spawn(ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        min_width: Val::Percent(5.0),
                        min_height: Val::Percent(2.0),
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Percent(1.0)),
                        padding: UiRect::axes(Val::Percent(10.0), Val::Percent(2.0)),
                        ..Default::default()
                    },
                    background_color: Color::BLACK.into(),
                    border_color: Color::RED.into(),
                    ..Default::default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle {
                        style: Style {
                            // height: Val::Percent(100.0),
                            // width: Val::Percent(100.0),
                            ..Default::default()
                        },
                        text: Text {
                            sections: vec!["Generate World".into()],
                            justify: JustifyText::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            });
    }

    /// Drives the reactivity of components on the main menu.
    #[allow(clippy::type_complexity)]
    fn perform_menu_actions(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, &mut BorderColor),
            (Changed<Interaction>, With<Button>),
        >,
        mut state: ResMut<NextState<MenuState>>,
    ) {
        for (interaction, mut color, mut border_color) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => {
                    *color = Color::RED.into();
                    border_color.0 = Color::RED;
                    state.set(MenuState::Game);
                }
                Interaction::Hovered => {
                    *color = Color::ALICE_BLUE.into();
                    border_color.0 = Color::WHITE;
                }
                _ => {}
            }
        }
    }
}
