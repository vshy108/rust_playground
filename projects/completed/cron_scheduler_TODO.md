# TODO: cron_scheduler (⭐ 5/10)

## Usage

```bash
cargo run --bin cron_scheduler
cargo test --bin cron_scheduler
```

## Milestones

- [x] Parse a minimal cron-like schedule expression.
- [x] Compute the next execution time from a base timestamp.
- [x] Run shell commands or internal tasks on schedule.
- [x] Add graceful shutdown and missed-run handling.
- [x] Add tests for schedule parsing and boundary times.

## Extra

- [x] Add persistent job definitions in a config file.

## Status

Completed.

## Specification

- Goal: calculate and run minimal cron-like jobs with explicit missed-run behavior.
- Inputs: five-field schedule expressions, Unix timestamps, commands, or tab-separated config files.
- Output: next execution timestamps and optional command execution.
- Errors: reject malformed fields, invalid timestamps, malformed config records,
  and schedules without a match in the bounded search window.
- Non-goals: daemon management, timezone-specific schedules, and automatic retries.
- Acceptance: parser, boundary, step, persistent-config, and strict Clippy tests pass.

## Change record

- Implemented calendar-aware schedule parsing, next-run calculation, shell command
  execution, missed-run skipping, persistent job loading, and boundary tests.

## Tips

- Start with minute-level scheduling before widening the cron grammar.
- Keep the next-run calculation pure and heavily tested.

## Progress record

- Implemented minimal five-field parsing, wildcard/step fields, calendar-aware
  next-run calculation, shell command execution, and focused boundary tests.
- Remaining: graceful shutdown/missed-run policy and persistent job definitions.
