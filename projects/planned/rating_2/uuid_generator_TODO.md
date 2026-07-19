# TODO: uuid_generator (⭐ 2/10)

## Usage

```bash
cargo run --bin uuid_generator
cargo test --bin uuid_generator
```

## Milestones

- [x] Generate UUIDs from a simple CLI.
- [x] Add count or batch generation mode.
- [x] Support a couple of output formats such as plain or uppercase.
- [x] Validate output shape in tests.
- [x] Add tests for formatting and count behavior.

## Extra

- [x] Add namespace-based deterministic UUID mode.

## Tips

- Output formatting is the main behavior to pin down.
- Keep generation logic and CLI formatting separate.
