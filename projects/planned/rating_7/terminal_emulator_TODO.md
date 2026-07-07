# TODO: terminal_emulator (⭐ 7/10)

## Usage

```bash
cargo run --bin terminal_emulator
cargo test --bin terminal_emulator
```

## Milestones

- [ ] Implement a screen buffer with cursor, scroll region, and attributes.
- [ ] Parse ANSI escape sequences for movement, color, and clear commands.
- [ ] Add PTY-backed subprocess integration.
- [ ] Handle line wrapping, scrolling, and alternate screen behavior.
- [ ] Add keyboard input translation for control and arrow keys.
- [ ] Add tests for escape sequence parsing and buffer transitions.

## Extra

- [ ] Add mouse reporting and bracketed paste support.

## Tips

- Build the frame/update loop first, then add features one subsystem at a time.
- Keep input, simulation, and rendering separated for easier testing.
- Add deterministic replay fixtures so behavior can be reproduced exactly.
- Gate expensive rendering or effects behind flags while debugging logic.
- Add smoke tests for startup, shutdown, and basic interaction paths.
