# TODO: ci_runner (⭐ 7/10)

## Usage

```bash
cargo run --bin ci_runner
cargo test --bin ci_runner
```

## Milestones

- [ ] Model pipelines, jobs, and execution steps.
- [ ] Load pipeline definitions from a config file.
- [ ] Execute steps with status tracking and logs.
- [ ] Add retry or fail-fast behavior.
- [ ] Add tests for pipeline parsing and job orchestration.

## Extra

- [ ] Add artifact passing between jobs.

## Tips

- Keep job state transitions explicit so failures are easy to reason about.
- Separate command execution from pipeline planning.
