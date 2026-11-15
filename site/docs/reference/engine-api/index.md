# Verdant Engine API reference

      

Look up crates, modules, rollback types, CLI commands, and 60fps budgets.

      60fps worker frame stage budgets in microseconds. A 60fps frame spans 16,666 microseconds; the six-stage parallel worker DAG completes in 6,300 microseconds.The data in this figure
|  | us |
| --- | --- |
| ECS hierarchy | 900 |
| Physics + XPBD | 1800 |
| --- | --- |
| BVH + cull | 1100 |
| NavMesh min-plus | 700 |
| --- | --- |
| FDN audio ring | 600 |
| Tile and draw | 1200 |
      
      

## Semiring and simulation modules

      
        Crates and simulation modules across verdant-tensor and verdant-core
        
          
| Module | Primary types and functions | Purpose |
        
        
          
| verdant_tensor::linear | propagate_hierarchy_matrices_exact, delassus_effective_mass_matrix | Exact (+, *) transforms, 4-joint LBS skinning, and Delassus operators |
          
| verdant_tensor::coded | contract_coded_geometry_grid, contract_packed_animation_tracks | Zero-decode coded_gemm over Grid, Packed, and Runs |
          
| verdant_tensor::tropical | tropical_frustum_distance_bounds, tropical_min_plus_all_pairs_closure | Tropical (max, +) culling/GJK and (min, +) NavMesh closure |
          
| verdant_tensor::audio_ring | convolve_modal_fir_exact, step_sabine_fdn_reverb_ring | Quantized Q15 FIR bars, stereo pan, and 4x4 Hadamard FDN |
          
| ecs & render | SceneWorld, BoundingVolumeHierarchy, RenderFrameGraph | Entity hierarchy, SAH raycasts, 5-pass CSM/IBL graph, and WGSL |
          
| streaming | StandardSceneState, decode_portable_uorc_single_frame, CodedLodFrameBundle | Stateless Wasm UORC streaming and zero-decode coded bundles |
        
      
      

## Authoring, physics, and deterministic rollback modules

      
        GLB transcoding, articulated dynamics, audio occlusion, and netcode
        
          
| Module | Primary types and functions | Purpose |
        
        
          
| gltf_io | import_glb_to_scene, transcode_glb_to_progressive_uorc | Full Khronos GLB ingestion and progressive UORC transcoding |
          
| physics | RigidJointConstraint, RaycastVehicle, CapsuleController | XPBD joints, 4-wheel suspension vehicles, and swept BVH capsules |
          
| collision & navmesh | gjk_intersect_and_manifold, NavMesh | Tropical GJK hulls, Coulomb friction, and A-star / min-plus paths |
          
| audio | evaluate_binaural_cue_with_occlusion, synthesize_binaural_reverb_quantum_exact | BVH-occluded Woodworth binaural cues and Hadamard FDN reverb |
          
| runtime | RollbackSession, DeterministicInputJournal, DeltaUorcSnapshot | 64-frame rollback resimulation, SHA-256 input logs, and delta UORC |
        
      
      

## Deterministic rollback and delta snapshot timeline

      

Networked lockstep and rollback sessions record quantized inputs in a SHA-256 chain and synchronize state deltas via stateless UORC.

      

  
    
  
  Deterministic rollback and delta state sync. Late network commands rewind the bounded 64-frame physics ring and resimulate to bit-identical convergence.
  Rollback and netcode bounds table
    
      
| Component | Bound or format | Verification predicate |
      
        
| DeterministicInputJournal | Monotonic frame commands + 32B SHA-256 chain | inputJournalSequenceValid |
        
| RollbackSession | 64-frame PhysicsWorld history ring | rollbackFrameWithinWindow |
        
| DeltaUorcSnapshot | XOR residual compressed via stateless UORC | WorldSnapshot::decode SHA-256 seal |
      
    
  

      

## CLI subcommands in crates/verdant-cli

      
        
- verdant import-glb  imports nodes, meshes, skeletons, keyframe clips, and PBR materials.
        
- verdant transcode-glb   [seed] transcodes an authored GLB into a portable four-LOD UORC stream.
        
- verdant pack-portable-uorc  [seed] and verify-portable-uorc  pack and verify stateless Wasm streams.
        
- verdant pack-scene , validate-glb, validate-wgsl, snapshot-verify, and benchmark validate assets in CI.
      
    
      
        Previous: Stream zero-decode scenes with UORC
        Next: Unified semiring tensor core and UORC streaming
