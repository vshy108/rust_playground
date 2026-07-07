# TODO: mini_git (⭐ 5/10)

## Usage

```bash
cargo run --bin mini_git -- init
cargo run --bin mini_git -- status
cargo test --bin mini_git
```

## Milestones

- [ ] Implement `init` and `.mini_git` directory layout.
- [ ] Implement blob hashing + object storage by content address.
- [ ] Implement `add` and index staging format.
- [ ] Implement `commit` objects + parent linkage.
- [ ] Implement `log` and `cat-file` for object inspection.
- [ ] Add snapshot tests over deterministic fixtures.

## Extra

- [ ] Add branch refs and `checkout` (detached HEAD not required first pass).

## Tips

- Implement one milestone at a time and keep each slice testable.
- Add a failing test first, then implement the smallest behavior to pass.
- Keep CLI/API surface stable while iterating internals.
- Validate both happy path and error path for every milestone.
- Run focused tests before broad checks.
