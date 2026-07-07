# TODO: gitignore_gen (⭐ 2/10)

## Usage

```bash
cargo run --bin gitignore_gen
cargo test --bin gitignore_gen
```

## Milestones

- [ ] Parse one or more project-type presets from CLI input.
- [ ] Combine matching ignore patterns into output text.
- [ ] Remove duplicates while preserving useful ordering.
- [ ] Add overwrite or print-to-stdout modes.
- [ ] Add tests for preset composition and ordering.

## Extra

- [ ] Add custom pattern merge support.

## Tips

- Preset data and rendering logic should stay separate.
- Ordering matters because humans will read the output.
