# TODO: terminal_multiplexer (⭐ 8/10)

## Usage

```bash
cargo run --bin terminal_multiplexer
cargo test --bin terminal_multiplexer
```

## Milestones

- [ ] Implement session, window, and pane abstractions.
- [ ] Add PTY process management and pane attachment lifecycle.
- [ ] Implement split layout calculation and resize propagation.
- [ ] Add input routing and pane focus semantics.
- [ ] Implement detach/reattach and session persistence basics.
- [ ] Add tests for layout math, pane lifecycle, and focus switching behavior.

## Extra

- [ ] Add copy mode or scrollback search support.

## Tips

- PTY management and layout computation should be separate modules.
- Focus rules need explicit tests because they drift easily.
- Session restoration can start small with structural metadata only.
- A fake terminal backend helps validate layout logic without real PTYs.
