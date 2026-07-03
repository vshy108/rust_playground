use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState;

fn main() {
    let mut game = Game::new();

    // setup game here
    game.run(GameState);
}
