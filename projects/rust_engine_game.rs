use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_labels: Vec::new(),
            // FIX: Newer Bevy/rusty_engine Timer API no longer accepts a bool.
            // Use TimerMode::Once to preserve the old `false` behavior (non-repeating).
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

fn main() {
    let mut game = Game::new();

    // setup game here
    // NOTE: old version can accept () but new version must accept Resource
    game.run(GameState::default());
}
