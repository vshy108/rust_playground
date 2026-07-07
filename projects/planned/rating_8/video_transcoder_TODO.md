# TODO: video_transcoder (⭐ 8/10)

## Usage

```bash
cargo run --bin video_transcoder
cargo test --bin video_transcoder
```

## Milestones

- [ ] Model input streams, codecs, and output presets.
- [ ] Implement probe step for source metadata and stream layout.
- [ ] Add transcode pipeline orchestration and progress reporting.
- [ ] Implement segment or chunk processing for large inputs.
- [ ] Add failure recovery and partial-output cleanup.
- [ ] Add tests for argument validation, preset mapping, and progress accounting.

## Extra

- [ ] Add queueing for batch jobs and parallel worker execution.

## Tips

- Separate media probing from job planning from execution.
- Progress reporting should be testable without running full transcodes.
- Temporary output cleanup is part of correctness.
- Start by orchestrating external tools or clear pipeline stages before codec internals.
