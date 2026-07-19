# TODO: gitignore_gen (⭐ 2/10)

## Usage

```bash
cargo run --bin gitignore_gen
cargo test --bin gitignore_gen
```

## Milestones

- [x] Parse one or more project-type presets from CLI input.
- [x] Combine matching ignore patterns into output text.
- [x] Remove duplicates while preserving useful ordering.
- [x] Add overwrite or print-to-stdout modes.
- [x] Add tests for preset composition and ordering.

## Extra

- [x] Add custom pattern merge support.

## Tips

- Preset data and rendering logic should stay separate.
- Ordering matters because humans will read the output.
