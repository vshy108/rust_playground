# TODO: wasm_runtime (⭐ 9/10)

## Usage

```bash
cargo run --bin wasm_runtime
cargo test --bin wasm_runtime
```

## Milestones

- [ ] Parse a minimal WebAssembly binary module format.
- [ ] Validate section layout, types, and function signatures.
- [ ] Implement a stack machine for numeric ops and local variables.
- [ ] Add memory support with bounds-checked loads and stores.
- [ ] Add host function imports for a tiny embedding API.
- [ ] Add tests for decoding, validation failures, and execution results.

## Extra

- [ ] Add WASI-style experiments for stdout and filesystem shims.

## Tips

- Freeze a tiny grammar subset first and ship that end-to-end.
- Keep phases isolated: tokenize, parse, validate, execute/render.
- Add golden tests for AST/IR snapshots to catch accidental regressions.
- Prefer precise parse errors with position info before adding new syntax.
- Build tooling hooks early (debug dump, trace mode, disassemble output).
