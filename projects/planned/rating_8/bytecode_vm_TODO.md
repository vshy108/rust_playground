# TODO: bytecode_vm (⭐ 8/10)

## Usage

```bash
cargo run --bin bytecode_vm
cargo test --bin bytecode_vm
```

## Milestones

- [ ] Define bytecode instruction set and binary encoding format.
- [ ] Implement VM core with stack, frames, and instruction pointer.
- [ ] Add arithmetic, branching, and function call instructions.
- [ ] Implement local/global variable slots and constant pool.
- [ ] Add disassembler output for debugging bytecode streams.
- [ ] Add tests for interpreter semantics and stack safety.

## Extra

- [ ] Add optional tracing mode with per-op execution timing.

## Tips

- Freeze a tiny grammar subset first and ship that end-to-end.
- Keep phases isolated: tokenize, parse, validate, execute/render.
- Add golden tests for AST/IR snapshots to catch accidental regressions.
- Prefer precise parse errors with position info before adding new syntax.
- Build tooling hooks early (debug dump, trace mode, disassemble output).
