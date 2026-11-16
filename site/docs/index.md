# Verdant Engine documentation

      

Build verified 3D games, compliant interfaces, and neural worlds for Rust and the web.

      Four ways to explore Verdant Engine documentation. Pick a hands-on tutorial, an authoring guide, the crate API reference, or the semiring architecture.The data in this figure
|  | Study and design | Hands-on code |
| --- | --- | --- |
| Deep context | Tensor & Neuro | Authoring & UI |
| Practical steps | API reference | Getting started |
      
      

## Choose your path by role

      

Verdant Engine is a universal 3D game and interface engine written in Rust and WebAssembly.

      

Every pass runs on scene worlds, bounding volume hierarchies, XPBD physics, WGSL shaders, glTF 2.0, and UORC streaming.

      

It also verifies 2D and 3D UI compliance, local SLM commands, and Stitch peer shards.

      
- KaiGame developerA runnable 3D game world with ECS, XPBD physics, neuromorphic attention, spatial audio, and a starter template.
- IrisTechnical artistExternal GLB scene ingestion, skeletal rigs, validated portable WGSL shaders, and progressive UORC transcoding.
- MiraUI and compliance architectVerified 2D screen-space and 3D diegetic interfaces with 48px touch targets, WCAG contrast, and reachable focus graphs.
- OttoEngine systems engineerTwelve unified uor-matmul semiring kernels, zero-copy Wasm buffers, 60fps worker budgets, and zero magic constants.
- VeraSecurity auditorProof that UORC streaming is monotonic, capability grants enforce denial-wins, and Stitch shards verify SHA-256 seals.
- WrenBrowser playerA responsive 60fps 3D viewport with progressive UORC streaming, mutual-attention camera framing, and spatial audio.
- SolMultiplayer world citizenSelf-custodied Foundry Prism ID sessions, 2-of-3 Stitch parity shards, and deterministic peer rollback rooms.
      

## Live WebAssembly 16x16 software tile viewport

      

Every page in these docs runs site/verdant-wasm.wasm live so you can test the engine while reading.

      

  
    
    Orbit +25 degCycle LOD0..LOD3
  
  Live 16x16 software tile PBR render executed directly by verdant-wasm without conditional GPU flags.
  Engine layer table
    
      
| Layer | Crate or path | Core responsibilities |
      
| 4 · Browser Studio | site/, crates/verdant-wasm | Zero-copy WebGL2 and 16x16 tile PBR, LIF/SLM, OPFS, and Stitch mesh |
| --- | --- | --- |
| 3 · Engine Core | crates/verdant-core, crates/verdant-cli | ECS, BVH, GLB import, XPBD joints, formal UI, and 64-frame rollback |
| 2 · Tensor Substrate | crates/verdant-tensor | Twelve unified semiring kernels and three zero-decode codec tiers |
| --- | --- | --- |
| 1 · Prism Model | app/src/*.lex.tex | Constructive Lean proofs compiled into prism_kernel.rs |
    
  

      

## Documentation map

      
        
- Build your first 3D world walks through spawning entities, stepping physics, and observing mutual attention.
        
- Author scenes, UI compliance, and spatial audio covers .glb import, XPBD joints, formal UI checks, and occluded sound.
        
- Stream zero-decode scenes and Stitch shards shows how uorc_decompress, coded_gemm, and Stitch parity shards run in WebAssembly.
        
- Verdant Engine API reference lists crates, types, rollback sessions, CLI subcommands, and 60fps budgets.
        
- Unified semiring tensor core and neuromorphic loop explains all twelve semiring kernels, LIF recurrence, and embedded SLM routing.
      
    
      
        
        Next: Build your first 3D world
