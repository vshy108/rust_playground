# TODO: base64_cli (⭐ 2/10)

## Usage

```bash
cargo run --bin base64_cli
cargo test --bin base64_cli
```

## Milestones

- [ ] Encode text or file input to Base64.
- [ ] Add decode mode back to raw bytes or text.
- [ ] Handle invalid Base64 input gracefully.
- [ ] Support stdin/stdout streaming basics.
- [ ] Add tests for encode/decode round-trips.

## Extra

- [ ] Add URL-safe Base64 mode.

## Tips

- Keep encode and decode paths separate.
- Round-trip fixtures will catch most mistakes here.
