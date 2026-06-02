# TODO: file_watcher

## Usage

```bash
# current Slice 1 binary:
cargo run --bin watchdir

# manual smoke check used so far:
touch .watchdir_smoke && rm .watchdir_smoke
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
- Current watch path: `.` means the program's current working directory. When run from
	the repo root, it watches `rust_playground/`.
- Smallest Slice 1 success criterion: change one file in a watched folder and see at
	least one raw event printed.
- Completed smoke check: running `cargo run --bin watchdir` from the repo root and then
	`touch .watchdir_smoke && rm .watchdir_smoke` produced raw `Create(File)` and
	`Remove(File)` events.

## 1. Watch Setup

- [x] Pick a filesystem watching approach.
- [x] Define the first CLI shape: current directory in, raw event stream out.
- [ ] Decide what event data to keep for nicer logs: path, event kind, maybe timestamp.

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

- [ ] Print each event in a readable format.
- [ ] Include enough detail to tell files apart and understand what changed.
- [ ] Avoid panicking on ordinary watcher noise or duplicate events.

Acceptance check:

```text
Change a file in the watched folder.
Expected: output shows the path and event kind in a readable log line.
```

## 4. Verification

- [x] Add at least one focused testable seam, or document a repeatable manual smoke test.
- [ ] Confirm create/update/delete scenarios are visible.
- [x] Confirm the program handles watching an existing folder without crashing.

Acceptance check:

```bash
# from the repo root in one terminal
cargo run --bin watchdir

# in another terminal
touch .watchdir_smoke && rm .watchdir_smoke
```

## Extra: Debounce

- [ ] Group rapid duplicate events into one cleaner logical update.
- [ ] Decide on a debounce window and document the tradeoff.
- [ ] Verify that saving a file no longer floods the output with near-identical events.

Acceptance check:

```text
Rapid writes to one file should produce fewer, more meaningful log lines.
```