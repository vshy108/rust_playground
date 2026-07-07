# TODO: invaders (⭐ 6/10)

## Status

In Progress


## Usage

```bash
cargo run --bin invaders
cargo test --bin invaders
```

## Goal

Build a playable terminal Space Invaders clone with keyboard input, audio cues,
and clean terminal rendering.

## Milestones

- [x] Set up terminal alternate-screen rendering and raw mode input.
- [x] Implement player movement and bounded shooting.
- [x] Implement shot movement and enemy collision handling.
- [x] Implement invader movement, direction changes, and downward progression.
- [x] Add win/lose conditions and audio feedback.
- [x] Reduce render backlog by using a bounded render channel.
- [x] Keep frame drawing on the queued writer to avoid terminal desync.
- [ ] Add focused unit tests for core game logic (`kill_invader_at`, shot cleanup, bottom reach detection).
- [ ] Add a repeatable manual smoke-check section for gameplay, quit path, win path, and lose path.
- [ ] Run and document a focused validation command for the bin.

## Extra

- [ ] Add scoring and difficulty progression.
- [ ] Add enemy projectiles or multiple wave patterns.

## Tips

- Keep gameplay logic separable from terminal I/O so it can be tested directly.
- Prefer deterministic helpers for movement/collision rules before adding more effects.
- Treat terminal setup/cleanup as a contract: alternate screen, cursor visibility, and raw mode should always be restored.
- Validate both normal exit and early quit behavior because terminal apps fail badly when cleanup is skipped.
