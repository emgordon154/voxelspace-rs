use voxelspace_rs::game::Game;
use std::env;

fn main() {
    #[cfg(debug_assertions)]
    env::set_var("RUST_BACKTRACE", "1");

    let game = Game::new();
    game.run();
}