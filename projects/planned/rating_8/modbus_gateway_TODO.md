# TODO: modbus_gateway (⭐ 8/10)

## Usage

```bash
cargo run --bin modbus_gateway
cargo test --bin modbus_gateway
```

## Milestones

- [ ] Implement Modbus frame parsing and function-code dispatch.
- [ ] Add register map abstraction for coils and holding/input registers.
- [ ] Implement TCP or RTU bridge behavior and request routing.
- [ ] Add polling, timeout, and retry orchestration for downstream devices.
- [ ] Implement metrics and error reporting for device health.
- [ ] Add tests for frame correctness, register access rules, and retry behavior.

## Extra

- [ ] Add protocol translation to HTTP or MQTT.

## Tips

- Register semantics should be explicit and type-safe where possible.
- Wire parsing must be isolated from device orchestration.
- Timeouts and retries are central system behavior in industrial protocols.
- A simulated device backend will pay off early.
