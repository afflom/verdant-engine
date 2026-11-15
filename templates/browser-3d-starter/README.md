# Verdant Engine Browser 3D Starter — Orbital Beacon Arena

A standalone, playable 3D starter project for building browser and native games on [`verdant-core`](../../crates/verdant-core) and its pure [`verdant-tensor`](../../crates/verdant-tensor) semiring substrate.

## Included files

- `Cargo.toml` — standalone Rust crate referencing `verdant-core`.
- `src/main.rs` — constructs the **Orbital Beacon Arena** 3D game world (`SceneWorld` + SAH `BVH`, two-joint LBS `Skeleton` + `SkeletalClip`, five-pass `RenderFrameGraph` + four-cascade CSM + `IblEnvironmentProbe`, 3D `GJK` convex beacon triggers, walkable terrain `NavMesh` + `A*` patrol drone corridor steering, `PhysicsWorld`, binaural `AudioScene`, progressive `UORC` stream verification, and SHA-256 `WorldSnapshot`), exports `custom-starter-world.glb`, and includes unit tests.
- `index.html` and `game.mjs` — zero-dependency playable 3D WebGL2 game where the player pilots a kinetic sphere with `WASD`/Arrow keys across a procedural valley heightfield to activate three crystalline relay beacons (triggering Euler-Bernoulli modal bar chords) while avoiding the patrolling drone.

## Verify and run locally

```bash
cargo test --manifest-path templates/browser-3d-starter/Cargo.toml
cargo run --manifest-path templates/browser-3d-starter/Cargo.toml
python3 -m http.server 8080 --directory templates/browser-3d-starter
```
