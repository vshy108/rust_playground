# TODO: random_password_cli

## 1. Update Test

- [x] Replace the placeholder `it_works` test with behavior-focused tests.
- [x] Move password generation into a small helper function so tests can call it without calling `main`.
- [x] Check that generated passwords have the requested length.
- [x] Check that the generated characters come from the allowed ASCII printable range.
- [x] Add a small parsing test for `--length 20` once argument parsing is extracted from `main`.

Acceptance check:

```bash
cargo test
```

## 2. AI Improve Version

- Refactor the CLI into smaller functions:
  - [x] `parse_args`
  - [x] `build_charset`
  - [x] `generate_password`
- [x] Use `usize` for password length instead of `i32`.
- [x] Add helpful error messages for missing length, invalid length, zero length, and unknown flags.
- [x] Add a `--symbols` toggle (alphanumeric by default; full printable ASCII with `--symbols`).
- [x] Remove debug argument printing once parsing is working.

Acceptance checks:

```bash
cargo run --bin genpass -- --length 20
cargo run --bin genpass -- --length 20 --symbols
cargo test
```