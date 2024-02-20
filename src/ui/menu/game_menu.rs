//! Game Menu
//!
//! The pause screen in a Macaw world.

use bevy::prelude::*;

pub struct GameMenuPlugin;

impl GameMenuPlugin {
    fn toggle(&mut self) {}
}

impl Plugin for GameMenuPlugin {
    fn build(&self, _app: &mut App) {
        //app.add_event::<GameMenuToggleEvent>();
    }
}
