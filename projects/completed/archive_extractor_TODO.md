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

- [x] Add support for gzip-compressed tar variants.

## Progress record

- Implemented uncompressed tar parsing, listing, extraction, safe-path validation,
  and gzip decoding for tar.gz input.

## Status

Completed.

## Specification

- Goal: list and extract safe tar and tar.gz archives.
- Inputs: list ARCHIVE or extract ARCHIVE DIRECTORY.
- Output: deterministic entry listing or extracted files/directories.
- Safety: reject absolute paths, parent traversal, malformed headers, truncated
  entries, and invalid UTF-8 metadata.
- Non-goals: archive creation, ZIP support, and preserving executable metadata.
- Acceptance: tar parsing, gzip decoding, extraction, listing, and traversal
  tests pass with strict Clippy enabled.

## Change record

- Added gzip tar input support and completed the archive extractor specification.

## Tips

- Archive entry validation is part of correctness.
- Listing and extraction should share parsing but not mutation code.
