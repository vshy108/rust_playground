# TODO: unit_converter (⭐ 3/10)

## Status

Completed

## Specification

### Goal

Convert values across supported length, mass, temperature, and speed units.

### Non-goals

- Arbitrary unit definitions or dimensional algebra
- Locale-specific number parsing
- Network or configuration-file input

### Inputs and outputs

- Input: numeric value, source unit, and target unit
- Output: normalized numeric result and target unit

### Errors and limits

- Reject invalid numbers, unsupported units, and incompatible categories.
- Keep precision formatting deterministic.

### Acceptance criteria

- [x] Length, mass, temperature, and compound speed conversions work.
- [x] Incompatible and unsupported units return errors.
- [x] Output precision is normalized.
- [x] Conversion tests pass.

## Usage

```bash
cargo run --bin unit_converter
cargo test --bin unit_converter
```

## Milestones

- [x] Parse value, source unit, and target unit from CLI input.
- [x] Implement a small conversion table for length, weight, or temperature.
- [x] Add error handling for unsupported or incompatible units.
- [x] Format output clearly with normalized precision.
- [x] Add tests for conversion correctness and invalid input cases.

## Extra

- [x] Add compound units like km/h.

## Tips

- Keep unit parsing separate from conversion math.
- Start with one category before mixing unrelated dimensions.
- Precision rules should be explicit so tests stay stable.

## Change record

- Scope: verified and moved the completed converter to `projects/completed/`.
- Tests added: existing tests cover conversions, compound speed, errors, and formatting.
- Commands run: focused `rustfmt`, `cargo check`, `cargo test`, and `cargo clippy`.
- Follow-up: continue with the next incomplete rating-3 project.
