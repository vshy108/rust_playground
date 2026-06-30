# TODO: code_sandbox (⭐ 9/10)

## Usage

```bash
cargo run --bin code_sandbox
cargo test --bin code_sandbox
```

## Milestones

- [ ] Model sandbox policy for filesystem, network, and process limits.
- [ ] Implement isolated execution backend abstraction.
- [ ] Add resource accounting for CPU, memory, and wall-clock time.
- [ ] Implement input/output capture and redaction boundaries.
- [ ] Add per-run audit trail with exit reason classification.
- [ ] Add tests for policy enforcement, timeout handling, and output capture behavior.

## Extra

- [ ] Add snapshot or warm-start support for repeated workloads.

## Tips

- Policy modeling should be declarative and independent from backend choice.
- Timeout, OOM, and syscall-denied exits need different failure reporting.
- Captured output can become a security surface; define truncation rules early.
- Start with a narrow backend contract before optimizing isolation tech.
