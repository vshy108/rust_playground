# TODO: mini_shell (⭐ 6/10)

## Usage

```bash
cargo run --bin mini_shell
cargo test --bin mini_shell
```

## Milestones

- [ ] Implement REPL loop with prompt + line reading.
- [ ] Parse command + args + quoted tokens.
- [ ] Implement built-ins: `cd`, `pwd`, `exit`.
- [ ] Implement external command spawn + wait.
- [ ] Implement pipes and redirections.
- [ ] Add tests for parser behavior.

## Extra

- [ ] Add command history and completion.

## Tips

- Implement one milestone at a time and keep each slice testable.
- Add a failing test first, then implement the smallest behavior to pass.
- Keep CLI/API surface stable while iterating internals.
- Validate both happy path and error path for every milestone.
- Run focused tests before broad checks.
