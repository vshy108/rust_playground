# TODO: fuzzer_engine (⭐ 9/10)

## Usage

```bash
cargo run --bin fuzzer_engine
cargo test --bin fuzzer_engine
```

## Milestones

- [ ] Implement input corpus storage and mutation strategies.
- [ ] Add target execution harness with crash and timeout detection.
- [ ] Implement coverage or feedback signal collection abstraction.
- [ ] Add queue scheduling and interesting-input promotion rules.
- [ ] Implement crash minimization and reproducibility output.
- [ ] Add tests for mutation determinism, crash capture, and corpus promotion behavior.

## Extra

- [ ] Add distributed workers for parallel fuzzing.

## Tips

- Corpus management and target execution should remain isolated.
- Feedback quality determines fuzzer usefulness more than mutation count.
- Reproducible crashes are a product requirement, not a debug convenience.
- Start with a synthetic instrumented target before real binaries.
