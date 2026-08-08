# TODO: batch_renamer (⭐ 4/10)

## Usage

```bash
cargo run --bin batch_renamer
cargo test --bin batch_renamer
```

## Milestones

- [x] Enumerate files from a directory input.
- [x] Implement prefix, suffix, and replace transforms.
- [x] Add dry-run preview mode before applying changes.
- [x] Prevent collisions and invalid rename plans.
- [x] Add tests for rename planning and collision handling.

## Extra

- [x] Add sequential numbering templates.

## Progress record

- Implemented deterministic directory enumeration, transforms, dry-run previews,
  apply mode, and collision validation.
- Added deterministic three-digit sequential numbering with `--numbered`.

## Status

Completed.

## Specification

- Goal: provide a safe, preview-first command for bulk renaming immediate files.
- Inputs: a directory plus optional prefix, suffix, replacement, numbering, and
  apply flags.
- Output: one deterministic `old -> new` plan; filesystem changes occur only
  with `--apply`.
- Safety: reject no-op renames, duplicate destinations, existing unrelated
  destinations, and empty target names before mutation.
- Non-goals: recursive traversal and undo history are outside this utility.
- Acceptance: focused tests pass and strict Clippy reports no warnings.

## Change record

- Implemented and verified all core milestones and the numbering extra.

## Tips

- Plan all renames before mutating the filesystem.
- Dry-run output should match apply behavior exactly.
- Collision detection is part of correctness, not a later polish item.
