use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

mod rusty;

use crossterm::{
    ExecutableCommand,
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use rusty_audio::Audio;

use crate::rusty::{
    frame::{self, Drawable, new_frame},
    invaders::Invaders,
    player::Player,
    render,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    // when running with cargo run, the working directory is root folder
    audio.add("explode", "assets/audio/sounds/explode.wav");
    audio.add("lose", "assets/audio/sounds/lose.wav");
    audio.add("move", "assets/audio/sounds/move.wav");
    audio.add("pew", "assets/audio/sounds/pew.wav");
    audio.add("startup", "assets/audio/sounds/startup.wav");
    audio.add("win", "assets/audio/sounds/win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    // Hide cursor
    stdout.execute(Hide)?;

    // Render loop in a separate thread.
    // FIX: Use a bounded channel to prevent stale-frame backlog and input lag.
    // OLD: let (render_tx, render_rx) = mpsc::channel();
    let (render_tx, render_rx) = mpsc::sync_channel(1);
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        // NOTE: remember to have () or not, else closure assigned
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        // CLIPPY: since Err is just break, the loop then let can be while let
        // loop {
        //     let curr_frame = match render_rx.recv() {
        //         Ok(x) => x,
        //         Err(_) => break,
        //     };
        //     render::render(&mut stdout, &last_frame, &curr_frame, false);
        //     last_frame = curr_frame;
        // }
        while let Ok(x) = render_rx.recv() {
            let curr_frame = x;
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        // for next delta calculation
        instant = Instant::now();
        let mut curr_frame = new_frame();
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    // spacebar use ' '
                    KeyCode::Char(' ') | KeyCode::Enter if player.can_shoot() => {
                        player.shoot();
                        audio.play("pew");
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // Draw & render
        // trait `Drawable` which provides `draw` is implemented but not in scope;
        // use crate::rusty::Drawable
        // &mut curr_frame first curr_frame needs let mut
        // Replace below with &dyn Drawable Generic
        // player.draw(&mut curr_frame);
        // invaders.draw(&mut curr_frame);
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or lose?
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    // Clean Up
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
