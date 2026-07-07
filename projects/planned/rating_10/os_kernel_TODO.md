# TODO: os_kernel (⭐ 10/10)

## Usage

```bash
cargo run --bin os_kernel
cargo test --bin os_kernel
```

## Milestones

- [ ] Build a freestanding `no_std` entry path and boot sequence.
- [ ] Add VGA or serial output for early debugging.
- [ ] Add interrupt and exception descriptor setup.
- [ ] Add paging and a minimal physical/virtual memory manager.
- [ ] Add heap allocation and a tiny task executor.
- [ ] Add tests for low-level components using QEMU-compatible harnesses.

## Extra

- [ ] Add userspace process loading or a tiny syscall interface.

## Tips

- Lock down data invariants first and encode them in tests.
- Implement persistence boundaries early (snapshot, log, recovery) even if minimal.
- Separate correctness path from optimization path; optimize only after passing invariants.
- Add deterministic simulation tests for retries, crashes, and restart behavior.
- Track state transitions with trace logs to simplify post-failure analysis.
