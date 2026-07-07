# TODO: toy_debugger (⭐ 8/10)

## Usage

```bash
cargo run --bin toy_debugger
cargo test --bin toy_debugger
```

## Milestones

- [ ] Launch or attach to a traced process.
- [ ] Add software breakpoints with trap instruction patching.
- [ ] Read and write registers and process memory.
- [ ] Add step, continue, and simple backtrace commands.
- [ ] Resolve source/line information from debug metadata where available.
- [ ] Add tests for command parsing and breakpoint bookkeeping.

## Extra

- [ ] Add expression evaluation against the current frame.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
