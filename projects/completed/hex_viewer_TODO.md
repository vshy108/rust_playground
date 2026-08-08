# TODO: hex_viewer (⭐ 3/10)

## Usage

```bash
cargo run --bin hex_viewer
cargo test --bin hex_viewer
```

## Milestones

- [x] Read binary input from file or stdin.
- [x] Render offset, hex bytes, and ASCII columns.
- [x] Add configurable bytes-per-row output.
- [x] Handle short final rows correctly.
- [x] Add tests for layout formatting on binary fixtures.

## Extra

- [x] Add colored byte-category rendering.

## Status

Completed

## Specification

### Goal

Render binary input as deterministic offset, hexadecimal, and ASCII columns.

### Non-goals

- Editing binary data
- Disassembly or file-format detection
- Automatic paging

### Inputs and outputs

- Input: optional file/stdin, `--bytes-per-row`, and `--color`
- Output: stable hex-dump rows with short final rows padded

### Errors and limits

- Reject invalid row widths and unreadable input.
- Render non-printable bytes as dots in ASCII output.

### Acceptance criteria

- [x] File/stdin input and layout rendering work.
- [x] Row width and short-row handling work.
- [x] Optional color rendering and layout tests work.

## Change record

- Scope: implemented binary input, hex/ASCII layout, row widths, and color output.
- Tests added: offsets, short rows, and invalid width coverage.
- Commands run: focused `rustfmt`, `cargo check`, `cargo test`, and `cargo clippy`.

## Tips

- Layout math should be isolated from file I/O.
- Binary fixtures make formatting regressions obvious.
