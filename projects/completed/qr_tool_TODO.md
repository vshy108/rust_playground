# TODO: qr_tool (⭐ 4/10)

## Usage

```bash
cargo run --bin qr_tool
cargo test --bin qr_tool
```

## Milestones

- [x] Parse text input and mode flags from the CLI.
- [x] Generate terminal-friendly QR output for simple text.
- [x] Add image export or file output mode.
- [x] Implement decode mode for existing QR images or fixtures.
- [x] Add tests for round-trip encode/decode behavior.

## Extra

- [x] Add error-correction level selection.

## Status

Completed.

## Specification

- Goal: encode text as terminal or PNG QR output and decode PNG fixtures.
- Inputs: text, optional L/M/Q/H error-correction level, and PNG paths.
- Output: deterministic terminal QR, PNG files, or decoded UTF-8 payloads.
- Errors: reject invalid levels, oversized/unencodable data, unreadable images,
  missing QR codes, and non-UTF-8 payloads.
- Non-goals: camera capture, SVG export, and binary payload presentation.
- Acceptance: deterministic rendering, all correction levels, PNG export,
  decode round-trip, and strict Clippy tests pass.

## Change record

- Implemented qrcode/rqrr encoding and decoding, PNG export, terminal rendering,
  error-correction selection, and round-trip fixture coverage.

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
