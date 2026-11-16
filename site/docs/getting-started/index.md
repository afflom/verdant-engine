# Build your first 3D world

      

Make a Rust 3D scene, step physics, and test live world focus.

      Four steps from starter crate to live 3D scene. The starter project builds a scene world, constructs a BVH, steps 60Hz physics, and packs a UORC stream.The data in this figure
| Step | What happens |
| --- | --- |
| 1. Spawn entities | SceneWorld + PBR materials |
| 2. Build BVH | Raycasts + frustum culling |
| --- | --- |
| 3. Step physics | 60Hz joints +observer |
| 4. Stream UORC | Portable GLB + UORC pack |
      
      

## Run the starter template

      

The repository ships a runnable project in templates/browser-3d-starter so you can launch a 3D world right away.

      

```
cargo run --manifest-path templates/browser-3d-starter/Cargo.toml
```

      

That command builds verdant-core, validates all five built-in WGSL shaders, steps GJK triggers, and writes custom-starter-world.glb.

      

## Spawn terrain and child beacons

      

Open templates/browser-3d-starter/src/main.rs to inspect how a scene world organizes parent and child 3D objects.

      

Spawn a root terrain patch first, attach columns as children, and build the bounding volume hierarchy.

      

```
let mut scene = SceneWorld::new(); let ground_id = scene.spawn("StarterTerrain", EntityRole::TerrainPatch, None, Transform::default(), bounds, (1.0, "Terrain")); let bvh = scene.build_bvh();
```

      

## Observe player attention with the live LIF loop

      

Test the integer-exact CyberneticObserverLoop right below by focusing different world slots in WebAssembly.

      

  
    
    Focus Slot 0Focus Slot 2Focus Slot 3Focus Slot 6
  
  Live integer-exact LIF spike raster and entity salience scores computed by verdant-wasm.
  Neuromorphic observer parameters
    
      
| Parameter | Verified bound | Role |
      
| State neurons | 8 Q15 LIF units | Recurrent membrane integration without float drift |
| --- | --- | --- |
| Decay factor | 7 / 8 per tick | Bounded exponential leak across rollback frames |
| Mutual gimbal yaw | -player_yaw_milli | Orients the world observer toward the player |
    
  

      

## Capture a world snapshot and portable UORC stream

      

Encode a world snapshot and a portable four-LOD UORC stream with SHA-256 verification before exporting.

      

```
let snap_bytes = WorldSnapshot::encode(&lod_scene)?; let restored = WorldSnapshot::decode(&snap_bytes)?; let uorc_bytes = pack_portable_progressive_uorc_stream(7)?;
```

    
      
        Previous: Verdant Engine documentation
        Next: Author scenes, UI compliance, and spatial audio
