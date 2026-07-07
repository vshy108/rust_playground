# TODO: archive_extractor (⭐ 4/10)

## Usage

```bash
cargo run --bin archive_extractor
cargo test --bin archive_extractor
```

## Milestones

- [ ] Detect a simple archive type such as zip or tar.
- [ ] Extract files into a target directory.
- [ ] Add listing mode without extraction.
- [ ] Prevent path traversal from malicious archive entries.
- [ ] Add tests for extraction layout and invalid-entry handling.

## Extra

- [ ] Add support for compressed tar variants.

## Tips

- Archive entry validation is part of correctness.
- Listing and extraction should share parsing but not mutation code.
