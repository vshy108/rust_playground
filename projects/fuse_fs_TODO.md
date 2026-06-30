# TODO: fuse_fs (⭐ 8/10)

## Usage

```bash
cargo run --bin fuse_fs
cargo test --bin fuse_fs
```

## Milestones

- [ ] Define an in-memory inode and directory tree model.
- [ ] Implement basic file and directory operations through a FUSE adapter.
- [ ] Add read, write, create, rename, and delete semantics.
- [ ] Add metadata handling for size, timestamps, and permissions.
- [ ] Add persistence or snapshotting for filesystem state.
- [ ] Add tests for tree mutations and path resolution logic.

## Extra

- [ ] Add a content-addressed backing store or deduplicated blobs.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
