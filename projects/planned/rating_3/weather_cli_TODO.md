# TODO: weather_cli (⭐ 3/10)

## Usage

```bash
cargo run --bin weather_cli
cargo test --bin weather_cli
```

## Milestones

- [ ] Parse location and unit preferences from CLI arguments.
- [ ] Add forecast response model and output formatting.
- [ ] Implement current-condition and short forecast views.
- [ ] Add graceful handling for missing or malformed responses.
- [ ] Add tests for argument parsing and display formatting.

## Extra

- [ ] Add ASCII icons or compact daily summaries.

## Tips

- Keep fetch/parsing separate from presentation logic.
- Use fixtures for forecast data instead of network-dependent tests.
- Output layout should stay readable on narrow terminals.
