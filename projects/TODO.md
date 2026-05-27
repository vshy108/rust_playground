# TODO: random_password_cli

## 1. Update Test

- Replace the placeholder `it_works` test with behavior-focused tests.
- Move password generation into a small helper function so tests can call it without calling `main`.
- Check that generated passwords have the requested length.
- Check that the generated characters come from the allowed ASCII printable range.
- Add a small parsing test for `--length 20` once argument parsing is extracted from `main`.

Acceptance check:

```bash
cargo test
```

## 2. AI Improve Version

- Refactor the CLI into smaller functions:
  - `parse_args`
  - `build_charset`
  - `generate_password`
- Use `usize` for password length instead of `i32`.
- Add helpful error messages for missing length, invalid length, zero length, and unknown flags.
- Add a `--symbols` or `--no-symbols` toggle.
- Remove debug argument printing once parsing is working.

Acceptance checks:

```bash
cargo run --bin genpass -- --length 20
cargo run --bin genpass -- --length 20 --symbols
cargo test
```