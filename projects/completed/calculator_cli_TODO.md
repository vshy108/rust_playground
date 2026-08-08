# TODO: calculator_cli (⭐ 2/10)

## Status

Completed

## Specification

### Goal

Read one arithmetic expression from standard input and print its integer result.

### Non-goals

- Floating-point arithmetic
- Variables, functions, or implicit multiplication
- Network access or configuration files

### Inputs and outputs

- Input: integers, `+`, `-`, `*`, `/`, parentheses, and whitespace
- Output: `Result: <integer>` on success; a readable error on failure

### Errors and limits

- Reject malformed expressions, unexpected characters, missing parentheses, and
  division by zero.
- Reject arithmetic and integer parsing overflow.

### Acceptance criteria

- [x] Operator precedence and parentheses are evaluated correctly.
- [x] Unary plus and minus are supported.
- [x] Invalid expressions return clear errors.
- [x] Focused tests and clippy pass.

## Usage

```bash
cargo run --bin calculator_cli
cargo test --bin calculator_cli
```

## Milestones

- [x] Parse simple expressions from CLI input.
- [x] Implement arithmetic operations with operator precedence.
- [x] Add clear error messages for malformed expressions.
- [x] Format numeric output predictably.
- [x] Add tests for arithmetic correctness and invalid expressions.

## Extra

- [x] Add parentheses support.

## Tips

- Keep tokenization separate from evaluation.
- Small expression fixtures are enough to guard behavior.

## Change record

- Scope: verified the existing calculator implementation and moved it to the
  completed-projects path.
- Assumptions: the existing integer-only expression contract is intentional.
- Tests added: no new tests; existing coverage exercises success and failure
  paths.
- Commands run: `rustfmt projects/completed/calculator_cli.rs`, `cargo test
  --bin calculator_cli`, `cargo clippy --bin calculator_cli --all-features --
  -D warnings`.
- Known limitations: no floating-point or variable support by design.
- Follow-up: begin the next unfinished rating-2 project, `timer_cli`.
