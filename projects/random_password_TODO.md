# TODO: random_password (⭐ 1/10)

## Usage

```bash
cargo run --bin genpass -- --length 20
cargo run --bin genpass -- --length 20 --symbols
cargo run --bin genpass -- --help
cargo test --bin genpass
```

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

## 3. Next Learning Topics

- [x] Add `--help` flag — print usage and exit with code 0; teaches early return pattern.
- [x] Detect duplicate flags — currently `--length 5 --length 20` silently uses 20; return an error instead.
- [x] Replace `(usize, bool)` tuple with a named struct `Config { length: usize, symbols: bool }` — teaches when a tuple outgrows itself.
- [x] `eprintln!` vs `println!` — errors go to stderr, output to stdout; note why this matters for piping (`genpass | pbcopy`).
## Tips

- Implement one milestone at a time and keep each slice testable.
- Add a failing test first, then implement the smallest behavior to pass.
- Keep CLI/API surface stable while iterating internals.
- Validate both happy path and error path for every milestone.
- Run focused tests before broad checks.
