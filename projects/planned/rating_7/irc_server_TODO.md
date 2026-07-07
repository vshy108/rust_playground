# TODO: irc_server (⭐ 7/10)

## Usage

```bash
cargo run --bin irc_server
cargo test --bin irc_server
```

## Milestones

- [ ] Implement line-based IRC command parser and connection lifecycle.
- [ ] Add NICK/USER registration and capability checks.
- [ ] Implement channels, joins, parts, and topic state.
- [ ] Add private messaging and broadcast fanout.
- [ ] Handle disconnect cleanup and nickname collisions.
- [ ] Add tests for registration order, channel fanout, and malformed commands.

## Extra

- [ ] Add history replay or IRCv3-style tags.

## Tips

- IRC is stateful; model registration and joined-channel state explicitly.
- Line framing and max message lengths should be enforced early.
- Cleanup paths after disconnects are as important as happy-path messaging.
- Start with a single-process server before clustering concerns.
