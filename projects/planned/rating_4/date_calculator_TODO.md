# TODO: date_calculator (⭐ 4/10)

## Usage

```bash
cargo run --bin date_calculator
cargo test --bin date_calculator
```

## Milestones

- [ ] Parse a base date from CLI input.
- [ ] Add or subtract day-based durations.
- [ ] Print the resulting date in a stable format.
- [ ] Support simple date-difference calculations.
- [ ] Add tests for month boundaries and leap years.

## Extra

- [ ] Add human-friendly duration parsing like `+2w` or `-3d`.

## Tips

- Keep date math in a small helper layer instead of mixing it into CLI parsing.
- Boundary tests matter more than broad feature count for this project.
