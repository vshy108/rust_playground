use std::time::Duration;

use rusty_time::Timer;

use crate::rusty::frame::{Drawable, Frame};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            // before 1.1.0, rusty_time has Timer::from_millis()
            timer: Timer::new(Duration::from_millis(50)),
        }
    }
    pub fn update(&mut self, delta: Duration) {
        // before 1.1.0, rusty_time has update()
        self.timer.tick(delta);
        // before 1.1.0, rusty_time has ready
        if self.timer.just_finished() && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::new(Duration::from_millis(250));
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.just_finished()) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" }
    }
}
