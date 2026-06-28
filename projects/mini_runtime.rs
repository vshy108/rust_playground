// example must have main function
// Goal: Deep Rust

// Build:

// ```bash
// cargo run --bin mini_runtime
// ```

// Learn:

// - pinning — `Pin<P>` prevents a value from being moved after it is pinned; required for
//   self-referential structs (which store internal pointers that would be invalidated by a move)
// - unsafe — the raw `Future::poll` method takes `Pin<&mut Self>`; manually implementing a
//   future requires `unsafe { Pin::new_unchecked(&mut val) }` when the struct is not Unpin
// - async internals — an executor calls `future.poll(cx)` in a loop; when poll returns
//   `Pending`, the Waker stored in `cx` is saved; when the future is ready to make progress,
//   it calls `waker.wake()` to re-schedule itself; the executor then calls `poll` again

// Notes:

// Extra:

// - [ ] multi-threaded scheduler — distribute tasks across a thread pool with work stealing

fn main() {
    println!("mini_runtime: not yet implemented");
}
