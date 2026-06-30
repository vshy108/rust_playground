# TODO: container_runtime (⭐ 9/10)

## Usage

```bash
cargo run --bin container_runtime
cargo test --bin container_runtime
```

## Milestones

- [ ] Build a CLI that describes rootfs, command, and resource limits.
- [ ] Add process spawning with isolated mount, PID, UTS, and network namespaces.
- [ ] Add rootfs pivot/bind mount setup for a contained filesystem view.
- [ ] Add cgroup and rlimit enforcement for memory, CPU, and PID limits.
- [ ] Add syscall and capability restriction hooks.
- [ ] Add tests for config validation and non-Linux feature gating.

## Extra

- [ ] Add OCI bundle import/export compatibility checks.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
