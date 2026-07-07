# TODO: hex_viewer (⭐ 3/10)

## Usage

```bash
cargo run --bin hex_viewer
cargo test --bin hex_viewer
```

## Milestones

- [ ] Read binary input from file or stdin.
- [ ] Render offset, hex bytes, and ASCII columns.
- [ ] Add configurable bytes-per-row output.
- [ ] Handle short final rows correctly.
- [ ] Add tests for layout formatting on binary fixtures.

## Extra

- [ ] Add colored byte-category rendering.

## Tips

- Layout math should be isolated from file I/O.
- Binary fixtures make formatting regressions obvious.
