# TODO: map_tile_server (⭐ 8/10)

## Usage

```bash
cargo run --bin map_tile_server
cargo test --bin map_tile_server
```

## Milestones

- [ ] Define tile source model for MBTiles, PMTiles, or flat files.
- [ ] Implement z/x/y request parsing and bounds validation.
- [ ] Add tile lookup and HTTP serving path.
- [ ] Implement caching and content-type negotiation.
- [ ] Add metadata endpoints for stylesets or source info.
- [ ] Add tests for tile addressing, cache behavior, and missing-tile responses.

## Extra

- [ ] Add vector tile filtering or live layer switching.

## Tips

- Tile addressing bugs are mostly coordinate-conversion bugs; fixture them early.
- Source abstraction should not leak storage details into HTTP handlers.
- Cache keys must include source identity and tile coordinates.
- Start with local tile assets before dynamic reprojection or styling.
