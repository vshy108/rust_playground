# TODO: physics_engine (⭐ 8/10)

## Usage

```bash
cargo run --bin physics_engine
cargo test --bin physics_engine
```

## Milestones

- [ ] Implement vector math and rigid body state representation.
- [ ] Add broad-phase collision detection over simple shapes.
- [ ] Add narrow-phase intersection and collision manifold generation.
- [ ] Implement impulse-based collision resolution.
- [ ] Add gravity, integration, and stable fixed-timestep updates.
- [ ] Add tests for collision cases, energy stability, and deterministic stepping.

## Extra

- [ ] Add joints, constraints, and sleeping bodies.

## Tips

- Build the frame/update loop first, then add features one subsystem at a time.
- Keep input, simulation, and rendering separated for easier testing.
- Add deterministic replay fixtures so behavior can be reproduced exactly.
- Gate expensive rendering or effects behind flags while debugging logic.
- Add smoke tests for startup, shutdown, and basic interaction paths.
