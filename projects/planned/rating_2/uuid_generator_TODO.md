# TODO: uuid_generator (⭐ 2/10)

## Usage

```bash
cargo run --bin uuid_generator
cargo test --bin uuid_generator
```

## Milestones

- [ ] Generate UUIDs from a simple CLI.
- [ ] Add count or batch generation mode.
- [ ] Support a couple of output formats such as plain or uppercase.
- [ ] Validate output shape in tests.
- [ ] Add tests for formatting and count behavior.

## Extra

- [ ] Add namespace-based deterministic UUID mode.

## Tips

- Output formatting is the main behavior to pin down.
- Keep generation logic and CLI formatting separate.
