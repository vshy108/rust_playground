# TODO: nes_emulator (⭐ 8/10)

## Usage

```bash
cargo run --bin nes_emulator
cargo test --bin nes_emulator
```

## Milestones

- [ ] Parse iNES ROM headers and map PRG/CHR memory.
- [ ] Implement 6502 CPU registers, addressing modes, and instruction dispatch.
- [ ] Add memory bus wiring between CPU, RAM, and cartridge.
- [ ] Add PPU timing model and basic frame rendering path.
- [ ] Add controller input and simple audio stubs.
- [ ] Add tests for CPU opcodes, ROM loading, and timing-sensitive cases.

## Extra

- [ ] Add mapper support beyond NROM and save-state snapshots.

## Tips

- Build the frame/update loop first, then add features one subsystem at a time.
- Keep input, simulation, and rendering separated for easier testing.
- Add deterministic replay fixtures so behavior can be reproduced exactly.
- Gate expensive rendering or effects behind flags while debugging logic.
- Add smoke tests for startup, shutdown, and basic interaction paths.
