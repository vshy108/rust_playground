# TODO: build_system (⭐ 7/10)

## Usage

```bash
cargo run --bin build_system
cargo test --bin build_system
```

## Milestones

- [ ] Parse build targets, inputs, outputs, and command rules.
- [ ] Build a dependency DAG and detect cycles.
- [ ] Implement topological execution with parallel workers.
- [ ] Add file hash-based incremental rebuild decisions.
- [ ] Implement local cache for reusable target artifacts.
- [ ] Add tests for cycle detection, cache hits, and invalidation.

## Extra

- [ ] Add remote cache protocol and execution sandboxing.

## Tips

- Keep graph construction pure and independent from execution side effects.
- Use content hashes instead of mtimes to avoid flaky rebuild logic.
- Capture command stdout/stderr for deterministic diagnostics.
- Add fixture projects with known dependency trees.
