# TODO: base64_cli (⭐ 2/10)

## Status

Completed

## Specification

### Goal

Encode and decode text, files, or standard input using standard or URL-safe
Base64.

### Non-goals

- Streaming output beyond basic stdin/stdout support
- Encryption, compression, or automatic character-set conversion
- Network access

### Inputs and outputs

- Input: `encode` or `decode`, optional `--url-safe`, optional `--file PATH`,
  and text input or standard input
- Output: encoded text or decoded raw bytes on standard output

### Errors and limits

- Reject unknown modes and options, conflicting input sources, malformed Base64,
  invalid padding, and impossible input lengths.
- Preserve binary bytes during file and stdin operations.

### Acceptance criteria

- [x] Standard Base64 encoding and decoding work.
- [x] URL-safe encoding and unpadded decoding work.
- [x] File and standard-input paths preserve raw bytes.
- [x] Invalid input returns a clear error.
- [x] Round-trip and failure-path tests pass.

## Usage

```bash
cargo run --bin base64_cli
cargo test --bin base64_cli
```

## Milestones

- [x] Encode text or file input to Base64.
- [x] Add decode mode back to raw bytes or text.
- [x] Handle invalid Base64 input gracefully.
- [x] Support stdin/stdout streaming basics.
- [x] Add tests for encode/decode round-trips.

## Extra

- [x] Add URL-safe Base64 mode.

## Tips

- Keep encode and decode paths separate.
- Round-trip fixtures will catch most mistakes here.

## Change record

- Scope: verified the existing Base64 implementation and moved it to the
  completed-projects path.
- Assumptions: standard output is the interface for both text and binary data.
- Tests added: no new tests; existing tests cover known encoding, binary
  round-trips, URL-safe mode, unpadded input, and invalid input.
- Commands run: `rustfmt projects/completed/base64_cli.rs`, `cargo test --bin
  base64_cli`, and `cargo clippy --bin base64_cli --all-features -- -D
  warnings`.
- Known limitations: no streaming chunk-size controls or MIME line wrapping.
- Follow-up: begin the next unfinished rating-2 project, `uuid_generator`.
