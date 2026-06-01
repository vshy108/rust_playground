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

// Progress:

// 1. Watch setup — decide the watcher crate / event source and the CLI shape:
//    folder path in, event stream out.
// 2. Event flow — start with `std::sync::mpsc::channel()` and keep the split
//    simple: watcher callback holds `tx`, main loop holds `rx`. The callback
//    should send the raw watcher result into the channel; the first receive loop
//    can be `while let Ok(event) = rx.recv()`.
// 3. Slice 1 success bar — keep the first end-to-end goal tiny: change one file
//    in a watched folder and see at least one raw event printed with
//    `println!("{event:?}")`. Learn the real event shape before adding prettier
//    formatting.
// 4. Logging — once raw events are flowing, print useful event details such as
//    path and event kind in a readable format.
// 5. Verification — add a small repeatable smoke test or focused test seam for
//    create / update / delete events.
// 6. Debounce (extra) — reduce noisy bursts of near-duplicate events into a
//    cleaner stream with an explicit tradeoff window.

fn main() {

}