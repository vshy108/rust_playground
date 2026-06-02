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
// 7a. Progress — raw `Debug` printing has been replaced with `format_event(...)`
//     so logging now has a small pure seam that is easy to test.
// 8. Verification — add a small repeatable smoke test or focused test seam for
//    create / update / delete events.
// 8a. Progress — focused unit tests now cover formatter output for a single path,
//     multiple paths, and watcher errors.
// 9. Debounce (extra) — reduce noisy bursts of near-duplicate events into a
//    cleaner stream with an explicit tradeoff window.

use notify::{RecursiveMode, Watcher, recommended_watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// Holds the latest event for a given path during the debounce window.
// Replacing an entry with a newer event collapses a burst into one log line.
struct PendingEvent {
    // The sequence number assigned when this event first entered the buffer.
    event_number: usize,
    // The latest raw event for this path; the whole Result is stored so the
    // formatter sees the same type it already handles.
    event: notify::Result<notify::Event>,
    // Reset to `Instant::now()` every time a newer event replaces this entry.
    // Flushing checks `last_seen.elapsed() >= debounce_window`.
    last_seen: Instant,
}

// Sent through the mpsc channel so the receive loop can handle both filesystem
// events and a graceful shutdown signal from the Ctrl+C handler.
enum WatchMessage {
    Event(notify::Result<notify::Event>),
    Shutdown,
}

fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        // 1970-01-01 00:00:00 UTC
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn format_event(res: &notify::Result<notify::Event>) -> String {
    match res {
        Ok(event) => {
            let paths = if event.paths.is_empty() {
                String::from("<no paths>")
            } else {
                // `map(...)` transforms each path into a `String`.
                // `collect::<Vec<_>>()` gathers those transformed items into a vector
                // so `.join(", ")` can turn them into one display string.
                event
                    .paths
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };

            format!("kind={:?} paths={paths}", event.kind)
        }
        Err(err) => format!("watch error: {err}"),
    }
}

fn format_event_line(event_number: usize, res: &notify::Result<notify::Event>) -> String {
    // Prefix each line with epoch milliseconds so event bursts can be compared in time
    // without pulling in a date/time formatting crate for this learning slice.
    let timestamp_ms = current_timestamp_ms();
    format!(
        "[{timestamp_ms}] event#{event_number} {}",
        format_event(res)
    )
}

// Print and remove each entry in `keys` from `pending`.
// Keys are passed in as a pre-collected Vec rather than derived inside this function
// because the caller holds a borrow on `pending` for filtering — passing them
// separately avoids a simultaneous mutable + immutable borrow of the same HashMap.
fn flush_pending_keys(
    keys: Vec<PathBuf>,
    pending: &mut HashMap<PathBuf, PendingEvent>,
    output: &mut impl FnMut(String),
) {
    for key in keys {
        if let Some(p) = pending.remove(&key) {
            output(format_event_line(p.event_number, &p.event));
        }
    }
}

// Core event loop: receives WatchMessages and applies debounce logic.
// `output` is a closure called for every log line — `main` passes `|line| println!("{line}")`
// while tests pass a collector so they can assert on the lines without touching stdout.
fn run_loop(rx: Receiver<WatchMessage>, debounce_window: Duration, mut output: impl FnMut(String)) {
    let mut event_number = 1;
    let mut pending: HashMap<PathBuf, PendingEvent> = HashMap::new();

    // `loop` is used instead of `while let Ok(...)` because `recv_timeout` returns
    // `Err(RecvTimeoutError::Timeout)` on every quiet tick. `while let Ok` would exit
    // the loop on that `Err`, dropping all buffered pending events. `loop` + `match`
    // lets us handle Timeout as a no-op and continue to the flush step.
    loop {
        match rx.recv_timeout(debounce_window) {
            Ok(WatchMessage::Event(res)) => match res {
                // The `if` here is a match guard, not an `if` expression.
                // When the guard fails (0 paths or 2+ paths), Rust does NOT drop
                // the event — it falls through to the next arm (`other =>`).
                Ok(event) if event.paths.len() == 1 => {
                    // Single-path success event: store/replace in pending so bursts
                    // for the same file collapse into the latest event seen.
                    let path = event.paths[0].clone();
                    pending.insert(
                        path,
                        PendingEvent {
                            event_number,
                            event: Ok(event),
                            last_seen: Instant::now(),
                        },
                    );
                    event_number += 1;
                }
                other => {
                    // Errors, empty-path events, and multi-path events are printed
                    // immediately because they cannot be collapsed by path.
                    output(format_event_line(event_number, &other));
                    event_number += 1;
                }
            },
            Ok(WatchMessage::Shutdown) => {
                // Flush any remaining pending events before stopping.
                let ready_keys: Vec<PathBuf> = pending.keys().cloned().collect();
                flush_pending_keys(ready_keys, &mut pending, &mut output);
                output("stopping watcher".to_string());
                break;
            }
            // Timeout means no new event arrived; fall through to flush ready entries.
            Err(RecvTimeoutError::Timeout) => {}
            // Channel closed (watcher dropped); stop.
            Err(RecvTimeoutError::Disconnected) => break,
        }

        // After every tick (new event or timeout), flush entries whose debounce
        // window has expired.
        // Collect keys into a Vec first: calling `pending.remove()` inside
        // `pending.iter()` would require two mutable borrows of the same HashMap
        // at the same time, which Rust does not allow.
        let ready_keys: Vec<PathBuf> = pending
            .iter()
            .filter(|(_, p)| p.last_seen.elapsed() >= debounce_window)
            .map(|(k, _)| k.clone())
            .collect();
        flush_pending_keys(ready_keys, &mut pending, &mut output);
    }
}

fn main() -> notify::Result<()> {
    // `.` means the current working directory where the program is launched.
    // It does not mean the `projects/` source folder unless you run the binary from there.
    let path_arg = std::env::args().nth(1).unwrap_or(".".to_string());
    let (tx, rx) = channel();
    // `Path` is a borrowed view of a path. Use `PathBuf` when a struct needs to own
    // and store path data, like `notify::Event { paths: Vec<PathBuf> }` in the tests.
    let watch_path = std::path::Path::new(&path_arg);

    // Ctrl+C cannot interrupt `rx.recv()` directly, so the signal handler sends an
    // explicit shutdown message through the same channel to unblock the receive loop.
    let shutdown_tx = tx.clone();
    ctrlc::set_handler(move || {
        let _ = shutdown_tx.send(WatchMessage::Shutdown);
    })
    .map_err(|err| notify::Error::generic(&format!("failed to install Ctrl+C handler: {err}")))?;

    // Keep the watcher in a local binding so it stays alive for the whole receive loop.
    let mut watcher = recommended_watcher(move |res| {
        let _ = tx.send(WatchMessage::Event(res));
    })?;
    watcher.watch(watch_path, RecursiveMode::Recursive)?;
    println!("watching {}", watch_path.display());

    // Events for the same path within this window are collapsed into one log line.
    run_loop(rx, Duration::from_millis(100), |line| println!("{line}"));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{WatchMessage, format_event, format_event_line, run_loop};
    // `notify::Event` stores owned paths (`Vec<PathBuf>`), so tests build `PathBuf`
    // values directly. In contrast, `Path::new(".")` in `main` only borrows a path.
    use std::path::PathBuf;
    use std::sync::mpsc::channel;
    use std::time::Duration;

    use notify::{
        Error, Event,
        event::{CreateKind, EventKind},
    };

    #[test]
    fn formats_success_event_with_single_path() {
        let event = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![PathBuf::from("/tmp/demo.txt")],
            // `attrs` expects `EventAttributes`, so `Default::default()` means
            // `EventAttributes::default()` here: an empty attribute set for this test.
            attrs: Default::default(),
        };

        let formatted = format_event(&Ok(event));

        assert_eq!(formatted, "kind=Create(File) paths=/tmp/demo.txt");
    }

    #[test]
    fn formats_success_event_with_multiple_paths() {
        let event = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![PathBuf::from("/tmp/from.txt"), PathBuf::from("/tmp/to.txt")],
            attrs: Default::default(),
        };

        let formatted = format_event(&Ok(event));

        assert_eq!(
            formatted,
            "kind=Create(File) paths=/tmp/from.txt, /tmp/to.txt"
        );
    }

    #[test]
    fn formats_watch_errors() {
        let formatted = format_event(&Err(Error::generic("boom")));

        assert_eq!(formatted, "watch error: boom");
    }

    #[test]
    fn prefixes_event_order_for_log_lines() {
        let event = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![PathBuf::from("/tmp/demo.txt")],
            attrs: Default::default(),
        };

        let formatted = format_event_line(3, &Ok(event));

        assert!(formatted.starts_with("["));
        assert!(formatted.ends_with("event#3 kind=Create(File) paths=/tmp/demo.txt"));
    }

    #[test]
    fn debounce_collapses_burst_for_same_path() {
        // Tests the core debounce invariant: multiple events for the same path
        // within the window are replaced in the HashMap, so only the last one
        // survives to be flushed.
        //
        // Using a 5-second window means none of the events expire on their own —
        // the Shutdown message is what triggers the final flush. This keeps the
        // test fast and deterministic: no real sleeping needed.
        let (tx, rx) = channel();
        let path = PathBuf::from("/tmp/test.txt");

        // Send 3 events for the same path in rapid succession.
        for _ in 0..3 {
            let event = Event {
                kind: EventKind::Create(CreateKind::File),
                paths: vec![path.clone()],
                attrs: Default::default(),
            };
            tx.send(WatchMessage::Event(Ok(event))).unwrap();
        }
        tx.send(WatchMessage::Shutdown).unwrap();

        let mut out: Vec<String> = Vec::new();
        // Large window so none expire naturally; Shutdown flushes whatever is pending.
        run_loop(rx, Duration::from_millis(5000), |line| out.push(line));

        // All 3 events collapse into 1 log line for that path, plus "stopping watcher".
        assert_eq!(out.len(), 2);
        assert!(out[0].contains("test.txt"));
        assert_eq!(out[1], "stopping watcher");
    }

    #[test]
    fn debounce_keeps_separate_paths_separate() {
        // Tests that debounce uses the path as the HashMap key, so events for
        // different paths are buffered independently and each produces its own
        // log line — debounce only collapses events *for the same path*.
        let (tx, rx) = channel();

        // One event each for two different paths.
        for name in &["a.txt", "b.txt"] {
            let event = Event {
                kind: EventKind::Create(CreateKind::File),
                paths: vec![PathBuf::from(format!("/tmp/{name}"))],
                attrs: Default::default(),
            };
            tx.send(WatchMessage::Event(Ok(event))).unwrap();
        }
        tx.send(WatchMessage::Shutdown).unwrap();

        let mut out: Vec<String> = Vec::new();
        run_loop(rx, Duration::from_millis(5000), |line| out.push(line));

        // Each path produces its own log line, plus "stopping watcher".
        assert_eq!(out.len(), 3);
        assert_eq!(out[2], "stopping watcher");
    }
}
