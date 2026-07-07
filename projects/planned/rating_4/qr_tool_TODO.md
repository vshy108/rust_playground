# TODO: qr_tool (⭐ 4/10)

## Usage

```bash
cargo run --bin qr_tool
cargo test --bin qr_tool
```

## Milestones

- [ ] Parse text input and mode flags from the CLI.
- [ ] Generate terminal-friendly QR output for simple text.
- [ ] Add image export or file output mode.
- [ ] Implement decode mode for existing QR images or fixtures.
- [ ] Add tests for round-trip encode/decode behavior.

## Extra

- [ ] Add error-correction level selection.

## Tips

- Treat encode and decode as separate code paths.
- Terminal rendering should be deterministic for snapshot tests.
- Fixtures help more than manual scanning for regressions.
