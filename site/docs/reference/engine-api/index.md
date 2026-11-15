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
          
| render | PbrMaterial, validate_wgsl_shader, validate_builtin_engine_shaders | Metallic-roughness PBR materials and W3C WGSL validation |
          
| gltf_io | export_scene_to_glb, validate_glb_bytes | Binary Khronos glTF 2.0 export and verification |
          
| physics | PhysicsWorld, RigidBody, CapsuleController, KinematicSplineRover | 60Hz rigid bodies, player capsules, and spline rovers |
          
| audio | AcousticEnvironment, ListenerPose, SpatialAudioSource, evaluate_binaural_cue | Binaural spatial audio, RT60, and Euler-Bernoulli chime synthesis |
          
| runtime | EngineConfig, FrameBudgetProfiler, WorkerJobGraph, WorldSnapshot | Topological job scheduling and SHA-256 world snapshots |
        
      
      

## Runnable workspace examples

      
        
- cargo run -p verdant-core --example cosmic_sanctuary synthesizes the full sanctuary at t_obs and validates shaders, GLB bytes, physics, and audio.
        
- cargo run -p verdant-core --example custom_game_arena builds a non-sanctuary 3D sci-fi arena to demonstrate general-purpose game usage.
        
- cargo run -p verdant-core --example headless_benchmark profiles 120 frames against the 16.67ms frame budget.
      
    
      
        Previous: Author terrain, shaders, and spatial audio
        Next: World expansion and architectural statics
