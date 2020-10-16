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
    {
        let window = platform::windows::create_window().unwrap();
        
        Game::initialize();
        let game = Game::new();
        
        platform::windows::r#loop(window, game);
    }
}
