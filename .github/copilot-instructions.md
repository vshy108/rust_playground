# Rust Playground Copilot Instructions

## Repository purpose

This repository is a Rust learning path made of small, standalone projects.
Projects are intentionally ordered by difficulty from rating 1 to rating 10.
Prefer completing planned projects from the lowest rating upward unless the
user explicitly requests a different project.

## Before changing code

- Read the target project's TODO guide and source file first.
- Check [PLAN.md](../PLAN.md) for the project's rating, status, and intended goal.
- Follow [TODO_RULES.md](../TODO_RULES.md) when creating or updating TODO guides.
- Follow [AI_WORKFLOW.md](../AI_WORKFLOW.md) for the inspect, plan, implement,
  review, and evidence workflow.
- Preserve unrelated user changes and do not rewrite completed projects without
  a specific reason.

## AI coding guardrails

- Treat the TODO guide as the project specification and identify non-goals
  before writing code.
- Do not invent requirements, silently broaden scope, or mark work complete
  without acceptance-test evidence.
- For ambiguous user-visible behavior, ask a focused clarification question;
  for low-risk details, make the smallest documented assumption.
- Prefer vertical slices and reviewable diffs over broad speculative refactors.
- Inspect the final diff for scope creep, unsafe defaults, flaky tests, and
  missing failure-path coverage before reporting completion.
- Report exact verification commands, results, and limitations.
- When a project or bounded milestone is complete, update its documentation and
  create one focused commit. Include the commit hash in the completion report.
- Do not commit unfinished work, unrelated projects, build artifacts, or a
  speculative refactor together with a completed project.

## Implementation workflow

- Work on one TODO milestone at a time.
- Add a focused failing test before implementing a new behavior when practical.
- Keep boundaries clear: parse input, validate it, execute the operation, then
  format output.
- Prefer small, readable functions and explicit error handling over clever
  abstractions.
- Avoid `unwrap()` and `expect()` in normal runtime paths. Return useful errors
  instead; use them only when an invariant is genuinely guaranteed or in tests.
- Keep CLI behavior deterministic and document commands, flags, errors, and
  acceptance criteria in the project's TODO guide.
- Add or update the TODO specification before implementing a substantially new
  behavior.
- Do not introduce networking, randomness, concurrency, persistence, or extra
  dependencies unless they are part of the project's stated goal.
- For networked projects, use timeouts, bounded work, clear cancellation, and
  deterministic tests with local fakes or fixtures.

## Project structure

- Completed projects live under `projects/completed/`.
- Planned projects live under `projects/planned/rating_<n>/`.
- Work-in-progress projects live under `projects/wip/`.
- Shared test data belongs in `fixtures/`.
- New Cargo binaries need a matching `[[bin]]` entry in `Cargo.toml` with a
  lowercase `snake_case` name and the correct source path.
- Add dependencies with `cargo add` when possible and keep `Cargo.lock`
  consistent with intentional dependency changes.

## Ratings and completion

- A TODO title must use `# TODO: <project_name> (⭐ x/10)`.
- The TODO rating must match `PLAN.md`.
- A project is complete only when all TODO checkboxes are checked, the guide
  contains `## Status` followed by `Completed`, and `PLAN.md` is updated.
- Do not raise a project's rating merely to make progress appear complete.

## Verification

Run the narrowest useful checks first, then broader checks as appropriate:

```bash
cargo fmt --all -- --check
cargo check --bin <name>
cargo test --bin <name>
cargo clippy --bin <name> --all-features -- -D warnings
```

For repository-wide changes, also run:

```bash
cargo check --bins
cargo test --all-targets
```

If a check cannot run because a project requires external services, explain the
limitation and provide a local deterministic alternative.

## Change discipline

- Keep diffs scoped to the requested project and its documentation/tests.
- Do not commit build artifacts, secrets, credentials, or generated `target/`
  files.
- Update documentation and tests in the same change as the implementation.
- Report which commands were run and any checks that remain unavailable.
