# TODO: bittorrent (⭐ 9/10)

## Usage

```bash
cargo run --bin bittorrent
cargo test --bin bittorrent
```

## Milestones

- [ ] Parse `.torrent` files (bencode decoder).
- [ ] Parse tracker response and peer list.
- [ ] Implement peer handshake + bitfield exchange.
- [ ] Implement piece request/download/verify flow.
- [ ] Add rarest-first piece selection strategy.
- [ ] Add tests for bencode codec and piece verifier.

## Extra

- [ ] Add DHT bootstrap support.

## Tips

- Implement one milestone at a time and keep each slice testable.
- Add a failing test first, then implement the smallest behavior to pass.
- Keep CLI/API surface stable while iterating internals.
- Validate both happy path and error path for every milestone.
- Run focused tests before broad checks.
