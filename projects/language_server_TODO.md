# TODO: language_server (⭐ 8/10)

## Usage

```bash
cargo run --bin language_server
cargo test --bin language_server
```

## Milestones

- [ ] Implement JSON-RPC framing and LSP message dispatch.
- [ ] Add document open/change/close lifecycle tracking.
- [ ] Build parser-backed diagnostics for a small toy language or config format.
- [ ] Implement hover, go-to-definition, and completion responses.
- [ ] Add incremental text sync and file cache invalidation.
- [ ] Add tests for protocol framing, diagnostics, and request/response correctness.

## Extra

- [ ] Add semantic tokens and rename support.

## Tips

- Keep transport, protocol, and language intelligence as separate layers.
- Start with one document at a time before handling workspace-wide indexing.
- JSON-RPC error responses should be explicit and test-covered.
- Incremental sync bugs are subtle; add deterministic edit-sequence fixtures.
