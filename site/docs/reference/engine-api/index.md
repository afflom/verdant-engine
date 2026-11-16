# Verdant Engine API reference

      

Find Rust crates, UI and peer types, CLI flags, and 60fps budgets.

      60fps worker frame stage budgets in microseconds. A 60fps frame spans 16,666 microseconds; the parallel worker DAG completes in 6,300 microseconds.The data in this figure
|  | us |
| --- | --- |
| ECS + UI solver | 900 |
| Physics + XPBD | 1800 |
| --- | --- |
| BVH + cull | 1100 |
| LIF + NavMesh | 700 |
| --- | --- |
| FDN audio ring | 600 |
| Tile and draw | 1200 |
      
      

## Core, UI, cybernetic, and network modules

      
        Primary modules across verdant-tensor and verdant-core
        
          
| Module | Primary types and functions | Purpose |
        
        
          
| verdant_tensor | propagate_hierarchy_matrices_exact, tropical_ui_box_extents, step_neuromorphic_lif_ssm_ring, infer_coded_slm_intent_logits | Twelve unified semiring kernels over uor-matmul |
          
| ui | UiTree, UiWidget, SolvedUiRect | Tropical 2D/3D layout, 48px targets, and WCAG 2.2 AA verification |
          
| cybernetics | CyberneticObserverLoop, CodedSlmDirector, ContainerGrantMatrix | Mutual-attention LIF recurrence, local SLM, and FK-07 denial-wins grants |
          
| network | SignedJournalBatch, StitchFragment, SurvivalEntropyTracker | Prism ID session signing and 2-of-3 Stitch parity custody |
          
| gltf_io & streaming | import_glb_to_scene, transcode_glb_to_progressive_uorc, StandardSceneState | Khronos GLB authoring and stateless Wasm UORC streaming |
          
| physics & runtime | RigidJointConstraint, RollbackSession, WorldSnapshot | XPBD joints, 64-frame rollback, and SHA-256 world snapshots |
        
      
      

## Test live SHA-256 snapshot verification in WebAssembly

      

Click Tamper 1 Byte below to watch verdant_wasm_snapshot_decode_and_restore refuse a corrupted payload live.

      

  
    
    Encode & Verify Clean VDNTTamper 1 Byte & Test Refusal
  
  Live WorldSnapshot SHA-256 round-trip and single-bit tamper refusal executed in verdant-wasm.
  Rollback and snapshot bounds
    
      
| Component | Bound or format | Verification predicate |
      
| WorldSnapshot | VDNT + 32B SHA-256 (max 32,768 B) | wasmBridgeEnvelopeValid |
| --- | --- | --- |
| RollbackSession | 64-frame PhysicsWorld ring | rollbackFrameWithinWindow |
| StitchFragment | STCH binary 2-of-3 parity | stitchPairRecoverable |
    
  

      

## CLI subcommands in crates/verdant-cli

      
        
- verdant import-glb and verdant transcode-glb ingest authored GLB files and pack progressive UORC streams.
        
- verdant verify-ui [width] [height] proves 2D and 3D widget touch bounds, WCAG contrast, and focus closure.
        
- verdant route-intent  routes natural-language commands through the embedded coded_gemm SLM.
        
- verdant stitch-shard and verdant stitch-recover split and reconstruct 2-of-3 SHA-256 Stitch shards.
      
    
      
        Previous: Stream zero-decode scenes and Stitch shards
        Next: Unified semiring tensor core and neuromorphic loop
