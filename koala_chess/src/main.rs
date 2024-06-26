mod bitmap;
mod board;
mod game;
mod mat4;
mod piece;
mod platform;
mod player;
mod projections;
mod renderer;
mod shader;
mod transformations;
mod vec3;
mod vec4;

use game::Game;

#[cfg(target_family = "windows")]
fn main() {
    // Create the window
    let window = platform::windows::create_window();

    // Initialize the game
    Game::initialize();
    let mut game = Game::new();

    // Enter the game loop
    platform::windows::r#loop(window, &mut game);
}

#[cfg(target_family = "unix")]
fn main() {
    // Create the window
    let (display, window) = platform::unix::create_window();

    // Initialize the game
    Game::initialize();
    let mut game = Game::new();

    // Enter the game loop
    platform::unix::r#loop(display, window, &mut game)
}
