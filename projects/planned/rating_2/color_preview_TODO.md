# TODO: color_preview (⭐ 2/10)

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
