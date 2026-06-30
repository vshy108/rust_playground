# TODO: smtp_server (⭐ 8/10)

## Usage

```bash
cargo run --bin smtp_server
cargo test --bin smtp_server
```

## Milestones

- [ ] Implement SMTP session state machine and command parser.
- [ ] Add envelope handling for HELO/EHLO, MAIL FROM, RCPT TO, and DATA.
- [ ] Implement message storage queue and retry scheduling.
- [ ] Add basic anti-abuse checks and size limits.
- [ ] Add pluggable local delivery or mailbox sink.
- [ ] Add tests for protocol sequencing, malformed input, and queue retries.

## Extra

- [ ] Add STARTTLS and authentication support.

## Tips

- Keep protocol parsing separate from delivery logic.
- Make session state transitions explicit; SMTP ordering bugs are easy to hide.
- Store raw messages plus parsed envelope metadata for debugging.
- Start with deterministic local-delivery tests before network hardening.
