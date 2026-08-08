# TODO: date_calculator (⭐ 4/10)

## Usage

```bash
cargo run --bin date_calculator
cargo test --bin date_calculator
```

## Milestones

- [x] Parse a base date from CLI input.
- [x] Add or subtract day-based durations.
- [x] Print the resulting date in a stable format.
- [x] Support simple date-difference calculations.
- [x] Add tests for month boundaries and leap years.

## Extra

- [x] Add human-friendly duration parsing like `+2w` or `-3d`.

## Status

Completed.

## Specification

- Goal: perform predictable Gregorian date arithmetic without external
  dependencies.
- Inputs: ISO dates in `YYYY-MM-DD`, signed day counts or `d`/`w` durations,
  and an optional `--diff` operation.
- Output: stable ISO dates for arithmetic and signed day counts for differences.
- Errors: reject malformed dates, impossible month/day combinations, invalid
  durations, and invalid argument shapes.
- Non-goals: time zones, times of day, recurring schedules, and locale output.
- Acceptance: month-boundary, leap-year, duration, invalid-input, and difference
  tests pass with strict Clippy enabled.

## Change record

- Implemented the date parser, proleptic Gregorian conversion helpers, CLI
  operations, human-friendly durations, and focused boundary tests.

## Tips

- Keep date math in a small helper layer instead of mixing it into CLI parsing.
- Boundary tests matter more than broad feature count for this project.
