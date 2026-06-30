# TODO: live_stream_server (⭐ 8/10)

## Usage

```bash
cargo run --bin live_stream_server
cargo test --bin live_stream_server
```

## Milestones

- [ ] Model ingest sessions, stream keys, and publisher authentication.
- [ ] Implement RTMP-like ingest or simplified live media input abstraction.
- [ ] Add HLS-style segment or playlist generation pipeline.
- [ ] Implement viewer session handling and cache-friendly segment serving.
- [ ] Add stream lifecycle cleanup and idle timeout behavior.
- [ ] Add tests for ingest authorization, playlist updates, and segment retention rules.

## Extra

- [ ] Add transcoding hooks for multiple output bitrates.

## Tips

- Ingest, packaging, and serving are separate stages with different failure modes.
- Playlist freshness and segment retention define viewer experience.
- Authentication and stream-key ownership should be modeled early.
- A fake clock will make time-based playlist tests manageable.
