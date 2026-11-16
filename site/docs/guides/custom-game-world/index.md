# Author scenes, UI compliance, and spatial audio

      

Import GLB scenes, verify 2D and 3D UI compliance, and step physics with occluded sound.

      Authoring, UI compliance, and physics pipeline in verdant-core. External GLB assets, formal UI trees, XPBD joints, and occluded audio unite inside SceneWorld.The data in this figure
| From | Link | To |
| --- | --- | --- |
| GLB importer | → | SceneWorld + BVH |
| Formal UiTree | → | SceneWorld + BVH |
| --- | --- | --- |
| SceneWorld + BVH | → | XPBD + vehicles |
| SceneWorld + BVH | → | UORC + Studio |
| --- | --- | --- |
| XPBD + vehicles | → | UORC + Studio |
      
      

## Import external GLB scenes and transcode to UORC

      

Call import_glb_to_scene or drop any .glb file onto the Studio viewport to load hierarchies, meshes, skins, and PBR materials.

      

Call transcode_glb_to_progressive_uorc or run verdant transcode-glb authored.glb scene.uorc to emit a four-LOD stream.

      

```
let imported = import_glb_to_scene(&glb_bytes)?; let uorc_stream = transcode_glb_to_progressive_uorc(&glb_bytes, 7)?;
```

      

## Solve and verify 2D and 3D diegetic UI compliance

      

Build a UiTree to solve intrinsic bounds over tropical (max, +) and flex remainders over exact integer math.

      

Every interactive widget must meet 48x48 CSS pixel touch bounds, WCAG 2.2 AA contrast, and tropical focus reachability.

      

```
let ui = UiTree::canonical_studio_ui_tree(first_entity); let rects = ui.solve_and_verify(1280, 900, Some(&imported.scene))?; let diegetic_bvh = UiTree::build_diegetic_bvh(&rects);
```

      

## Step XPBD joints, vehicles, and spatial audio live

      

Advance verdant_wasm_step live below to watch the spline mover, capsule controller, BVH frustum count, and Woodworth binaural pan update.

      

  
    
    Step 15 Physics Frames (250 ms)
  
  Live 60Hz physics step, SAH BVH frustum query, and Woodworth binaural ITD cue in verdant-wasm.
  Physics and acoustic subsystem bounds
    
      
| Subsystem | Method | Guarantee |
      
| XPBD joints | RigidJointConstraint | Delassus compliance across Hinge, BallSocket, Slider, and Spring |
| --- | --- | --- |
| Swept capsule | step_swept_against_bvh | Continuous BVH obstacle and stair stepping |
| Spatial audio | evaluate_binaural_cue_with_occlusion | BVH ray occlusion and 4-channel Hadamard FDN reverb |
    
  

      

## Validate portable WGSL shaders

      

Every built-in PBR, terrain, fluid, sky, and skinned shadow shader passes W3C naga validation before asset packing.

    
      
        Previous: Build your first 3D world
        Next: Stream zero-decode scenes and Stitch shards
