// For SyncLazy
#![feature(once_cell)]

mod bitmap;
mod board;
mod game;
mod piece;
mod platform;
mod shader;
mod traits;

use game::Game;

fn main() {
    #[cfg(target_family = "windows")]
    let window = platform::windows::create_window().unwrap();

    #[cfg(target_family = "unix")]
    let (display, window) = platform::unix::create_window().unwrap();

    Game::initialize();
    let game = Game::new();

    #[cfg(target_family = "windows")]
    platform::windows::r#loop(window, game);

    #[cfg(target_family = "unix")]
    platform::unix::r#loop(display, window, game)
}
