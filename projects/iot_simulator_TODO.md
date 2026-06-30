# TODO: iot_simulator (⭐ 9/10)

## Usage

```bash
cargo run --bin iot_simulator
cargo test --bin iot_simulator
```

## Milestones

- [ ] Model virtual devices with sensor and actuator traits.
- [ ] Build time-step simulation loop with deterministic scheduling.
- [ ] Simulate telemetry publishing and command ingestion.
- [ ] Add fault injection (packet loss, delay, sensor drift).
- [ ] Add scenario files for reproducible test runs.
- [ ] Add tests for timing behavior and state transitions.

## Extra

- [ ] Add MQTT bridge mode for hardware-in-the-loop experiments.

## Tips

- Start with a deterministic local fixture path before external integration.
- Add bounded concurrency controls early to prevent overload and flakiness.
- Separate collection from aggregation/output so each can be tested in isolation.
- Add backoff and retry policy tests for transient failures.
- Measure throughput and tail latency on representative input sizes.
