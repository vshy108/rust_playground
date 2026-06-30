# TODO: unit_converter (⭐ 3/10)

## Usage

```bash
cargo run --bin unit_converter
cargo test --bin unit_converter
```

## Milestones

- [ ] Parse value, source unit, and target unit from CLI input.
- [ ] Implement a small conversion table for length, weight, or temperature.
- [ ] Add error handling for unsupported or incompatible units.
- [ ] Format output clearly with normalized precision.
- [ ] Add tests for conversion correctness and invalid input cases.

## Extra

- [ ] Add compound units like km/h.

## Tips

- Keep unit parsing separate from conversion math.
- Start with one category before mixing unrelated dimensions.
- Precision rules should be explicit so tests stay stable.
