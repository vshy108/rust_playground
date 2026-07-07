# TODO: tcp_stack (⭐ 7/10)

## Usage

```bash
cargo run --bin tcp_stack
cargo test --bin tcp_stack
```

## Milestones

- [ ] Parse and encode Ethernet, ARP, IPv4, and ICMP packet headers.
- [ ] Implement a virtual NIC loop for feeding frames into the stack.
- [ ] Add TCP handshake state machine (SYN, SYN-ACK, ACK).
- [ ] Build a minimal socket-like API for send/receive streams.
- [ ] Add retransmission timer and basic congestion behavior.
- [ ] Add tests for parser correctness and handshake transitions.

## Extra

- [ ] Add pcap input/output support for offline replay.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
