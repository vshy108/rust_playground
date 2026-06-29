use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Color, Print, SetBackgroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{Stdout, Write};

use crate::rusty::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                // FIX: `print!` writes to global stdout, which can desync with queued cursor commands.
                // Queue the glyph on the same writer to keep cursor movement and drawing in lockstep.
                // OLD: print!("{}", *s);
                stdout.queue(Print(*s)).unwrap();
            }
        }
    }

    stdout.flush().unwrap();
}
