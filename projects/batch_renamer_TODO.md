# TODO: batch_renamer (⭐ 4/10)

## Usage

```bash
cargo run --bin batch_renamer
cargo test --bin batch_renamer
```

## Milestones

- [ ] Enumerate files from a path or glob input.
- [ ] Implement rename transforms such as prefix, suffix, or replace.
- [ ] Add dry-run preview mode before applying changes.
- [ ] Prevent collisions and invalid rename plans.
- [ ] Add tests for rename planning and collision handling.

## Extra

- [ ] Add sequential numbering templates.

## Tips

- Plan all renames before mutating the filesystem.
- Dry-run output should match apply behavior exactly.
- Collision detection is part of correctness, not a later polish item.
