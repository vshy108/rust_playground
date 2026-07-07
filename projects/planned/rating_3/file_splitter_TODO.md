# TODO: file_splitter (⭐ 3/10)

## Usage

```bash
cargo run --bin file_splitter
cargo test --bin file_splitter
```

## Milestones

- [ ] Split a file by line count or byte size.
- [ ] Generate deterministic part names.
- [ ] Add reassembly or manifest metadata mode.
- [ ] Handle tiny files and exact-boundary splits correctly.
- [ ] Add tests for split sizes and part naming.

## Extra

- [ ] Add compression option for output parts.

## Tips

- Naming rules should be stable and easy to test.
- Byte-based and line-based split logic should stay separate.
