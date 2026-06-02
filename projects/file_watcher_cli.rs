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
use std::sync::mpsc::channel;
use std::time::{SystemTime, UNIX_EPOCH};

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
    format!("[{timestamp_ms}] event#{event_number} {}", format_event(res))
}

fn main() -> notify::Result<()> {
    // `.` means the current working directory where the program is launched.
    // It does not mean the `projects/` source folder unless you run the binary from there.
    let path_arg = std::env::args().nth(1).unwrap_or(".".to_string());
    let (tx, rx) = channel();
    // `Path` is a borrowed view of a path. Use `PathBuf` when a struct needs to own
    // and store path data, like `notify::Event { paths: Vec<PathBuf> }` in the tests.
    let watch_path = std::path::Path::new(&path_arg);

    // Keep the watcher in a local binding so it stays alive for the whole receive loop.
    let mut watcher = recommended_watcher(move |res| {
        let _ = tx.send(res);
    })?;
    watcher.watch(watch_path, RecursiveMode::Recursive)?;
    println!("watching {}", watch_path.display());

    let mut event_number = 1;

    while let Ok(res) = rx.recv() {
        println!("{}", format_event_line(event_number, &res));
        event_number += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{format_event, format_event_line};
    // `notify::Event` stores owned paths (`Vec<PathBuf>`), so tests build `PathBuf`
    // values directly. In contrast, `Path::new(".")` in `main` only borrows a path.
    use std::path::PathBuf;

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
}
