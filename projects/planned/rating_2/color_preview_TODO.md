# TODO: color_preview (⭐ 2/10)

## Usage

```bash
cargo run --bin color_preview
cargo test --bin color_preview
```

## Milestones

- [ ] Parse color input as hex or RGB values.
- [ ] Render terminal color swatches and value summaries.
- [ ] Add named-palette or multiple-color preview mode.
- [ ] Validate malformed color input clearly.
- [ ] Add tests for parsing and formatting helpers.

## Extra

- [ ] Add ANSI 256-color lookup mode.

## Tips

- Parsing and terminal rendering should stay independent.
- Most correctness lives in input normalization and display text.
