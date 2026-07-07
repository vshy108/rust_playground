use std::{cmp::max, time::Duration};

use rusty_time::Timer;

use crate::rusty::{
    NUM_COLS, NUM_ROWS,
    frame::{Drawable, Frame},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if x > 1 && x < NUM_COLS - 2 && y > 0 && y < 9 && x % 2 == 0 && y % 2 == 0 {
                    army.push(Invader { x, y })
                }
            }
        }
        Self {
            army,
            move_timer: Timer::new(Duration::from_secs(2)),
            direction: 1,
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.tick(delta);
        if self.move_timer.just_finished() {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                let new_duration = max(self.move_timer.duration().as_millis() - 250, 250);
                self.move_timer = Timer::new(Duration::from_millis(new_duration as u64));
                // update invader y when downwards
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    // NOTE: when invader.x = 0, self.direction = -1 then become big number
                    // invader.x = ((invader.x as i32) + self.direction) as usize;
                    // checked_add_signed, returning None if overflow occurred
                    if let Some(next_x) = invader.x.checked_add_signed(self.direction as isize) {
                        invader.x = next_x;
                    }
                }
            }
            // cannot true only here for fast return because not last line
            return true;
        }
        false
    }
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }
    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        self.kill_invader(x, y).is_some()
    }
    pub fn kill_invader(&mut self, x: usize, y: usize) -> Option<Invader> {
        let idx = self
            .army
            .iter()
            .position(|invader| invader.x == x && invader.y == y)?;

        Some(self.army.remove(idx))
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            // before 1.1.0, rusty_time has move_timer.time_left
            frame[invader.x][invader.y] = if self.move_timer.percent_left() > 0.5 {
                "x"
            } else {
                "+"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // add new function for Invaders
    impl Invaders {
        fn with_army(army: Vec<Invader>) -> Self {
            let mut invaders = Self::new();
            invaders.army = army;
            invaders
        }
    }

    #[test]
    fn kill_existing_invader() {
        let mut invaders =
            Invaders::with_army(vec![Invader { x: 1, y: 1 }, Invader { x: 5, y: 3 }]);

        assert!(invaders.kill_invader_at(5, 3));

        assert_eq!(invaders.army.len(), 1);
        assert!(!invaders.army.iter().any(|i| i.x == 5 && i.y == 3));
    }

    #[test]
    fn kill_nonexistent_invader() {
        let mut invaders =
            Invaders::with_army(vec![Invader { x: 1, y: 1 }, Invader { x: 5, y: 3 }]);

        assert!(!invaders.kill_invader_at(10, 10));

        assert_eq!(invaders.army.len(), 2);
    }

    #[test]
    fn kill_from_empty_army() {
        let mut invaders = Invaders::with_army(Vec::new());

        assert!(!invaders.kill_invader_at(0, 0));
    }

    #[test]
    fn kill_only_matching_invader() {
        let mut invaders = Invaders::with_army(vec![
            Invader { x: 2, y: 2 },
            Invader { x: 3, y: 3 },
            Invader { x: 4, y: 4 },
        ]);

        invaders.kill_invader_at(3, 3);

        assert_eq!(
            invaders.army,
            vec![Invader { x: 2, y: 2 }, Invader { x: 4, y: 4 },]
        );
    }

    #[test]
    fn kill_middle_invader() {
        let mut invaders = Invaders::with_army(vec![
            Invader { x: 1, y: 1 },
            Invader { x: 2, y: 2 },
            Invader { x: 3, y: 3 },
        ]);

        assert!(invaders.kill_invader_at(2, 2));

        assert_eq!(invaders.army.len(), 2);

        assert_eq!(invaders.army[0].x, 1);
        assert_eq!(invaders.army[1].x, 3);
    }

    #[test]
    fn kill_first_invader() {
        let mut invaders =
            Invaders::with_army(vec![Invader { x: 1, y: 1 }, Invader { x: 2, y: 2 }]);

        assert!(invaders.kill_invader_at(1, 1));

        assert_eq!(invaders.army.len(), 1);
        assert_eq!(invaders.army[0].x, 2);
    }

    #[test]
    fn kill_last_invader() {
        let mut invaders =
            Invaders::with_army(vec![Invader { x: 1, y: 1 }, Invader { x: 2, y: 2 }]);

        assert!(invaders.kill_invader_at(2, 2));

        assert_eq!(invaders.army.len(), 1);
        assert_eq!(invaders.army[0].x, 1);
    }
}
