# TODO: static_analyzer (⭐ 8/10)

## Usage

```bash
cargo run --bin static_analyzer
cargo test --bin static_analyzer
```

## Milestones

- [ ] Parse source files into a syntax or semantic model.
- [ ] Implement rule engine for a first set of checks.
- [ ] Add file and symbol-level diagnostics with severities.
- [ ] Implement suppression or baseline mechanism.
- [ ] Add SARIF or machine-readable output format.
- [ ] Add tests for rule triggering, false-positive control, and output stability.

## Extra

- [ ] Add inter-file dataflow or taint analysis.

## Tips

- Start with one language or syntax subset before generalizing.
- Rule evaluation and rendering should be decoupled.
- Stable output ordering matters for CI diffs.
- False-positive control is part of product quality, not a later polish item.
