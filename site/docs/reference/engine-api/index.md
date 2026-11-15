# Verdant Core API reference

      

Look up engine modules, core types, CLI commands, and 60fps budgets.

      60fps frame stage budgets in microseconds. A 60fps browser frame spans 16,666 microseconds; the six-stage job graph completes in 6,300 microseconds.The data in this figure
|  | us |
| --- | --- |
| ECS hierarchy | 900 |
| Physics step | 1800 |
| --- | --- |
| BVH culling | 1100 |
| Spline steering | 700 |
| --- | --- |
| Spatial audio | 600 |
| Tile and draw | 1200 |
      
      

## Core modules in crates/verdant-core

      
        Simulation, tensor, and streaming modules
        
          
| Module | Primary types and functions | Purpose |
        
        
          
| math | Vec3, Quat, Mat4, Aabb, Frustum | 3D linear algebra, rays, and frustum planes |
          
| ecs | SceneWorld, EntityId, BoundingVolumeHierarchy | Entity hierarchy, raycasts, and frustum culling |
          
| tensor | batch_transform_positions_exact, render_triangles_tile_framebuffer | Exact uor-matmul contractions and 16x16 tile PBR |
          
| streaming | StandardSceneState, LodLevel, pack_progressive_uorc_stream | Progressive LOD synthesis and UORC chain streaming |
          
| geometry | ProceduralMesh, MorphTarget, generate_terrain_mesh | Procedural terrain, columns, prisms, and ribbons |
          
| animation | Skeleton, SkeletalClip, skin_mesh_linear_blend | Acyclic joint trees and four-joint LBS skinning |
          
| render | PbrMaterial, RenderFrameGraph, compute_csm_splits | Five-pass PBR graph, CSM splits, IBL mips, and WGSL |
        
      
      

## Physics, audio, and runtime modules

      
        Dynamics, navigation, audio, and snapshot modules
        
          
| Module | Primary types and functions | Purpose |
        
        
          
| gltf_io | export_scene_to_glb, validate_glb_bytes | Binary Khronos glTF 2.0 export and verification |
          
| collision | ConvexHullCollider, gjk_intersect_and_manifold | 3D GJK manifolds, Baumgarte, and Coulomb cones |
          
| physics | PhysicsWorld, RigidBody, KinematicSplineBody | 60Hz rigid bodies, capsules, buoyancy, and splines |
          
| navmesh | NavMesh, NavPolygon, steer_along_corridor | Slope-filtered navmeshes and admissible A-star paths |
          
| audio | AcousticEnvironment, evaluate_binaural_cue | Woodworth binaural cues, Sabine RT60, and modal bars |
          
| runtime | EngineConfig, WorkerJobGraph, WorldSnapshot | Topological job scheduling and SHA-256 world snapshots |
        
      
      

## WebAssembly bridge and CLI commands

      
        
- crates/verdant-wasm compiles verdant-core to WebAssembly for OffscreenCanvas workers, 16x16 tile rendering, AudioWorklet synthesis, and #vdnt= URLs.
        
- cargo run -p verdant-cli -- pack-scene  packs a validated .glb, .vdnt world snapshot, and .uorc stream.
        
- cargo run -p verdant-cli -- verify-uorc  verifies UORC-SHA256-CHAIN-v1 multi-LOD archives.
        
- cargo run -p verdant-cli -- validate-glb  and validate-wgsl  run Khronos and W3C validators.
        
- cargo run -p verdant-cli -- benchmark profiles 120 headless simulation frames against the 60fps budget.
      
    
      
        Previous: Stream high-resolution scenes with UORC
        Next: Portable tensor kernels and UORC streaming
