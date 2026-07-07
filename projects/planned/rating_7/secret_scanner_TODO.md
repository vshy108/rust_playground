# TODO: secret_scanner (⭐ 7/10)

## Usage

```bash
cargo run --bin secret_scanner
cargo test --bin secret_scanner
```

## Milestones

- [ ] Implement file discovery with ignore rules and binary detection.
- [ ] Add pattern-based detectors for common secret formats.
- [ ] Add entropy-based detector for unknown high-risk tokens.
- [ ] Implement report output with severity and file context.
- [ ] Add optional baseline mode to suppress accepted findings.
- [ ] Add tests for detector precision, recall, and false positives.

## Extra

- [ ] Add live validation hooks for supported providers.

## Tips

- Separate candidate extraction from validation/reporting for clarity.
- Keep detector metadata explicit so findings are explainable.
- Include fixtures with redacted but realistic token structures.
- Validate path filtering against nested VCS directories.
