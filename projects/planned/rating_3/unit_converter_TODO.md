# TODO: unit_converter (⭐ 3/10)

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
