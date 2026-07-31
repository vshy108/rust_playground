# TODO: simple_todo_app (⭐ 3/10)

## Usage

```bash
cargo run --bin simple_todo_app -- add "Buy groceries"
cargo run --bin simple_todo_app -- list
cargo run --bin simple_todo_app -- complete 1
cargo test --bin simple_todo_app
```

## Milestones

- [ ] Parse commands (add, list, complete, remove, save).
- [ ] Store todos in memory (Vec<Task>).
- [ ] Add CLI for task management.
- [ ] Persist todos to JSON file on save.
- [ ] Load todos from file at startup.
- [ ] Add tests for task operations.

## Extra

- [ ] Add priority levels.
- [ ] Add due dates.

## Tips

- Use serde_json for persistence.
- Define a simple Task struct.
- Test file I/O separately from business logic.
