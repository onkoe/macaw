# Macaw

Just another voxel game. Maybe.

## Development

If you want to work on this game, please verify you've [installed the dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) for Bevy and [have nightly Rust installed](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html). It's necessary for Bevy internally.

### Layout

The workspace contains various crates with differing uses. Let's take a brief look:

- `macaw`: The main crate for the game. Mostly a pile of rendering, UI, and interactivity code.
- `shared`: Some collective resources used both by the client (`macaw` crate) and server. This includes behaviors and game concepts (like blocks, coordinates, and chunks).
- `server_lib`: The library allowing for servers to run. It's a library to allow singleplayer sessions to run their own server locally.
- `server`: A multiplayer session client. This won't be worked on until later. (TODO)

Additional crates may appear in the workspace in the future, possibly collecting mechanics into separated components.

### Contributions

If you want to contribute, that sounds good to me! Just keep in mind one simple rule: this is a personal passion project of mine, and I won't add features that are out of scope... i.e. dissimilar to Minecraft Beta 1.7.3.

Send an issue before you start writing new code, please! Otherwise, bug fixes are much appreciated! 😄
