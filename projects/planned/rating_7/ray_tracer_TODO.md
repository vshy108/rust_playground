# TODO: ray_tracer (⭐ 7/10)

## Usage

```bash
cargo run --bin ray_tracer > image.ppm
cargo test --bin ray_tracer
```

## Milestones

- [ ] Write a PPM image with a simple color gradient.
- [ ] Implement vector math, rays, and a pinhole camera.
- [ ] Add sphere intersection and surface normal shading.
- [ ] Add anti-aliasing with multiple samples per pixel.
- [ ] Add diffuse, metal, and dielectric materials.
- [ ] Add tests for vector ops, intersections, and color clamping.

## Extra

- [ ] Add BVH acceleration and scene loading.

## Tips

- Build the frame/update loop first, then add features one subsystem at a time.
- Keep input, simulation, and rendering separated for easier testing.
- Add deterministic replay fixtures so behavior can be reproduced exactly.
- Gate expensive rendering or effects behind flags while debugging logic.
- Add smoke tests for startup, shutdown, and basic interaction paths.
