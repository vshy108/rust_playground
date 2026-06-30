# TODO: collaborative_editor (⭐ 9/10)

## Usage

```bash
cargo run --bin collaborative_editor
cargo test --bin collaborative_editor
```

## Milestones

- [ ] Model documents, sessions, and edit operations.
- [ ] Implement single-user text buffer operations first.
- [ ] Add operational transform or CRDT-based merge semantics.
- [ ] Implement cursor/presence broadcasting across clients.
- [ ] Add persistence and reconnect/replay behavior.
- [ ] Add tests for concurrent edits, merge convergence, and reconnect recovery.

## Extra

- [ ] Add rich-text attributes or inline comments.

## Tips

- Convergence semantics are the hard part; isolate them from UI or transport.
- Start with reproducible operation traces before live networking.
- Presence state should not be mixed with document state.
- Replay fixtures make race conditions much easier to debug.
