# TODO: ftp_server (⭐ 7/10)

## Usage

```bash
cargo run --bin ftp_server
cargo test --bin ftp_server
```

## Milestones

- [ ] Implement command parser and control connection state machine.
- [ ] Add authentication and session lifecycle basics.
- [ ] Implement file listing and retrieval commands.
- [ ] Add passive-mode data connection handling.
- [ ] Implement upload, delete, and rename semantics.
- [ ] Add tests for command ordering, data-channel behavior, and permission failures.

## Extra

- [ ] Add TLS support and chroot-like path restrictions.

## Tips

- Control and data channels should be separate abstractions.
- Passive-mode behavior is the main protocol wrinkle; test it directly.
- Path normalization matters to avoid escaping the served root.
- Start with a local fixture directory before user management complexity.
