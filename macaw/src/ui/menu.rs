//! # Menu
//!
//! A Macaw module that handles the game's menus, including the options screen,
//! title screen, and pause menu.

use bevy::prelude::*;

pub mod game_menu;
pub mod main_menu;
pub mod options;

/// The `States` of which menu should currently be displayed,
/// if there is one (psst. Game variant).
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, States)]
pub enum MenuState {
    /// The title screen.
    #[default]
    MainMenu,
    /// No menu - just in-game.
    Game,
    /// In-game pause menu.
    GameMenu,
    /// In-game chat.
    Chat,
    /// Options, whether through the pause or main menu.
    Options,
}

pub struct MacawMenuPlugin;

impl Plugin for MacawMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((game_menu::GameMenuPlugin, main_menu::MainMenuPlugin));
        app.init_state::<MenuState>();
    }
}
