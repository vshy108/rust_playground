# TODO: api_deprecation_tracker (⭐ 5/10)


## Usage

```bash
cargo run --bin api_deprecation_tracker
cargo test --bin api_deprecation_tracker
```

## Milestones

- [ ] Model API versions, deprecation windows, and owners.
- [ ] Track endpoint usage against sunset timelines.
- [ ] Generate notifications for upcoming deadlines.
- [ ] Produce reports for clients at risk.
- [ ] Add tests for window calculations and report output.

## Extra

- [ ] Add policy checks for minimum notice periods.

## Tips

- Keep date/time logic centralized to avoid drift bugs.
- Separate usage ingestion from deprecation policy checks.
