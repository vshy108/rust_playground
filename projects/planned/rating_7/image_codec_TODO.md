# TODO: image_codec (⭐ 7/10)

## Usage

```bash
cargo run --bin image_codec
cargo test --bin image_codec
```

## Milestones

- [ ] Parse a simple image container format header and chunk layout.
- [ ] Decode pixel data into an in-memory image buffer.
- [ ] Add encoding path back to the same format.
- [ ] Add checksums, chunk validation, and bounds checking.
- [ ] Add streaming decode for large files.
- [ ] Add tests with golden fixtures for round-trip correctness.

## Extra

- [ ] Add PNG-style filter experiments or custom compression modes.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
