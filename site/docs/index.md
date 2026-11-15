# Verdant Engine documentation

      

Build fast 3D games and interactive worlds for the web and Rust.

      Four ways to explore Verdant Engine documentation. Pick a hands-on tutorial, an authoring guide, the crate API reference, or the semiring tensor architecture.The data in this figure
|  | Study and design | Hands-on code |
| --- | --- | --- |
| Deep context | Tensor architecture | Authoring & GLB |
| Practical steps | API reference | Getting started |
      
      

## Choose your path by role

      

Verdant Engine is a universal 3D game engine written in Rust for native tools and WebAssembly browsers.

      

Every pass runs on standard 3D constructs: scene worlds, bounding volume hierarchies, XPBD physics, spatial audio, WGSL shaders, glTF 2.0, and UORC streaming.

      
- KaiGame developerA runnable 3D game world with ECS, rigid-body physics, spatial audio, and a starter template.
- IrisTechnical artistProcedural terrain, skeletal rigs, validated portable WGSL shaders, progressive UORC streams, and binary glTF 2.0 exports.
- OttoEngine systems engineerExact uor-matmul tensor kernels, progressive UORC streaming budgets, 60fps worker job graphs, and zero magic constants.
- VeraSecurity auditorProof that UORC streaming is monotonic, tensor contractions never overflow, and all assets and snapshots are validated.
- WrenBrowser playerA responsive, clutter-free 60fps 3D viewport with progressive LOD streaming, orbit and walk cameras, and spatial audio.
      

## Full-stack engine architecture

      

Constructive Prism proofs generate the semiring kernel bounds that drive verdant-tensor, verdant-core, and verdant-wasm.

      

  
    
  
  Verdant Engine full-stack architecture. Constructive Prism theorems compile directly into the semiring tensor substrate, 3D subsystems, and WebAssembly workers.
  Architecture layer table
    
      
| Layer | Crate or directory | Core responsibilities |
      
        
| 4 · Browser Studio | site/, crates/verdant-wasm | OffscreenCanvas rendering, native Wasm uorc_decompress, AudioWorklet, and #vdnt= snapshots |
        
| 3 · Engine Core | crates/verdant-core, crates/verdant-cli | ECS, BVH, .glb import and transcode, XPBD joints, vehicles, rollback, and WGSL checks |
        
| 2 · Tensor Substrate | crates/verdant-tensor | Nine semiring kernels and three zero-decode CodedMatrix tiers over uor-matmul |
        
| 1 · Prism Model | app/src/*.lex.tex | Constructive Lean proofs compiled into prism_kernel.rs and app.holo |
      
    
  

      

## Documentation map

      
        
- Build your first 3D world walks through spawning entities, stepping physics, and saving a binary asset.
        
- Author scenes, physics joints, and spatial audio covers .glb import, XPBD joints, vehicles, and occluded 3D sound.
        
- Stream zero-decode scenes with UORC shows how uorc_decompress and coded_gemm stream LOD0 through LOD3 in WebAssembly.
        
- Verdant Engine API reference lists crates, types, deterministic rollback sessions, CLI commands, and 60fps budgets.
        
- Unified semiring tensor core and UORC streaming explains linear, tropical, coded, and audio-ring contractions.
      
    
      
        
        Next: Build your first 3D world
