# TODO: archive_extractor (⭐ 4/10)

## Usage

```bash
cargo run --bin archive_extractor
cargo test --bin archive_extractor
```

## Milestones

- [x] Detect and parse a simple tar archive.
- [x] Extract files into a target directory.
- [x] Add listing mode without extraction.
- [x] Prevent path traversal from malicious archive entries.
- [x] Add tests for extraction layout and invalid-entry handling.

## Extra

- [ ] Add support for compressed tar variants.

## Progress record

- Implemented uncompressed tar parsing, listing, extraction, and safe-path validation.
- Remaining: compressed tar variants.

## Tips

- Archive entry validation is part of correctness.
- Listing and extraction should share parsing but not mutation code.
