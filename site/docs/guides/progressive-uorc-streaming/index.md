# Stream zero-decode scenes with UORC

      

Pack and read UORC scene streams step by step in the browser.

      Four progressive UORC streaming levels from LOD0 to LOD3. Browser workers decode stateless UORC frames in WebAssembly and refine scenes monotonically to lossless source fidelity.The data in this figure
| Step | What happens |
| --- | --- |
| 1. LOD0 proxy | 1/8 scale · bounds + NavMesh |
| 2. LOD1 medium | 2/8 scale · terrain + shadows |
| --- | --- |
| 3. LOD2 high | 5/8 scale · IBL + vehicle rig |
| 4. LOD3 lossless | 8/8 scale · 5.7 KB UORC |
      
      

## Decode UORC natively inside the browser worker

      

Verdant Engine compiles uorc_decompress directly to wasm32-unknown-unknown without shipping the five-megabyte search tables.

      

Cold boot stays inside the 120 KiB mobile budget while verdant-uorc.wasm decodes scene-stream.uorc frames on demand.

      

  
    
  
  Native WebAssembly zero-decode streaming. Portable .uorc archives decompress inside the worker and contract directly via uor_matmul::codec without float buffer expansion.
  Zero-decode codec tier table
    
      
| Stage or tier | Representation | Scratch memory |
      
        
| Portable UORC stream | 4 self-delimiting UORC-SHA256-CHAIN-v1 frames (5.7 KB) | Stateless uorc_decompress::run |
        
| Tier 0 · Geometry | CodedMatrix Grid 16-entry codebook | 0 bytes intermediate float buffer |
        
| Tier 1 · Animation | Packed sub-byte 4-bit nibbles | 0 bytes intermediate float buffer |
        
| Tier 2 · IBL harmonics | Runs run-length sparse stream | 0 bytes intermediate float buffer |
      
    
  

      

## Contract CodedMatrix tiers without float expansion

      

Each streamed tier pairs with uor_matmul::codec so coded_gemm contracts compressed weights without intermediate float buffers.

      

Geometry uses Grid lookup tables, animation tracks use 4-bit Packed nibbles, and IBL probes use sparse Runs.

      

## Scrub progressive LODs and inspect visual telemetry

      

Select LOD or Stream .uorc in the Studio rail to step from coarse proxy bounds to lossless LOD3 detail.

      

Select Visuals in the Studio rail to open the high-contrast pipeline, semiring kernel, and UORC compression inspector.

      

```
let stream = pack_portable_progressive_uorc_stream(7)?; let lod3 = decode_portable_uorc_single_frame(&stream, 3)?; let coded = CodedLodFrameBundle::synthesize(7, 3)?;
```

    
      
        Previous: Author scenes, physics joints, and spatial audio
        Next: Verdant Engine API reference
