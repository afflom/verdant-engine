# Stream high-resolution scenes with UORC

      

Stream multi-LOD 3D scenes smoothly up to lossless source detail.

      Four progressive UORC streaming levels from LOD0 to LOD3. Scenes boot fast from coarse proxy bounds and refine monotonically up to SHA-256 verified lossless resolution.The data in this figure
| Step | What happens |
| --- | --- |
| 1. LOD0 proxy | 1/8 scale · bounds and NavMesh |
| 2. LOD1 medium | 2/8 scale · terrain and shadows |
| --- | --- |
| 3. LOD2 high | 5/8 scale · IBL and spline mover |
| 4. LOD3 lossless | 8/8 scale · 1:1 UORC source |
      
      

## Keep the 3D viewport fast on every device

      

Verdant Engine runs simulation, bounding volume hierarchy culling, and WebGL2 or software tile rendering inside an OffscreenCanvas worker.

      

The studio bar keeps six clear controls beside the 3D stage: LOD scrubbing, simulation mode, camera mode, spatial audio, glTF export, and world snapshot sharing.

      

## Scrub progressive LODs and inspect scene nodes

      

Select LOD in the studio bar to step from LOD0 proxy bounds up to lossless LOD3 detail.

      

In Orbit mode, drag or scroll to frame any of the seven canonical engine node presets or follow the kinematic spline mover.

      

Switch to Walk mode on desktop to traverse the terrain at eye level with WASD or arrow keys.

      

## Pack and verify progressive UORC streams

      

Encode all four LOD tiers into one UORC-SHA256-CHAIN-v1 stream and verify every frame within bounded memory.

      

```
let stream = pack_progressive_uorc_stream(7)?; let summary = verify_progressive_uorc_stream(&stream)?; let lod3 = decode_uorc_stream_up_to_lod(&stream, 3)?;
```

    
      
        Previous: Author terrain, shaders, and spatial audio
        Next: Verdant Core API reference
