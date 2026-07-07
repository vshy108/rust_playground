# TODO: ebpf_monitor (⭐ 9/10)

## Usage

```bash
cargo run --bin ebpf_monitor
cargo test --bin ebpf_monitor
```

## Milestones

- [ ] Define event model for process, network, and syscall telemetry.
- [ ] Implement userspace collector pipeline and buffering.
- [ ] Add eBPF program loading and attach lifecycle management.
- [ ] Implement event enrichment with process metadata.
- [ ] Add output backends for console and structured JSON.
- [ ] Add tests for parser logic, filtering, and buffer pressure handling.

## Extra

- [ ] Add anomaly detection with pluggable rules.

## Tips

- Start with replayed event fixtures to validate userspace code on any OS.
- Keep kernel-specific paths isolated behind feature flags.
- Design bounded queues so burst traffic does not exhaust memory.
- Add explicit drop counters to observe overload behavior.
