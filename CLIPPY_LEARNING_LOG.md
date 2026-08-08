# Strict Clippy Learning Log

This log records strict Clippy failures found while implementing the learning
projects. The verification command used was:

```bash
cargo clippy --bin <name> --all-features -- -D warnings
```

`-D warnings` turns warnings into errors, so these are treated as correctness
and maintainability gates rather than optional style feedback.

## `duplicate_line_filter`

### Failure

```text
clippy::needless-option-as-deref
derefed type is same as origin
```

The code passed `options.path.as_deref()` even though `options.path` was already
an `Option<&str>`.

### Fix

Pass the value directly:

```rust
let input = read_input(options.path)?;
```

### Lesson

Check the concrete type before adding conversion helpers. `as_deref()` is useful
for `Option<String>` to `Option<&str>`, but redundant when the value is already
borrowed.

## `line_ending_converter`

### Failure

```text
dead_code: enum `DetectedEnding` is never used
dead_code: function `detect_line_endings` is never used
```

The detection helper was covered by unit tests but was not used by the runtime
file-conversion path.

### Fix

Use detection to skip rewriting files that already have the requested line
ending. This made the helper part of the production behavior and avoided an
unnecessary write.

### Lesson

Test-only usage does not make production code used. A helper should either be
part of the runtime path, intentionally removed, or explicitly documented as a
test-only helper.

## `file_finder`

### Failure 1

Standalone formatting initially rejected Rust 2024 let-chain syntax because the
file was formatted without an edition setting:

```text
let chains are only allowed in Rust 2024 or later
```

### Fix

Avoided edition-sensitive syntax in the implementation and used explicit
predicate helpers instead. Repository Cargo commands still use the package's
declared Rust 2024 edition.

### Lesson

Tool invocations outside Cargo may not inherit the package edition. Prefer
portable syntax in standalone files, or invoke formatters with the correct
edition when appropriate.

### Failure 2

```text
clippy::collapsible-if
this `if` statement can be collapsed
```

The implementation used nested `if let` and `if` statements for optional
filters.

### Fix

Expressed the optional predicates with `Option::is_none_or`:

```rust
if !options.name.as_ref().is_none_or(|pattern| matches(name, pattern)) {
    return Ok(false);
}
```

### Lesson

When a condition means “missing filter or predicate matches,” an `Option`
predicate helper is clearer and satisfies Clippy without suppressing the lint.

## General rules learned

- Run strict Clippy on the focused binary before marking a milestone complete.
- Prefer fixing the underlying design issue over adding `#[allow(...)]`.
- Keep helpers connected to production behavior, not only tests.
- Inspect inferred types before applying conversion methods.
- Be aware of differences between direct `rustfmt` and Cargo-managed edition
  settings.
- Treat every strict-clippy failure as a candidate for a regression test or a
  reusable design rule.
