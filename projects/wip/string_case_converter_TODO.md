# TODO: string_case_converter (⭐ 2/10)

## Usage

```bash
cargo run --bin string_case_converter -- "hello world" to-snake
cargo run --bin string_case_converter -- "HelloWorld" to-kebab
cargo test --bin string_case_converter
```

## Milestones

- [ ] Parse command-line arguments (input text, target case).
- [ ] Implement to_snake_case, to_kebab_case, to_pascal_case.
- [ ] Handle edge cases (numbers, special chars, consecutive capitals).
- [ ] Add tests for each case conversion.
- [ ] Output formatted result to stdout.

## Extra

- [ ] Add to_camelCase variant.
- [ ] Handle Unicode characters gracefully.

## Tips

- Use iterator chains for character processing.
- Test with mixed-case and punctuation inputs.
