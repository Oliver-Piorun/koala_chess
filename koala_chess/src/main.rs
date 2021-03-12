// For SyncLazy
#![feature(once_cell)]

mod bitmap;
mod board;
mod game;
mod piece;
mod platform;
mod renderer;
mod shader;
mod traits;

use game::Game;

#[cfg(target_family = "windows")]
fn main() {
    // Create the window
    let window = platform::windows::create_window().unwrap();

    // Initialize the game
    Game::initialize();
    let game = Game::new();

    // Enter the game loop
    platform::windows::r#loop(window, game);
}

#[cfg(target_family = "unix")]
fn main() {
    // Create the window
    let (display, window) = platform::unix::create_window().unwrap();

    // Initialize the game
    Game::initialize();
    let game = Game::new();

    // Enter the game loop
    platform::unix::r#loop(display, window, game)
}
