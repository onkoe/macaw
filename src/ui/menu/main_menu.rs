//! # Main Menu
//!
//! The title screen menu in Macaw.

use bevy::prelude::*;

use super::MenuState;

#[derive(Event)]
struct GameMenuCloseEvent;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameMenuCloseEvent>();

        app.add_systems(Startup, MainMenuPlugin::render_menu);
        app.add_systems(
            Update,
            MainMenuPlugin::perform_menu_actions.run_if(in_state(MenuState::MainMenu)),
        );
        app.add_systems(OnExit(MenuState::MainMenu), MainMenuPlugin::close);
        app.init_state::<MenuState>();
    }
}

#[derive(Component)]
struct MainMenuRoot;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitButton;

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
            // button holding 'div'
            .with_children(|p| {
                p.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|p| {
                    // play button
                    spawn_button(p, "Play", PlayButton);

                    // quit button
                    spawn_button(p, "Quit", QuitButton);
                });
            });
    }

    /// Drives the reactivity of components on the main menu.
    #[allow(clippy::type_complexity)]
    fn perform_menu_actions(
        mut query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                Option<&PlayButton>,
                Option<&QuitButton>,
            ),
            Changed<Interaction>,
        >,
        mut state: ResMut<NextState<MenuState>>,
    ) {
        for (interaction, mut color, mut border_color, play_button, quit_button) in &mut query {
            match interaction {
                Interaction::Pressed => {
                    // TODO: this should be on mouse release. NOT on press
                    if play_button.is_some() {
                        *color = Color::RED.into();
                        border_color.0 = Color::RED;
                        state.set(MenuState::Game);
                    }
                    if quit_button.is_some() {
                        tracing::warn!("oh you wanna leave do you? here you go!");
                        panic!("critical scary warning panic error corruption scary bad");
                    }
                }
                Interaction::Hovered => {
                    *color = Color::ALICE_BLUE.into();
                    border_color.0 = Color::BLUE;
                }
                Interaction::None => {
                    *color = Color::BLACK.into();
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }

    /// Closes the main menu.
    fn close(mut q: Query<&mut Visibility, With<MainMenuRoot>>) {
        let mut visibility = q.single_mut();
        *visibility = Visibility::Hidden;
    }
}

/// Spawns a button on the given node.
fn spawn_button(p: &mut ChildBuilder, text: impl AsRef<str>, ident: impl Component) {
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
            padding: UiRect::axes(Val::Percent(8.0), Val::Percent(1.0)),
            ..Default::default()
        },
        background_color: Color::BLACK.into(),
        border_color: Color::RED.into(),
        ..Default::default()
    })
    .insert(ident)
    .with_children(|p| {
        p.spawn(TextBundle::from_section(
            text.as_ref(),
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..Default::default()
            },
        ));
    });
}
