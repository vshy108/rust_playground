# TODO: file_watcher

## Usage

```bash
# planned binary name once implementation starts:
cargo run --bin watch -- /tmp/folder
cargo test --bin watch
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

## Progress

- [ ] Slice 1: choose the watcher crate / event source and define the event flow.
- [ ] Slice 2: watch a folder and receive events through a channel.
- [ ] Slice 3: log useful event details (path, kind, timestamp/order).
- [ ] Slice 4: add focused tests or a small manual smoke-check plan.
- [ ] Extra: debounce noisy event bursts into a cleaner stream.

Progress notes:

- Slice 1 mental model: watcher callback -> channel -> receive loop -> log.
- Channel choice: start with `std::sync::mpsc::channel()`.
	- `tx` stays with the watcher callback.
	- `rx` stays with the main receive loop.
	- `mpsc` fits this project because one central loop should consume and log events.
- First receive loop shape: `while let Ok(event) = rx.recv() { ... }`
	- this blocks for one event at a time and stops when the channel closes.
- First implementation slice should send the raw watcher result through the channel and
	print it with debug formatting before attempting prettier logging or debounce.
- Smallest Slice 1 success criterion: change one file in a watched folder and see at
	least one raw event printed.

## 1. Watch Setup

- [ ] Pick a filesystem watching approach.
- [ ] Define the CLI shape: folder path in, event stream out.
- [ ] Decide what event data to keep: path, event kind, maybe timestamp.

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

## 2. Event Loop

- [ ] Start watching a provided folder.
- [ ] Receive filesystem events through a channel.
- [ ] Keep the process alive so events can continue arriving.

Acceptance check:

```text
Create / edit / delete a file in the watched folder.
Expected: an event arrives and is handled by the receive loop.
```

## 3. Logging

- [ ] Print each event in a readable format.
- [ ] Include enough detail to tell files apart and understand what changed.
- [ ] Avoid panicking on ordinary watcher noise or duplicate events.

Acceptance check:

```text
Change a file in the watched folder.
Expected: output shows the path and event kind in a readable log line.
```

## 4. Verification

- [ ] Add at least one focused testable seam, or document a repeatable manual smoke test.
- [ ] Confirm create/update/delete scenarios are visible.
- [ ] Confirm the program handles watching an existing folder without crashing.

Acceptance check:

```bash
# once implementation exists
cargo test --bin watch
```

## Extra: Debounce

- [ ] Group rapid duplicate events into one cleaner logical update.
- [ ] Decide on a debounce window and document the tradeoff.
- [ ] Verify that saving a file no longer floods the output with near-identical events.

Acceptance check:

```text
Rapid writes to one file should produce fewer, more meaningful log lines.
```