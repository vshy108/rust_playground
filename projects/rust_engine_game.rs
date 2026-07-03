use rand::Rng;
use rusty_engine::prelude::*;

// ThreadRng is internally Rc + UnsafeCell, so it is not Sync.
// GameState is a Bevy-style Resource, and Resource types must be thread-safe.
// So putting ThreadRng inside GameState fails trait bounds.
#[derive(Resource)]
struct GameState {
    high_score: u32,
    score: u32,
    ferries_index: i32,
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
    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);

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
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", game_state.score);

            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;

                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High score: {}", game_state.high_score);
            }
        }
    }

    // handle keyboard movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 100.0;
    // NOTE: when the 4 directions in one if..else if, then cannot support diagonal
    // old version is Up ---> ArrowUp, W to KeyW
    // pressed or pressed_any for hold-to-move directions
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowUp, KeyCode::KeyW])
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    } else if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowDown, KeyCode::KeyS])
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowRight, KeyCode::KeyD])
    {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    } else if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowLeft, KeyCode::KeyA])
    {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine.keyboard_state.just_pressed(KeyCode::KeyR) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("ferries{}", game_state.ferries_index);
        game_state.ferries_index += 1;

        let ferris = engine.add_sprite(label.clone(), SpritePreset::RacingCarYellow);
        let mut rng = rand::rng();
        let x = rng.random_range(-550.0..550.0);
        let y = rng.random_range(-325.0..325.0);
        ferris.translation = Vec2 { x, y };
        // NOTE: two objects has collision true only will have Collision event
        ferris.collision = true;
    }

    // handle mouse input
    // just_pressed for one-shot actions
    if engine.mouse_state.just_pressed(MouseButton::Left)
        && let Some(mouse_location) = engine.mouse_state.location()
    {
        let label = format!("ferries{}", game_state.ferries_index);
        game_state.ferries_index += 1;

        let ferris = engine.add_sprite(label.clone(), SpritePreset::RacingCarYellow);
        ferris.translation = mouse_location;
        // NOTE: two objects has collision true only will have Collision event
        ferris.collision = true;
    }
}
