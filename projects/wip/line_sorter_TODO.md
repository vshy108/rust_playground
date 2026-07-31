# TODO: line_sorter (⭐ 1/10)

## Usage

```bash
cargo run --bin line_sorter -- file.txt
cargo run --bin line_sorter -- file.txt --reverse
cargo run --bin line_sorter -- file.txt --unique
cargo test --bin line_sorter
```

## Milestones

- [ ] Read lines from input file.
- [ ] Sort lines alphabetically.
- [ ] Add --reverse flag for descending order.
- [ ] Add --unique flag to remove duplicates.
- [ ] Output sorted lines to stdout.
- [ ] Add tests for sort and filter logic.

## Extra

- [ ] Add --numeric flag for numeric sorting.
- [ ] Add case-insensitive sorting option.

## Tips

- Use Vec<String> to collect lines.
- Use `.sort()` and `.dedup()` methods.
- Handle both file and stdin input.
