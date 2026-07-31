# TODO: file_concatenator (⭐ 1/10)

## Usage

```bash
cargo run --bin file_concatenator -- file1.txt file2.txt file3.txt
cargo run --bin file_concatenator -- file1.txt file2.txt > combined.txt
cargo test --bin file_concatenator
```

## Milestones

- [ ] Accept multiple file paths as arguments.
- [ ] Read and concatenate file contents.
- [ ] Output combined result to stdout.
- [ ] Handle missing files gracefully.
- [ ] Add tests for concatenation.

## Extra

- [ ] Add --with-filenames flag to prepend filename headers.
- [ ] Add line numbering option.

## Tips

- Use clap or simple args parsing.
- Handle file read errors with Result.
- Keep file operations separate from output logic.
