# Author scenes, physics joints, and spatial audio

      

Import GLB scenes, solve articulated joints and vehicles, and mix occluded 3D sound.

      Authoring, physics, and sensory pipeline in verdant-core. External GLB assets, procedural meshes, XPBD joints, raycast vehicles, and occluded audio unite inside SceneWorld.The data in this figure
| From | Link | To |
| --- | --- | --- |
| GLB importer | → | SceneWorld + BVH |
| WGSL shaders | → | SceneWorld + BVH |
| --- | --- | --- |
| SceneWorld + BVH | → | XPBD + vehicles |
| SceneWorld + BVH | → | UORC + Studio |
| --- | --- | --- |
| XPBD + vehicles | → | UORC + Studio |
      
      

## Import external GLB scenes and transcode to UORC

      

Call import_glb_to_scene to load node hierarchies, indexed meshes, skin weights, skeletal clips, and PBR materials into a SceneWorld.

      

Call transcode_glb_to_progressive_uorc or run verdant transcode-glb authored.glb scene.uorc to emit a four-LOD stream.

      

```
let imported = import_glb_to_scene(&glb_bytes)?; let uorc_stream = transcode_glb_to_progressive_uorc(&glb_bytes, 7)?;
```

      

## Configure XPBD joints, swept capsules, and raycast vehicles

      

Attach RigidJointConstraint links with Hinge, BallSocket, Slider, or SpringDamper topology solved via Delassus matrices.

      

Drive characters over stairs with step_swept_against_bvh and simulate four-wheel suspension rigs with RaycastVehicle.

      

## Validate portable WGSL shaders

      

Every built-in PBR, terrain, fluid, sky, and skinned shadow shader passes W3C naga validation before asset packing.

      

```
let shaders = validate_builtin_engine_shaders()?; assert_eq!(shaders.len(), 5);
```

      

## Mix occluded spatial audio and Hadamard reverb

      

Call evaluate_binaural_cue_with_occlusion to cast a line-of-sight ray against the BVH and attenuate obstructed sound sources.

      

Call synthesize_binaural_reverb_quantum_exact to convolve Euler-Bernoulli modal bars and step the four-channel Hadamard FDN tank.

    
      
        Previous: Build your first 3D world
        Next: Stream zero-decode scenes with UORC
