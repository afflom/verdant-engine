# Verdant Core API reference

      

Look up engine modules, core types, error codes, and 60fps frame budgets.

      60fps frame stage budgets in microseconds. A 60fps browser frame spans 16,667 microseconds; the default five-stage job graph completes in 7,900 microseconds.The data in this figure
|  | us |
| --- | --- |
| Physics step | 1800 |
| Spline rover | 600 |
| --- | --- |
| BVH culling | 1100 |
| GPU draw prep | 3200 |
| --- | --- |
| Spatial audio | 1200 |
      
      

## Crate modules in verdant-core

      
        Shipped modules in crates/verdant-core/src
        
          
| Module | Primary types and functions | Purpose |
        
        
          
| math | Vec3, Quat, Mat4, Aabb, Ray, Frustum | 3D linear algebra, rays, and frustum planes |
          
| ecs | SceneWorld, EntityId, Transform, BoundingVolumeHierarchy | Entity hierarchy, world transforms, raycasts, and frustum culling |
          
| genesis | CausalWorldState, CosmicEpoch, BuilderDiscipline, ArchitecturalBlueprint | Monotonic world expansion and Builder statics |
          
| geometry | ProceduralMesh, MorphTarget, generate_terrain_mesh | Procedural terrain, fluted columns, crystals, and botanical geometry |
          
| animation | Skeleton, SkeletalClip, SkinWeights4, skin_mesh_linear_blend | Acyclic joint trees, four-joint LBS skinning, and keyframe sampling |
          
| render | PbrMaterial, RenderFrameGraph, compute_csm_splits, IblEnvironmentProbe | Five-pass PBR frame graph, four-cascade CSM splits, IBL mips, and WGSL checks |
          
| gltf_io | export_scene_to_glb, validate_glb_bytes | Binary Khronos glTF 2.0 export and verification |
          
| collision | ConvexHullCollider, gjk_intersect_and_manifold, solve_contact_impulse | 3D GJK convex manifolds, Baumgarte stabilization, and Coulomb friction cones |
          
| physics | PhysicsWorld, RigidBody, CapsuleController, KinematicSplineRover | 60Hz rigid bodies, player capsules, lagoon buoyancy, and spline rovers |
          
| navmesh | NavMesh, NavPolygon, steer_along_corridor | Slope-filtered walkable navmeshes and admissible A-star corridor steering |
          
| audio | AcousticEnvironment, ListenerPose, SpatialAudioSource, evaluate_binaural_cue | Binaural spatial audio, RT60, and Euler-Bernoulli chime synthesis |
          
| runtime | EngineConfig, FrameBudgetProfiler, WorkerJobGraph, WorldSnapshot | Topological job scheduling and SHA-256 world snapshots |
        
      
      

## WebAssembly bridge and CLI packer

      
        
- crates/verdant-wasm compiles verdant-core to a compact WebAssembly module for OffscreenCanvas WebGPU and WebGL2 workers, AudioWorklet synthesis, and #vdnt= snapshot URLs.
        
- cargo run -p verdant-cli -- validate-glb  checks binary Khronos glTF 2.0 containers and buffer bounds.
        
- cargo run -p verdant-cli -- validate-wgsl  validates W3C WGSL shaders and entry points via naga.
        
- cargo run -p verdant-cli -- export-sanctuary  packs a causal world GLB asset and its companion .vdnt snapshot.
        
- cargo run -p verdant-cli -- snapshot-verify  verifies SHA-256 trailers and URL bridge byte bounds.
      
      

## Runnable workspace examples

      
        
- cargo run -p verdant-core --example cosmic_sanctuary synthesizes the full sanctuary at t_obs and validates shaders, GLB bytes, physics, and audio.
        
- cargo run -p verdant-core --example custom_game_arena builds a non-sanctuary 3D sci-fi arena to demonstrate general-purpose game usage.
        
- cargo run -p verdant-core --example headless_benchmark profiles 120 frames against the 16.67ms frame budget.
      
    
      
        Previous: Author terrain, shaders, and spatial audio
        Next: World expansion and architectural statics
