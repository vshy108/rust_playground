# TODO: color_preview (⭐ 2/10)

## Status

Completed

## Specification

### Goal

Parse common color inputs and render terminal previews with readable color
summaries.

### Non-goals

- Image or GUI rendering
- Color-profile conversion
- Network access or external palette sources

### Inputs and outputs

- Input: hex, `r,g,b`, `rgb(r,g,b)`, named colors, `--palette`, or `--ansi CODE`
- Output: ANSI swatches with uppercase hex and RGB summaries

### Errors and limits

- Reject malformed hex, out-of-range RGB values, unknown colors, and ANSI codes
  outside 0–255.
- Keep parsing independent from terminal rendering.

### Acceptance criteria

- [x] Hex, RGB, and named colors parse correctly.
- [x] Multiple colors, named palette, and ANSI-256 lookup modes work.
- [x] ANSI true-color swatches and summaries render predictably.
- [x] Invalid color input returns clear errors.
- [x] Parsing, rendering, and ANSI lookup tests pass.

## Usage

```bash
cargo run --bin color_preview
cargo test --bin color_preview
```

## Milestones

- [x] Parse color input as hex or RGB values.
- [x] Render terminal color swatches and value summaries.
- [x] Add named-palette or multiple-color preview mode.
- [x] Validate malformed color input clearly.
- [x] Add tests for parsing and formatting helpers.

## Extra

- [x] Add ANSI 256-color lookup mode.

## Tips

- Parsing and terminal rendering should stay independent.
- Most correctness lives in input normalization and display text.

## Change record

- Scope: verified the existing color preview implementation and moved it to the
  completed-projects path.
- Assumptions: ANSI escape sequences are the intended terminal output format.
- Tests added: no new tests; existing tests cover parsing, validation, rendering,
  and ANSI-256 cube/grayscale lookup.
- Commands run: `rustfmt projects/completed/color_preview.rs`, `cargo test --bin
  color_preview`, and `cargo clippy --bin color_preview --all-features -- -D
  warnings`.
- Known limitations: terminal color support depends on the user's terminal.
- Follow-up: begin the next unfinished rating-2 project, `wc_clone`.
