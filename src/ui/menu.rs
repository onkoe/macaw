//! # Menu
//!
//! A Macaw module that handles the game's menus, including the options screen,
//! title screen, and pause menu.

use bevy::prelude::*;

pub mod game_menu;
pub mod main_menu;
pub mod options;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, States)]
pub enum MenuState {
    #[default]
    MainMenu,
    Game,
    GameMenu,
    Options,
}

pub struct MacawMenuPlugin;

impl Plugin for MacawMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((game_menu::GameMenuPlugin, main_menu::MainMenuPlugin));
        app.init_state::<MenuState>();
    }
}
