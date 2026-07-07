# TODO: mini_runtime (⭐ 10/10)

## Usage

```bash
cargo run --bin mini_runtime
cargo test --bin mini_runtime
```

## Learn Notes

- pinning: `Pin<P>` prevents a value from being moved after pinning; needed for self-referential structures.
- unsafe polling: manual `Future::poll` implementations often require `unsafe { Pin::new_unchecked(...) }` when not `Unpin`.
- async internals: executors poll futures with a `Context`; on `Pending`, futures store the waker and call `wake()` when ready.

## Notes

- Source file intentionally stays as a scaffold stub until implementation starts.

## 1. Manual Future

- [ ] Implement a `TimerFuture` that yields `Pending` on the first poll and `Ready` on the second.
- [ ] Store a `completed: Arc<AtomicBool>` flag; background thread sets it and calls `waker.wake()`.

Acceptance check: awaiting `TimerFuture` in a test resolves after the delay.

## 2. Task and Waker

- [ ] Define a `Task` struct wrapping a `Pin<Box<dyn Future<Output = ()>>>`.
- [ ] Implement the `Wake` trait for `Task` so `waker.wake()` pushes the task back to the ready queue.

Acceptance check: `wake()` moves the task to the run queue without panicking.

## 3. Single-threaded executor

- [ ] Maintain a `VecDeque<Arc<Task>>` as the run queue.
- [ ] `run()` loop: pop a task, call `poll` with a Waker backed by `Arc::clone(&task)`.
- [ ] Stop when the queue is empty and no tasks are outstanding.

Acceptance check: `executor.spawn(async { 42 })` runs to completion.

## 4. Multiple tasks

- [ ] Spawn two `TimerFuture` tasks; verify both complete.
- [ ] Demonstrate that task 2 makes progress while task 1 is sleeping (interleaving).

Acceptance check: both tasks print their completion message; task 2 completes before task 1 if its delay is shorter.

## 5. Tests

- [ ] A simple `async { 1 + 1 }` task resolves to 2.
- [ ] Two concurrent timer tasks both complete.
- [ ] Waker only reschedules the task that called `wake`, not all tasks.

## Extra: multi-threaded scheduler

- [ ] Add a thread pool; distribute tasks with a work-stealing deque (`crossbeam-deque`).

## Tips

- Implement one milestone at a time and keep each slice testable.
- Add a failing test first, then implement the smallest behavior to pass.
- Keep CLI/API surface stable while iterating internals.
- Validate both happy path and error path for every milestone.
- Run focused tests before broad checks.
