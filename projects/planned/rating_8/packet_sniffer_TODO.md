# TODO: packet_sniffer (⭐ 8/10)

## Usage

```bash
cargo run --bin packet_sniffer
cargo test --bin packet_sniffer
```

## Milestones

- [ ] Capture packets from an interface or read from pcap file.
- [ ] Parse Ethernet, IPv4, TCP, and UDP headers with safe bounds checks.
- [ ] Implement filter expressions for protocol, host, and port.
- [ ] Add stream-like aggregation for top talkers and protocol mix.
- [ ] Add pretty-print and JSON output modes.
- [ ] Add tests for parser correctness and malformed packet handling.

## Extra

- [ ] Add basic flow reassembly for TCP sessions.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
