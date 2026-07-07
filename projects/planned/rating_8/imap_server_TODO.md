# TODO: imap_server (⭐ 8/10)

## Usage

```bash
cargo run --bin imap_server
cargo test --bin imap_server
```

## Milestones

- [ ] Implement mailbox, message UID, and session state model.
- [ ] Add IMAP command parsing for login, select, fetch, search, and store flows.
- [ ] Implement message flags and mailbox metadata tracking.
- [ ] Add incremental synchronization semantics for new and changed messages.
- [ ] Implement authentication and concurrent mailbox session handling.
- [ ] Add tests for protocol parsing, UID stability, and flag mutation correctness.

## Extra

- [ ] Add IDLE support for push-style mailbox updates.

## Tips

- UID semantics and mailbox state transitions define most of the protocol complexity.
- Separate parser, command execution, and message storage concerns.
- Start with a small in-memory mailbox model before persistence.
- Protocol transcripts make strong regression fixtures.
