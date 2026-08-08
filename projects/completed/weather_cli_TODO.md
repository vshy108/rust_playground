# TODO: weather_cli (⭐ 3/10)

## Usage

```bash
cargo run --bin weather_cli
cargo test --bin weather_cli
```

## Milestones

- [x] Parse location and unit preferences from CLI arguments.
- [x] Add forecast response model and output formatting.
- [x] Implement current-condition and short forecast views.
- [x] Add graceful handling for missing or malformed responses.
- [x] Add tests for argument parsing and display formatting.

## Extra

- [x] Add ASCII/Unicode icons and compact daily summaries.

## Status

Completed

## Specification

### Goal

Fetch current conditions and a short forecast for a latitude/longitude location.

### Non-goals

- API-key management or historical weather
- Network-dependent tests
- Long-range forecast presentation

### Inputs and outputs

- Input: LAT,LON, optional Fahrenheit units, and optional compact output
- Output: readable current conditions and a three-day forecast

### Errors and limits

- Validate coordinate ranges and report request or malformed-response errors.
- Keep network fetching separate from deterministic formatting.

### Acceptance criteria

- [x] Location/unit parsing and validation work.
- [x] Current and forecast response models work.
- [x] Human and compact/icon output work.
- [x] Fixture-based formatting tests pass.

## Change record

- Scope: implemented Open-Meteo fetching, response modeling, formatting, units,
  compact summaries, and icons.
- Tests added: argument parsing and fixture-based display tests.
- Commands run: focused rustfmt, cargo check, cargo test, and cargo clippy.

## Tips

- Keep fetch/parsing separate from presentation logic.
- Use fixtures for forecast data instead of network-dependent tests.
- Output layout should stay readable on narrow terminals.
