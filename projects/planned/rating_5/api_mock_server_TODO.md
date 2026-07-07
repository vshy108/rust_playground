# TODO: api_mock_server (⭐ 5/10)

## Usage

```bash
cargo run --bin api_mock_server
cargo test --bin api_mock_server
```

## Milestones

- [ ] Load endpoint definitions from a config file.
- [ ] Match requests by method and path.
- [ ] Return static JSON or text fixtures.
- [ ] Add basic delay, status-code, and header controls.
- [ ] Add tests for route matching and mock responses.

## Extra

- [ ] Add request recording for later inspection.

## Tips

- Define a narrow route schema first so matching rules stay simple.
- Request matching and response rendering should be separate layers.
