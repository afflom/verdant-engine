# Verdant Engine Browser 3D Starter — Sanctuary Relic Resonator

A standalone, playable 3D starter project for building browser and native games on [`verdant-core`](../../crates/verdant-core).

## Included files

- `Cargo.toml` — standalone Rust crate referencing `verdant-core`.
- `src/main.rs` — constructs the **Sanctuary Relic Resonator** game world (`SceneWorld` + SAH `BVH`, two-joint LBS `Skeleton` + `SkeletalClip`, five-pass `RenderFrameGraph` + four-cascade CSM + `IblEnvironmentProbe`, 3D `GJK` convex relic triggers, walkable terrain `NavMesh` + `A*` Guardian Drone corridor steering, `PhysicsWorld`, binaural `AudioScene`, SHA-256 `WorldSnapshot`), exports `custom-starter-world.glb`, and includes unit tests.
- `index.html` and `game.mjs` — zero-dependency playable 3D WebGL2 game where the player pilots the Resonator Orb with `WASD`/Arrow keys across an amphitheater heightfield to harmonize three Quartz Relic Beacons (triggering Euler-Bernoulli WebAudio chime chords) while avoiding the patrolling Guardian Drone.

## Verify and run locally

```bash
cargo test --manifest-path templates/browser-3d-starter/Cargo.toml
cargo run --manifest-path templates/browser-3d-starter/Cargo.toml
python3 -m http.server 8080 --directory templates/browser-3d-starter
```
