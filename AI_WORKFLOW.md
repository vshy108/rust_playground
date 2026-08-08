# AI-Assisted Development Workflow

This repository uses AI as a disciplined implementation partner, not as a
replacement for project specifications, tests, or review.

## 1. Establish the task contract

Before changing code, identify:

- the exact project and rating
- the requested behavior
- explicit non-goals
- input, output, and error contracts
- constraints such as determinism, performance, safety, or offline testing
- the acceptance checks that prove the work is complete

If a requirement is ambiguous but affects architecture or user-visible
behavior, stop and ask for clarification. Otherwise make the smallest safe
assumption and record it in the TODO guide or change summary.

## 2. Inspect before planning

Read the target source, TODO guide, related tests, `PLAN.md`, `Cargo.toml`, and
nearby fixtures before proposing a change. Search for existing helpers and
patterns before adding new abstractions or dependencies.

## 3. Plan in vertical slices

Create a short plan where each slice has:

1. one observable behavior
2. one focused test or fixture
3. the smallest implementation needed
4. a verification command

Prefer a complete small path through parsing, validation, execution, and output
over a large incomplete subsystem.

## 3.1 Dependency decision rule

Before adding a crate, record why the project needs it and which project starts
using it. The dependency note must include:

- project name and rating
- concrete capability that the standard library does not reasonably provide
- selected crate and the feature flags used
- why an existing dependency or local implementation is not preferable
- license, maintenance, security, size, and offline/cache considerations
- the verification command, such as `cargo check`, focused tests, and strict
  Clippy

Add dependencies at the smallest project slice that requires them, not in
anticipation of future projects. Update `Cargo.toml`, `Cargo.lock`, the
project TODO change record, and this rationale together. For security-sensitive
work, use a vetted crate rather than implementing cryptography, QR encoding,
or protocol primitives from scratch. If the registry is unavailable, leave the
dependency change uncommitted and report the exact blocker.

Current planned dependency starts:

| Project | Dependency need | Start when |
|---------|-----------------|------------|
| `password_store_cli` | authenticated encryption, hashing, and encoding; proposed `aes-gcm`, `sha2`, and `base64` | password-store implementation begins |
| `totp_manager` | HMAC-SHA1, Base32, and constant-time code generation; proposed `hmac`, `sha1`, and `base32` | TOTP implementation begins |
| `qr_tool` | QR matrix encoding/decoding and optional image output; proposed `qrcode` plus an image crate only if export requires it | QR implementation begins |

Existing dependency starts should remain documented in the relevant TODO
guide; do not add a broad dependency bundle merely because several future
projects may eventually need it.

## 4. Implement test-first where practical

Write a failing test for the next behavior, implement the smallest fix, then
refactor only after the test passes. Include at least one invalid-input or
failure-path test for every user-visible operation.

## 5. Review the change

Before declaring completion, inspect the diff for:

- scope creep or invented requirements
- broken public behavior or accidental API changes
- panics, unbounded work, leaks, races, and unsafe defaults
- flaky tests, hidden network access, or time-dependent assumptions
- missing documentation, fixtures, or error-path coverage

Treat AI-generated code as untrusted until it passes tests and manual review.

## 6. Verify and report evidence

Run focused checks first, then broader checks when the change warrants them.
Report the exact commands run, their results, known limitations, and any
follow-up work. Never mark a TODO item complete based only on compilation.

Every strict-Clippy failure must be recorded in
[CLIPPY_LEARNING_LOG.md](CLIPPY_LEARNING_LOG.md) with the diagnostic, root
cause, fix, and reusable lesson. Do this before closing the related milestone.

## 7. Commit each finished project

When a project or clearly bounded milestone is complete:

1. confirm its acceptance criteria are satisfied
2. run the focused verification commands
3. review the final diff and status
4. update its TODO guide and `PLAN.md` if appropriate
5. create one focused commit describing the completed work

Do not combine unrelated projects or unfinished work in the completion commit.
Record the commit hash in the handoff when reporting completion. Keep changes
uncommitted only when the user explicitly asks for a draft or review first.

## Standard change record

For a non-trivial project milestone, record this in the TODO guide or pull
request description:

```md
### Change record

- Scope:
- Assumptions:
- Tests added:
- Commands run:
- Known limitations:
- Follow-up:
```

## Useful prompt pattern

```text
Project: <name> (rating <n>)
Goal: <one observable behavior>
Non-goals: <what must not be changed>
Context: <relevant files, fixtures, and existing behavior>
Contract: <inputs, outputs, errors, invariants>
Acceptance tests: <specific checks>
Constraints: <determinism, safety, performance, offline behavior>

First inspect the repository and propose a small implementation plan.
Do not edit until the plan is clear. Then implement one slice, run focused
verification, review the diff, and report evidence and remaining limitations.
```
