# TODO: job_queue_server (⭐ 7/10)

## Usage

```bash
cargo run --bin job_queue_server
cargo test --bin job_queue_server
```

## Milestones

- [ ] Accept job submissions over a simple API.
- [ ] Store pending, running, and completed jobs.
- [ ] Add worker polling or push-based delivery.
- [ ] Support acknowledgements and retry behavior.
- [ ] Add tests for queue ordering and failure recovery.

## Extra

- [ ] Add delayed jobs and priority queues.

## Tips

- Make job lifecycle states explicit before adding concurrency.
- Retry logic should not obscure the original failure cause.
