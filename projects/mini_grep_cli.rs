// example must have main function
// Goal: Ownership + iterators

// Build:

// ```bash
// rgrep "hello" logs.txt
// ```

// Learn:

// - borrowing
//   - References let you use a value without taking ownership.
//   - &str is a borrowed string slice; String is an owned heap string.
//   - Functions that only read data should take &str, not String.
//
// - iterators
//   - Iterators are lazy: they produce values on demand, not all at once.
//   - .lines() splits a &str into an iterator of &str lines.
//   - .filter() keeps only items where the closure returns true.
//   - .enumerate() pairs each item with its 0-based index.
//   - .collect() pulls all iterator values into a Vec or other collection.
//
// - slices
//   - A slice (&[T] or &str) is a view into a contiguous sequence.
//   - args: &[String] is a slice of the collected CLI arg Vec.
//   - Slices do not own their data; they borrow it.

// Progress:

// (fill in as you implement)

// Extra:

// - [ ] regex support (use the `regex` crate)

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM: &str = "rgrep";
}