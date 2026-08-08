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

## Dependency plan

- Start when this project implementation begins: `qrcode` for QR matrix
  generation and decoding support; add an image crate only if file export
  cannot remain terminal/text based.
- Reason: QR mode tables, error correction, masking, and decoding are protocol
  work that should rely on a reviewed implementation.
- Verification: run deterministic snapshot tests, round-trip fixtures, strict
  Clippy, and `cargo audit` when the registry is available.
- Status: `qrcode` is available in Cargo.toml; image export remains conditional.

## Tips

- Treat encode and decode as separate code paths.
- Terminal rendering should be deterministic for snapshot tests.
- Fixtures help more than manual scanning for regressions.
