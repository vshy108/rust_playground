# TODO: text_diff_cli (⭐ 4/10)

## Usage

```bash
cargo run --bin text_diff_cli
cargo test --bin text_diff_cli
```

## Milestones

- [ ] Load two text inputs from files or stdin.
- [ ] Implement line-by-line diff output.
- [ ] Add markers for added, removed, and unchanged lines.
- [ ] Handle newline and empty-file edge cases.
- [ ] Add tests for diff rendering on representative fixtures.

## Extra

- [ ] Add word-level highlighting inside changed lines.

## Tips

- Diff algorithm and output formatting should not be tightly coupled.
- Small fixture pairs give good coverage for this type of tool.
