# Unified semiring tensor core and UORC streaming

      

See how the engine runs exact math and scene streams in any browser.

      Exact accumulator bit widths across all nine unified tensor kernels. Every Prism-certified kernel in Tensor.lex.tex reserves 64 to 128 exact accumulator bits so contractions never round mid-sum.The data in this figure
|  | bits |
| --- | --- |
| Linear (+,*) · transforms (k0,6) | 96 |
| Linear (+,*) · LBS & CSM (k1,k4) | 128 |
| --- | --- |
| Tropical (max,+) · cull/GJK (k2) | 64 |
| Linear (+,*) · tile PBR (k3) | 128 |
| --- | --- |
| Audio Q15 · FIR & FDN (k5) | 128 |
| Tropical (min,+) · NavMesh (k7) | 64 |
| --- | --- |
| Coded zero-decode · stream (k8) | 128 |
      
      

## Why Verdant Engine distills every pass to semiring tensor logic

      

Conditional browser GPU compute APIs are absent on many mobile browsers, embedded WebViews, and hardened desktop clients.

      

Instead of maintaining divergent code paths, Tensor.lex.tex models nine unified semiring kernels executed by crates/verdant-tensor over uor-matmul.

      

  
    
  
  Four unified semiring execution domains in verdant-tensor. Every hot loop reduces to an exact, rounding-free contraction over uor-matmul.
  Semiring kernel domain table
    
      
| Semiring domain | Algebra | Engine subsystems |
      
        
| LinearExact | (+, *) exact Laurent / i128 | Transforms, 4-joint LBS skinning, Delassus joints, 16x16 tile PBR |
        
| CodedZeroDecode | coded_gemm over Grid, Packed, Runs | Zero-scratch compressed geometry, animation tracks, and IBL probes |
        
| TropicalMaxPlus / MinPlus | (max, +) and (min, +) | Frustum culling, GJK hull support, and NavMesh shortest-path closure |
        
| ModularAudioRing | Quantized Q15 -> i64 | Modal FIR bars, Woodworth stereo pan, and 4x4 Hadamard FDN reverb |
      
    
  

      

## Zero-decode contraction with uor_matmul::codec

      

Decompressing assets into temporary float arrays wastes cache bandwidth and mobile memory during progressive scene streaming.

      

By pairing uorc_decompress with CodedMatrix tiers, coded_gemm decodes each codebook entry into an exact integer accumulator and writes once.

      

## Portable 16x16 software tile rasterization and runtime proofs

      

When WebGL2 is present, the OffscreenCanvas worker streams static and dynamic batches at 60fps off the main thread.

      

When hardware contexts are absent, render_triangles_tile_framebuffer shades 16x16 RGBA8 tiles in WebAssembly with identical deterministic visibility.

      

Every constant in shipped Rust and constructive LexLean derives from {0, 1}, enforced continuously by the FI-08 zero-constants ratchet.

    
      
        Previous: Verdant Engine API reference
