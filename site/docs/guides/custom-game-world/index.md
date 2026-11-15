# Author terrain, shaders, and spatial audio

      

Build 3D terrain meshes, check GPU shaders, and mix 3D sound.

      Asset and sensory pipeline in verdant-core. Procedural geometry, PBR materials, WGSL shaders, and spatial audio converge into browser rendering and GLB export.The data in this figure
| From | Link | To |
| --- | --- | --- |
| Geometry | → | SceneWorld |
| PBR materials | → | SceneWorld |
| --- | --- | --- |
| SceneWorld | → | Browser and GLB |
| AudioScene | → | Browser and GLB |
      
      

## Generate heightfield terrain and botanical meshes

      

Call generate_terrain_mesh with TerrainReliefParams to create an indexed triangle grid with analytical surface normals.

      

For columns, quartz outcrops, and botanical petals, call generate_fluted_entasis_column_mesh, generate_hexagonal_quartz_mesh, or canonical_scene_mesh_library.

      

When foliage bends in the wind, blend vertex offsets with MorphTarget.

      

## Validate custom WGSL shaders before GPU upload

      

Every custom shader passes through W3C naga front-end parsing and module validation before reaching a browser GPU.

      

```
let shaders = validate_builtin_engine_shaders()?; assert_eq!(shaders.len(), 3);
```

      

## Mix spatial audio with reverberation

      

Create an AcousticEnvironment with a ListenerPose and evaluate SpatialAudioSource emitters in 3D space.

      

Each call to evaluate_binaural_cue computes Woodworth spherical-head interaural time difference, level difference, and Sabine RT60 decay.

      

## Export binary glTF 2.0 scenes

      

Call export_scene_to_glb(&scene, &meshes, &materials) to pack positions, normals, UV coordinates, indices, and metallic-roughness materials into one GLB file.

      

Use validate_glb_bytes in asset pipelines to verify the binary container through the Khronos gltf parser.

    
      
        Previous: Build your first 3D world
        Next: Verdant Core API reference
