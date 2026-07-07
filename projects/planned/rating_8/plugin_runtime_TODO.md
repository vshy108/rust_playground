# TODO: plugin_runtime (⭐ 8/10)

## Usage

```bash
cargo run --bin plugin_runtime
cargo test --bin plugin_runtime
```

## Milestones

- [ ] Define a minimal plugin contract and host API.
- [ ] Load plugins from a directory or manifest.
- [ ] Isolate plugin execution and capture failures.
- [ ] Pass structured requests and responses through the host.
- [ ] Add tests for loading, dispatch, and plugin errors.

## Extra

- [ ] Add sandboxing or version compatibility checks.

## Tips

- Keep the host/plugin boundary narrow and typed.
- Error isolation matters more than feature count in early iterations.
