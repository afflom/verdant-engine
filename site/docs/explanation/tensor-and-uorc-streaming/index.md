# Portable tensor kernels and UORC streaming

      

Learn how exact tensor contractions and UORC chains run in every browser.

      Exact accumulator bit widths across engine tensor kernels. Every Prism-certified engine tensor kernel reserves at least 64 bits so inner products never suffer intermediate rounding.The data in this figure
|  | bits |
| --- | --- |
| Static mesh batch | 96 |
| Linear skinning | 128 |
| --- | --- |
| Tropical BVH cull | 64 |
| Tile rasterizer | 128 |
| --- | --- |
| Modal audio mix | 128 |
      
      

## Why Verdant Engine hosts tensor compute in Prism

      

Conditional browser GPU compute extensions are absent on many mobile browsers, embedded WebViews, and privacy-hardened clients.

      

Instead of branching on optional browser APIs, Verdant Engine models its seven core tensor kernels in Prism and executes them via uor-matmul.

      

Every contraction decodes operands into an exact Laurent, integer, or tropical accumulator, sums without intermediate rounding, and encodes once.

      

## Portable 16x16 software tile rasterization

      

When a browser provides WebGL2, the worker streams static and dynamic batches at 60fps inside an OffscreenCanvas context.

      

When hardware contexts are unavailable, render_triangles_tile_framebuffer shades 16x16 RGBA8 tiles with depth testing in WebAssembly and blits directly to Canvas.

      

## Monotonic UORC streaming and runtime integrity

      

Multi-LOD scene worlds compress through UORC into self-delimiting UORC-SHA256-CHAIN-v1 archives achieving ten-to-one lossless compression across four progressive tiers.

      

At runtime, WGSL shaders are checked by naga, glTF binaries are checked by gltf, and every world snapshot verifies a 32-byte SHA-256 digest.

    
      
        Previous: Verdant Core API reference
