// example must have main function
// Goal: Events

// Build:

// ```text
// watch folder
// -> detect changes
// -> log
// ```

// Learn:

// - channels
// - filesystem
// - `notify`
//   - `notify` is the standard cross-platform Rust crate for filesystem events.
//   - It gives a real watcher + callback API, which fits this project's shape:
//     watcher callback -> channel -> receive loop -> log.
// 2. Slice 1 architecture — fix the pipeline first:
//    watcher callback -> tx.send(result) -> rx.recv() loop -> println!("{res:?}").
//    The exact watcher crate and exact event type can be decided later; the
//    stable design is producer -> channel -> single consumer.
// 3. Channel choice — start with `std::sync::mpsc::channel()`.
//    `tx` belongs to the watcher callback, `rx` belongs to the main receive loop.
//    `mpsc` fits well because one central loop should consume and log events.
// 3a. Progress — the placeholder closure has been replaced with
//     `recommended_watcher(...)`. `main` now returns `notify::Result<()>`, so
//     watcher setup errors bubble up with `?` instead of panicking with `expect`.
// 4. First receive loop — `while let Ok(res) = rx.recv()` blocks for one event at a
//    time and stops when the channel closes; good enough for the first raw event flow.
// 5. Watcher lifetime — keep the watcher value alive in `main`; if it is dropped
//    early, filesystem watching stops even if the channel receiver still exists.
// 6. Slice 1 success bar — edit one file in the watched folder and see at least
//    one raw event printed. Debug the fundamentals first: watcher alive, path
//    actually watched, callback sending, receive loop running.
// 7. Logging — once raw events are flowing, print useful event details such as
//    path and event kind in a readable format.
// 8. Verification — add a small repeatable smoke test or focused test seam for
//    create / update / delete events.
// 9. Debounce (extra) — reduce noisy bursts of near-duplicate events into a
//    cleaner stream with an explicit tradeoff window.

use std::sync::mpsc::channel;
use notify::{recommended_watcher, RecursiveMode, Watcher};

fn main() -> notify::Result<()> {
    let (tx, rx) = channel();
    // `.` means the current working directory where the program is launched.
    // It does not mean the `projects/` source folder unless you run the binary from there.
    let watch_path = std::path::Path::new(".");

    // Keep the watcher in a local binding so it stays alive for the whole receive loop.
    let mut watcher = recommended_watcher(move |res| {
        let _ = tx.send(res);
    })?;
    watcher.watch(watch_path, RecursiveMode::Recursive)?;

    while let Ok(res) = rx.recv() {
        println!("{res:?}");
    }
    Ok(())
}
