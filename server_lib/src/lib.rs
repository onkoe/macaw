//! # server_lib
//!
//! A library for Macaw's servers. Powers both singleplayer and multiplayer
//! sessions for the game.
//!
//! ## Features (thank u github copilot)
//! - **Singleplayer** - Play by yourself.
//! - **Multiplayer** - Play with friends.
//! - **World Generation** - Generate worlds.
//! - **World Saving** - Save worlds.
//! - **World Loading** - Load worlds.
//! - **World Editing** - Edit worlds.
//! - **World Networking** - Network worlds.
//! - **World Physics** - Simulate worlds.
//! - **World Rendering** - Render worlds.
//! - **World Audio** - Play sounds in worlds.
//! - **World Input** - Interact with worlds.
//! - **World Entities** - Entities in worlds.
//! - **World Blocks** - Blocks in worlds.
//! - **World Items** - Items in worlds.
//! - **World Crafting** - Craft items in worlds.
//! - **World Inventory** - Manage items in worlds.
//! - **World Entities** - Entities in worlds.
//! - **Commands** - Execute commands.

#![deny(missing_docs)]

use shared::world::MacawWorld;

struct Server {
    world: MacawWorld,
}
