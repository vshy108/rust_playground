# TODO: file_watcher

## Usage

```bash
# watch current directory
cargo run --bin watchdir

# watch a specific directory
cargo run --bin watchdir -- /tmp/testdir

# manual smoke checks used so far:
touch .watchdir_smoke && rm .watchdir_smoke
touch /tmp/testdir/abc
echo hi >> /tmp/testdir/abc
rm /tmp/testdir/abc
```

## Goal

Detect filesystem changes in a folder and log the events.

Build:

```text
watch folder
→ detect changes
→ log
```

## Learn

- channels
- filesystem
- notify

## Progress

- [x] Slice 1: choose the watcher crate / event source and define the event flow.
- [x] Slice 2: watch a folder and receive events through a channel.
- [ ] Slice 3: log useful event details (path, kind, timestamp/order).
- [x] Slice 4: add focused tests or a small manual smoke-check plan.
- [ ] Extra: debounce noisy event bursts into a cleaner stream.

Progress notes:

- Slice 1 mental model: watcher callback -> channel -> receive loop -> log.
- Slice 3 has started: log lines now include timestamps, event order, event kind,
	and paths.
- Current watcher crate: `notify`, using `recommended_watcher(...)`.
- Channel choice: start with `std::sync::mpsc::channel()`.
	- `tx` stays with the watcher callback.
	- `rx` stays with the main receive loop.
	- `mpsc` fits this project because one central loop should consume and log events.
	- `mpsc` = multi-producer, single-consumer. Even if Slice 1 only has one callback,
	  the sender can be cloned later; the important part is still one central consumer.
- First receive loop shape: `while let Ok(event) = rx.recv() { ... }`
	- this blocks for one event at a time and stops when the channel closes.
- First implementation slice should send the raw watcher result through the channel and
	print it with debug formatting before attempting prettier logging or debounce.
- First payload choice: send the raw `Result<Event, Error>` through the channel so the
	main loop can observe both successful events and watcher errors early.
- Architecture first: it is okay to leave the exact watcher crate and exact event type
	undecided at first, as long as the pipeline shape is fixed:
	callback -> channel -> receive loop.
- Closure capture: if the watcher crate expects a callback like `move |res| { ... }`,
	that closure usually needs to capture `tx` by move so it still owns the sender when
	filesystem events arrive later.
- Watcher lifetime: keep the watcher value alive in `main`; if it is dropped early,
	filesystem watching stops even if the receive loop is still blocked on `rx.recv()`.
- CLI watch path support exists: `cargo run --bin watchdir -- /tmp/testdir` watches
	the provided folder, while no argument still falls back to `.`.
- Current watch path: `.` means the program's current working directory. When run from
	the repo root, it watches `rust_playground/`.
- Startup visibility exists: the program prints `watching {path}` after
	`watcher.watch(...)` succeeds and before the receive loop starts.
- Graceful shutdown exists: Ctrl+C now sends an explicit shutdown message through
	the channel so the blocked receive loop can print `stopping watcher` and exit.
- Smallest Slice 1 success criterion: change one file in a watched folder and see at
	least one raw event printed.
- Completed smoke check: running `cargo run --bin watchdir` from the repo root and then
	`touch .watchdir_smoke && rm .watchdir_smoke` produced raw `Create(File)` and
	`Remove(File)` events.
- Formatting is no longer raw debug-only:
	- `format_event(...)` formats event kind and paths.
	- `format_event_line(...)` prefixes logs with epoch milliseconds and `event#N`.
- Focused formatter tests now cover:
	- single path formatting
	- multiple path formatting
	- watcher error formatting
	- event-order prefix formatting
- Manual verification so far:
	- missing path returns `PathNotFound`
	- existing custom path can be watched successfully
	- Ctrl+C exits cleanly after printing `stopping watcher`
	- a fresh file in `/tmp/testdir` produced visible `Create(File)`,
	  `Modify(Data(Content))`, and `Remove(File)` events on the custom watch path
	- event order can be bursty or backend-dependent, so rapid create/write/remove
	  actions may not print in the same order the shell commands were issued

## 1. Watch Setup

- [x] Pick a filesystem watching approach.
- [x] Define the first CLI shape: current directory in, raw event stream out.
- [x] Expand the first CLI shape to accept an optional custom watch path.
- [x] Keep the first readable log shape to: timestamp, event order, event kind, and paths.
- [ ] Revisit whether timestamps still pull their weight after debounce is added.

Acceptance check:

```text
Can explain the planned control flow:
watcher -> channel -> receive loop -> log
```

Slice 1 Feynman Q&A:

- Q: What is the main job of the channel here?
	A: Move filesystem events from the watcher side to the logging loop.
- Q: Inside the watcher callback, what should happen first?
	A: Send the event into the channel so one central loop can handle it.
- Q: Which side should the watcher callback hold?
	A: The sender (`tx`).
- Q: What should the main thread do after the watcher starts?
	A: Block in a receive loop waiting for events from the channel.
- Q: What problem is debounce trying to solve?
	A: Collapse noisy bursts into fewer, more useful logical events.
- Q: What is the best first implementation slice?
	A: Make one raw filesystem event successfully travel through watcher -> channel -> receive loop -> `println!`.
- Q: What is the simplest useful first log output?
	A: Print the raw event with debug formatting, e.g. `println!("{event:?}")`.
- Q: What is the smallest success criterion for Slice 1?
	A: Change one file in a watched folder and see at least one raw event printed.
- Q: Why start with `mpsc`?
	A: It matches the current shape well: watcher side produces, one main loop consumes.
- Q: Why is one consumer a good thing for Slice 1?
	A: One receive loop keeps logging, formatting, and future debounce in one place.
- Q: If the watcher callback yields `Result<Event, Error>`, what is the simplest first
	  thing to send?
	A: Send the whole `Result` so successes and failures are both visible early.
- Q: If the exact watcher crate is not chosen yet, what can still be fixed now?
	A: The architecture: channel + callback + receive loop.
- Q: Why might `tx` need to be moved into the watcher callback?
	A: Because the callback runs later and needs to own or capture the sender it will use
	   to call `tx.send(...)`.

## 2. Event Loop

- [x] Start watching the current working directory.
- [x] Receive filesystem events through a channel.
- [x] Keep the process alive so events can continue arriving.

Acceptance check:

```text
Create / edit / delete a file in the watched folder.
Expected: an event arrives and is handled by the receive loop.
```

## 3. Logging

- [x] Prefix each event with a stable order number.
- [x] Print event kind and paths in a readable format.
- [x] Prefix each log line with epoch milliseconds.
- [ ] Avoid panicking on ordinary watcher noise or duplicate events.

Acceptance check:

```text
Change a file in the watched folder.
Expected: output shows timestamp, event order, event kind, and path in one readable log line.
```

## 4. Verification

- [x] Add at least one focused testable seam, or document a repeatable manual smoke test.
- [x] Confirm create/update/delete scenarios are visible on one chosen custom watch path.
- [x] Confirm the program handles watching an existing folder without crashing.
- [x] Confirm the program reports a missing watch path as `PathNotFound`.
- [x] Confirm Ctrl+C stops the watcher cleanly.

Acceptance check:

```bash
# from the repo root in one terminal
cargo run --bin watchdir

# in another terminal
touch .watchdir_smoke && rm .watchdir_smoke

# or with a custom path
cargo run --bin watchdir -- /tmp/testdir
touch /tmp/testdir/abc
echo hi >> /tmp/testdir/abc
rm /tmp/testdir/abc

# for a cleaner create/modify/remove check, use a fresh filename
f=/tmp/testdir/verify_$RANDOM && touch "$f" && echo hi >> "$f" && rm "$f"

# then in the watcher terminal
# press Ctrl+C
```

## Extra: Debounce

- [ ] Group rapid duplicate events into one cleaner logical update.
- [ ] Decide on a debounce window and document the tradeoff.
- [ ] Verify that saving a file no longer floods the output with near-identical events.

Acceptance check:

```text
Rapid writes to one file should produce fewer, more meaningful log lines.
```