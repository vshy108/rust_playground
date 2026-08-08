# TODO Rules

This file records the conventions for project TODO guides in this repository.

## Title Format

Every project TODO file must start with a title in this exact format:

```md
# TODO: <project_name> (⭐ x/10)
```

Examples:

```md
# TODO: random_password (⭐ 1/10)
# TODO: mini_shell (⭐ 6/10)
# TODO: raft_consensus (⭐ 10/10)
```

## Source Of Truth

The difficulty rating in the TODO title must match the project rating in [PLAN.md](PLAN.md).

If a project's difficulty changes in [PLAN.md](PLAN.md), update the matching TODO title as part of the same change.

## Specification Contract

Every new or substantially revised TODO guide should define the behavior before
implementation. Include these sections after the title:

```md
## Specification

### Goal
<one observable outcome>

### Non-goals
- <behavior intentionally excluded>

### Inputs and outputs
- Input: <CLI/API/file input>
- Output: <format and exit behavior>

### Errors and limits
- <invalid input, failure behavior, and resource limits>

### Acceptance criteria
- [ ] <specific behavior or verification>
```

Acceptance criteria must be observable and testable. Do not add a checkbox for
an internal implementation detail unless it protects a documented contract.

## Completion Tracking

A project is considered completed when its TODO guide has no remaining unchecked
`- [ ]` items.

When a project reaches that state:

- add an explicit completion marker to the TODO guide
- update the completed-project summary in [PLAN.md](PLAN.md) as part of the same change

Preferred completion marker format:

```md
## Status

Completed
```

Before marking a project complete, record the verification commands and any
known limitations. A successful compile alone is not sufficient evidence.
After the project passes its acceptance checks and the documentation is updated,
create a focused commit for the completed project and record its hash in the
handoff or change record.

## Clippy Learning Record

Every strict-Clippy failure must be documented in
[CLIPPY_LEARNING_LOG.md](CLIPPY_LEARNING_LOG.md) before the affected milestone
is closed. Each entry should include:

- the diagnostic and affected project
- the underlying cause
- the chosen fix
- the reusable Rust lesson

## File Naming

Project TODO guides should use this file naming pattern:

```md
projects/<project_name>_TODO.md
```

Examples:

```md
projects/random_password_TODO.md
projects/mini_shell_TODO.md
projects/raft_consensus_TODO.md
```

## Why This Rule Exists

- keeps difficulty visible when opening a TODO file directly
- keeps TODO guides consistent with the learning path in [PLAN.md](PLAN.md)
- makes bulk maintenance and review easier
- makes project completion visible without scanning every checkbox manually
