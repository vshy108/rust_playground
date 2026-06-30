# TODO: text_editor (⭐ 7/10)

## Usage

```bash
cargo run --bin textedit -- README.md
cargo test --bin textedit
```

## Milestones

- [ ] Open a file into line buffer and render viewport.
- [ ] Implement cursor movement with bounds safety.
- [ ] Implement insert/delete/newline editing operations.
- [ ] Implement save and dirty-state tracking.
- [ ] Implement search mode with match navigation.
- [ ] Add tests for buffer edit operations.

## Extra

- [ ] Add syntax highlighting for Rust.

## Tips

- Build the frame/update loop first, then add features one subsystem at a time.
- Keep input, simulation, and rendering separated for easier testing.
- Add deterministic replay fixtures so behavior can be reproduced exactly.
- Gate expensive rendering or effects behind flags while debugging logic.
- Add smoke tests for startup, shutdown, and basic interaction paths.
