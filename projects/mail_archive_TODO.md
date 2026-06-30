# TODO: mail_archive (⭐ 8/10)

## Usage

```bash
cargo run --bin mail_archive
cargo test --bin mail_archive
```

## Milestones

- [ ] Implement MIME-aware message parsing and metadata extraction.
- [ ] Add archival storage for raw messages and normalized fields.
- [ ] Implement full-text indexing over bodies and headers.
- [ ] Add retention policies and legal-hold style pinning support.
- [ ] Implement query APIs by sender, subject, date, and content.
- [ ] Add tests for MIME parsing, indexing, and retention edge cases.

## Extra

- [ ] Add duplicate-message detection by message-id and content hash.

## Tips

- Preserve raw message bytes even if normalized parsing evolves later.
- MIME parsing and search indexing should be independent modules.
- Retention logic changes system correctness, not just storage cost.
- Realistic malformed email fixtures will catch parser assumptions early.
