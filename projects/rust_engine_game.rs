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

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    // origin at center of screen
    player.translation = Vec2 { x: 0.0, y: 0.0 };
    // FRAC_PI_2 is pi / 2, SOUTH_WEST for direction
    player.rotation = SOUTH_WEST;
    // zoom in or zoom out
    player.scale = 1.0;
    // this is working like z-index, if got another player, larger layer will on top
    player.layer = 1.0;
    player.collision = true;

    let car_one = game.add_sprite("car1", SpritePreset::RacingCarYellow);
    // translation to car_one
    car_one.translation = Vec2 { x: 300.0, y: 0.0 };
    // NOTE: two objects has collision true only will have Collision event
    car_one.collision = true;

    // setup game here
    game.add_logic(game_logic);
    // NOTE: old version can accept () but new version must accept Resource
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // your actual game logic goes

    // collider is white border nearby the sprite
    // engine.show_colliders = true;

    // handle collisions
    // .drain(..) returns each event and removes it, so each event is handled once.
    for event in engine.collision_events.drain(..) {
        println!("{:#?}", event);
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
            game_state.current_score += 1;
            println!("Current score is {}", game_state.current_score);
        }
    }

    // handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 100.0;
    // NOTE: when the 4 directions in one if..else if, then cannot support diagonal
    // old version is Up ---> ArrowUp, W to KeyW
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
}
