// FIX: NUM_COLS/NUM_ROWS live in the parent rusty module, so this file failed
// name resolution on its own. Importing them from super restores compilation.
use super::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        // error: it looks like the same item is being pushed into this `Vec`
        // let mut col = Vec::with_capacity(NUM_ROWS);
        // for _ in 0..NUM_ROWS {
        //     col.push(" ");
        // }
        let col = vec![" ";NUM_ROWS];
        // `col.extend(std::iter::repeat_n(" ", NUM_ROWS))`
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}