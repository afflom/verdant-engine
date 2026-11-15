# Verdant Engine Browser 3D Starter

A standalone starter project for building custom 3D browser and native games on [`verdant-core`](../../crates/verdant-core).

## Included files

- `Cargo.toml` — standalone Rust binary crate referencing `verdant-core`.
- `src/main.rs` — constructs a custom 3D game world (`SceneWorld`, `PhysicsWorld`, `AudioScene`), validates custom WGSL shaders through `naga`, steps rigid-body physics and spatial audio, captures a SHA-256 `WorldSnapshot`, and exports `custom-starter-world.glb`.
- `index.html` and `game.mjs` — zero-dependency WebGL2 3D arena viewer with orbit camera and real-time directional lighting.

## Run locally

```bash
cargo run --manifest-path templates/browser-3d-starter/Cargo.toml
python3 -m http.server 8080 --directory templates/browser-3d-starter
```
