# TODO: word_frequency_counter (⭐ 2/10)

## Usage

```bash
cargo run --bin word_frequency_counter -- file.txt
cargo run --bin word_frequency_counter -- file.txt --top 10
cargo test --bin word_frequency_counter
```

## Milestones

- [ ] Read text file and split into words.
- [ ] Count occurrences of each word (case-insensitive).
- [ ] Sort words by frequency.
- [ ] Display top-N words with counts.
- [ ] Add tests for counting and sorting logic.

## Extra

- [ ] Add filtering for stop words (the, a, an, etc).
- [ ] Output as CSV or JSON.

## Tips

- Use HashMap for frequency tracking.
- Normalize words (lowercase, strip punctuation).
- Test with fixture files.
