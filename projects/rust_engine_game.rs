use rand::Rng;
use rusty_engine::prelude::*;

// FIX: Magic-number configuration made tuning and maintenance error-prone.
// Centralized constants make game balancing and window tweaks safe and explicit.
const WINDOW_WIDTH: u32 = 1400;
const WINDOW_HEIGHT: u32 = 500;
const MOVEMENT_SPEED: f32 = 100.0;
const SPAWN_X_MIN: f32 = -550.0;
const SPAWN_X_MAX: f32 = 550.0;
const SPAWN_Y_MIN: f32 = -325.0;
const SPAWN_Y_MAX: f32 = 325.0;
const MAX_FERRIES: usize = 200;
const SCORE_TEXT: &str = "score";
const HIGH_SCORE_TEXT: &str = "high_score";

// ThreadRng is internally Rc + UnsafeCell, so it is not Sync.
// GameState is a Bevy-style Resource, and Resource types must be thread-safe.
// So putting ThreadRng inside GameState fails trait bounds.
#[derive(Resource)]
struct GameState {
    high_score: u32,
    score: u32,
    // FIX: Index values never go negative; usize matches indexing/count semantics.
    // This prevents accidental signed/unsigned mismatch in spawn-related logic.
    ferries_index: usize,
    // enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            ferries_index: 0,
            // enemy_labels: Vec::new(),
            // FIX: Newer Bevy/rusty_engine Timer API no longer accepts a bool.
            // Use TimerMode::Repeating to preserve the old `true` behavior (repeating).
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.window_settings(Window {
        title: "Tutorial!".to_string(),
        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        ..Default::default()
    });

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.1);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    // origin at center of screen
    player.translation = Vec2::new(0.0, 0.0);
    // FRAC_PI_2 is pi / 2, SOUTH_WEST for direction
    player.rotation = SOUTH_WEST;
    // zoom in or zoom out
    player.scale = 1.0;
    // this is working like z-index, if got another player, larger layer will on top
    player.layer = 1.0;
    player.collision = true;

    // score display
    let _ = game.add_text(SCORE_TEXT, "Score: 0");
    // score.translation = Vec2::new(520.0, 320.0);

    let _ = game.add_text(HIGH_SCORE_TEXT, "High Score: 0");

    // setup game here
    game.add_logic(game_logic);
    // NOTE: old version can accept () but new version must accept Resource
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // your actual game logic goes

    // quit if Q is pressed
    if engine.keyboard_state.just_pressed(KeyCode::KeyQ) {
        engine.should_exit = true;

        return;
    }

    // keep text near the edges of the screen even window resized
    let offset = ((engine.time_since_startup_f64 * 3.0).cos() * 5.0) as f32;
    // FIX: `unwrap()` would panic if text entities were missing or renamed.
    // Guarding with `if let` keeps the game running and skips only the UI update.
    if let Some(score) = engine.texts.get_mut(SCORE_TEXT) {
        score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
        score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;
    }

    if let Some(high_score) = engine.texts.get_mut(HIGH_SCORE_TEXT) {
        high_score.translation.x = -engine.window_dimensions.x / 2.0 + 120.0;
        high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;
    }

    // collider is white border nearby the sprite
    // engine.show_colliders = true;

    // handle collisions
    // .drain(..) returns each event and removes it, so each event is handled once.
    for event in engine.collision_events.drain(..) {
        // println!("{:#?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            // remove the sprite the player collided with
            // for label in [event.pair.0, event.pair.1] {
            //     if label != "player" {
            //         engine.sprites.remove(&label);
            //     }
            // }
            let other = if event.pair.0 == "player" {
                event.pair.1
            } else {
                event.pair.0
            };
            engine.sprites.remove(&other);

            game_state.score += 1;
            engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.3);
            if let Some(score) = engine.texts.get_mut(SCORE_TEXT) {
                score.value = format!("Score: {}", game_state.score);
            }

            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;

                if let Some(high_score) = engine.texts.get_mut(HIGH_SCORE_TEXT) {
                    high_score.value = format!("High Score: {}", game_state.high_score);
                }
            }
        }
    }

    // handle keyboard movement
    // NOTE: when the 4 directions in one if..else if, then cannot support diagonal
    // old version is Up ---> ArrowUp, W to KeyW
    // pressed or pressed_any for hold-to-move directions
    // FIX: Direct per-axis translation made diagonal movement faster than straight movement.
    // Build one direction vector and normalize so speed stays consistent in every direction.
    let mut direction = Vec2::ZERO;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowUp, KeyCode::KeyW])
    {
        direction.y += 1.0;
    } else if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowDown, KeyCode::KeyS])
    {
        direction.y -= 1.0;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowRight, KeyCode::KeyD])
    {
        direction.x += 1.0;
    } else if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowLeft, KeyCode::KeyA])
    {
        direction.x -= 1.0;
    }

    if direction != Vec2::ZERO && let Some(player) = engine.sprites.get_mut("player") {
        player.translation += direction.normalize() * MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine.keyboard_state.just_pressed(KeyCode::KeyR) {
        game_state.score = 0;
        if let Some(score) = engine.texts.get_mut(SCORE_TEXT) {
            score.value = "Score: 0".to_string();
        }
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let mut rng = rand::rng();
        let x = rng.random_range(SPAWN_X_MIN..SPAWN_X_MAX);
        let y = rng.random_range(SPAWN_Y_MIN..SPAWN_Y_MAX);
        spawn_ferris(engine, game_state, Vec2 { x, y });
    }

    // handle mouse input
    // just_pressed for one-shot actions
    if engine.mouse_state.just_pressed(MouseButton::Left)
        && let Some(mouse_location) = engine.mouse_state.location()
    {
        spawn_ferris(engine, game_state, mouse_location);
    }
}

fn spawn_ferris(engine: &mut Engine, game_state: &mut GameState, position: Vec2) {
    let active_ferries = engine
        .sprites
        .keys()
        .filter(|label| label.starts_with("ferries"))
        .count();

    // FIX: Unbounded spawns can degrade performance over long sessions.
    // Enforcing a cap avoids runaway entity growth while keeping gameplay stable.
    if active_ferries >= MAX_FERRIES {
        return;
    }

    // FIX: Spawn logic previously lived in multiple call sites and could drift.
    // This helper centralizes label/index/collision setup so both spawn paths stay consistent.
    let label = format!("ferries{}", game_state.ferries_index);
    game_state.ferries_index += 1;

    let ferris = engine.add_sprite(label, SpritePreset::RacingCarYellow);
    ferris.translation = position;
    // NOTE: two objects has collision true only will have Collision event
    ferris.collision = true;
}
