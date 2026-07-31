# TODO: text_statistics (⭐ 2/10)

## Usage

```bash
cargo run --bin text_statistics -- file.txt
cargo run --bin text_statistics -- file.txt --words
cargo test --bin text_statistics
```

## Milestones

- [ ] Read input file or stdin.
- [ ] Count lines, words, and characters.
- [ ] Display statistics in formatted output.
- [ ] Support --lines, --words, --chars flags.
- [ ] Add tests for counting logic.

## Extra

- [ ] Add average word length.
- [ ] Add sentence count and average sentence length.

## Tips

- Use simple iteration for counting.
- Split on whitespace for word count.
- Keep counting logic testable.
