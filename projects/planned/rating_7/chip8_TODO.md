# TODO: chip8 (⭐ 7/10)

## Usage

```bash
cargo run --bin chip8
cargo test --bin chip8
```

## Milestones

- [ ] Implement CHIP-8 memory, registers, stack, and program counter.
- [ ] Decode and execute core opcode set with instruction dispatch.
- [ ] Build timer tick handling for delay and sound timers.
- [ ] Add keypad input model and screen framebuffer updates.
- [ ] Support loading and running ROM files from disk.
- [ ] Add tests for opcode behavior and edge-case flag updates.

## Extra

- [ ] Add Super-CHIP extension instructions and resolution mode.

## Tips

- Build the frame/update loop first, then add features one subsystem at a time.
- Keep input, simulation, and rendering separated for easier testing.
- Add deterministic replay fixtures so behavior can be reproduced exactly.
- Gate expensive rendering or effects behind flags while debugging logic.
- Add smoke tests for startup, shutdown, and basic interaction paths.
