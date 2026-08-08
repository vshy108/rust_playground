# TODO: timer_cli (⭐ 2/10)

## Status

Completed

## Specification

### Goal

Provide a CLI countdown timer and stopwatch with predictable duration parsing
and display formatting.

### Non-goals

- Persisting timers or schedules
- Background services or network access
- Sub-second display precision

### Inputs and outputs

- Input: a positive duration such as `30s`, `5m`, or `1h`, or `stopwatch`
- Output: countdown/elapsed-time display and a completion message
- Optional: `--sound` emits a terminal bell when the countdown completes

### Errors and limits

- Reject zero, malformed, unsupported, and excessively large durations.
- Reject unsupported options and extra stopwatch arguments.

### Acceptance criteria

- [x] Duration parsing supports seconds, minutes, hours, and plain seconds.
- [x] Countdown and stopwatch modes are available.
- [x] Invalid input returns a clear error.
- [x] Pure parsing and formatting tests pass.
- [x] Focused clippy verification passes.

## Usage

```bash
cargo run --bin timer_cli
cargo test --bin timer_cli
```

## Milestones

- [x] Parse durations from CLI arguments.
- [x] Implement countdown display and completion message.
- [x] Add simple stopwatch mode.
- [x] Handle invalid or zero-length durations cleanly.
- [x] Add tests for duration parsing and display formatting helpers.

## Extra

- [x] Add desktop notification hook or sound toggle.

## Tips

- Time parsing and display formatting should be isolated from runtime waiting.
- Use pure helpers for most tests instead of wall-clock timing.

## Change record

- Scope: verified the existing timer implementation and moved it to the
  completed-projects path.
- Assumptions: terminal output and one-second countdown resolution are the
  intended interface.
- Tests added: no new tests; existing tests cover parsing, invalid input, and
  formatting.
- Commands run: `rustfmt projects/completed/timer_cli.rs`, `cargo test --bin
  timer_cli`, and `cargo clippy --bin timer_cli --all-features -- -D warnings`.
- Known limitations: stopwatch duration is displayed after Enter is pressed.
- Follow-up: begin the next unfinished rating-2 project, `base64_cli`.
