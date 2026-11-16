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
        
        
          
| verdant_tensor | propagate_hierarchy_matrices_exact, tropical_swept_aabb_toi_bounds, solve_two_bone_ik_exact, verify_conservation_and_reachability | Fifteen unified semiring kernels over uor-matmul |
          
| camera & character | LawfulCameraRig, ArticulatedCharacter, TwoLevelBvh | Near-frustum guard sphere, 22-joint Foot IK, and swept crease CCD |
          
| universe & narrative | UniversePosition, PortalGraph, StoryAutomaton, ConservationLedger | 128m floating-origin cells, portals, and conserved story automata |
          
| ui & cybernetics | UiTree, CyberneticObserverLoop, CodedSlmDirector, ContainerGrantMatrix | Tropical UI compliance, LIF observer, local SLM, and FK-07 grants |
          
| network & runtime | OntologicalRuntimeGovernor, SignedJournalBatch, StitchFragment, RollbackSession, WorldSnapshot | W5H telos governor, Prism ID signing, 2-of-3 Stitch parity, and 64-frame rollback |
        
      
      

## Test live SHA-256 snapshot verification in WebAssembly

      

Click Tamper 1 Byte below to watch verdant_wasm_snapshot_decode_and_restore refuse a corrupted payload live.

      

  
    
    Encode & Verify Clean SnapshotTamper 1 Byte & Test Refusal
  
  Live WorldSnapshot SHA-256 round-trip and single-bit tamper refusal executed in verdant-wasm.
  Rollback and snapshot bounds
    
      
| Component | Bound or format | Verification predicate |
      
| WorldSnapshot | UORC + 32B SHA-256 (max 32,768 B) | wasmBridgeEnvelopeValid |
| --- | --- | --- |
| RollbackSession | 64-frame PhysicsWorld ring | rollbackFrameWithinWindow |
| StitchFragment | STCH binary 2-of-3 parity | stitchPairRecoverable |
    
  

      

## CLI subcommands in crates/verdant-cli

      
        
- verdant import-glb and verdant transcode-glb ingest authored GLB files and pack progressive UORC streams.
        
- verdant verify-ui [width] [height] proves 2D and 3D widget touch bounds, WCAG contrast, and focus closure.
        
- verdant route-intent  routes natural-language commands through the embedded coded_gemm SLM.
        
- verdant stitch-shard and verdant stitch-recover split and reconstruct 2-of-3 SHA-256 Stitch shards.
        
- verdant verify-story and verdant pack-universe audit Chronicle conservation and pack multi-sector Stitch universes.
      
    
      
        Previous: Stream zero-decode scenes and Stitch shards
        Next: Unified semiring tensor core and neuromorphic loop
