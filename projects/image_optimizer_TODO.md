# TODO: image_optimizer (⭐ 6/10)

## Usage

```bash
cargo run --bin image_optimizer
cargo test --bin image_optimizer
```

## Milestones

- [ ] Parse input image formats and metadata.
- [ ] Implement lossless optimization passes for one format first.
- [ ] Add quality/size reporting before and after optimization.
- [ ] Implement batch processing across directory trees.
- [ ] Add overwrite, dry-run, and output-path modes.
- [ ] Add tests for byte savings, visual preservation assumptions, and malformed input handling.

## Extra

- [ ] Add parallel optimization and format-specific tunables.

## Tips

- Keep decoding/encoding separate from optimization passes.
- Start with PNG-like lossless transformations before broader format coverage.
- Dry-run reporting is useful enough to build early.
- Preserve metadata intentionally; stripping it silently is a product choice.
