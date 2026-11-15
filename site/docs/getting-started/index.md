# Build your first 3D world

      

Create a Rust 3D scene, step rigid-body physics, and export a binary asset.

      Four steps from starter template to exported 3D world. The starter template constructs a scene world, steps 60Hz physics, captures a world snapshot, and writes a binary GLB file.The data in this figure
| Step | What happens |
| --- | --- |
| 1. Spawn entities | SceneWorld and PBR materials |
| 2. Build BVH | Fast raycasts and frustum culling |
| --- | --- |
| 3. Step physics | 60Hz symplectic Euler integration |
| 4. Export GLB | Validated Khronos glTF 2.0 binary |
      
      

## Run the starter template

      

The repository ships a playable project in templates/browser-3d-starter so you can run a 3D game world right away.

      

```
cargo run --manifest-path templates/browser-3d-starter/Cargo.toml
```

      

That command builds verdant-core, validates all four built-in WGSL shaders, steps GJK relic triggers and NavMesh steering, and writes custom-starter-world.glb.

      

## Spawn terrain and child beacons

      

Open templates/browser-3d-starter/src/main.rs to inspect how a scene world organizes 3D objects.

      

You spawn a root terrain mesh first, attach three fluted columns and skinned quartz beacons as children, and build the bounding volume hierarchy.

      

```
let mut scene = SceneWorld::new(); let ground_id = scene.spawn("StarterTerrain", EntityRole::TerrainFeature, None, Transform::default(), bounds, (1.0, "Terrain")); let beacon_id = scene.spawn("NorthRelicBeacon", EntityRole::EngineeredStructure, Some(col_id), Transform::from_xyz(0.0, 4.6, 0.0), beacon_bounds, (1.0, "Beacon")); let bvh = scene.build_bvh();
```

      

## Capture a deterministic snapshot

      

Before exporting the scene, encode a world snapshot so worker threads or URL share links can restore the exact state with SHA-256 verification.

      

```
let snap_bytes = WorldSnapshot::encode(&causal_world)?; let restored = WorldSnapshot::decode(&snap_bytes)?;
```

      

## Play Sanctuary Relic Resonator in the browser

      

Serve templates/browser-3d-starter over HTTP and open index.html to pilot the Resonator Orb with WASD keys and awaken all three beacons.

    
      
        Previous: Verdant Engine documentation
        Next: Author terrain, shaders, and spatial audio
