# TODO: sbom_scanner (⭐ 7/10)

## Usage

```bash
cargo run --bin sbom_scanner
cargo test --bin sbom_scanner
```

## Milestones

- [ ] Parse package manifests into a normalized dependency graph.
- [ ] Generate a simple SBOM document (JSON or CycloneDX-like shape).
- [ ] Add vulnerability feed lookup abstraction.
- [ ] Produce severity summary and remediation hints.
- [ ] Add tests for graph correctness and report formatting.

## Extra

- [ ] Add SPDX output support.

## Tips

- Keep the internal graph format independent from output schema.
- Model package identity with ecosystem + name + version triplets.
