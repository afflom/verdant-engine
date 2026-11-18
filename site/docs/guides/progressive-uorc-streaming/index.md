# Stream zero-decode scenes and Stitch shards

      

Decode UORC streams in WebAssembly and shard snapshots across Stitch peers.

      Five progressive UORC and HD synthesis levels from LOD0 to LOD4. Browser workers decode stateless UORC frames in WebAssembly and subdivide vaulted architecture up to ultra-high-definition LOD4.The data in this figure
| Step | What happens |
| --- | --- |
| 1. LOD0 proxy | 1/8 scale · bounds + NavMesh |
| 2. LOD1 medium | 2/8 scale · pavilion + observer |
| --- | --- |
| 3. LOD2 high | 5/8 scale · XPBD + vehicle rig |
| 4. LOD3 lossless | 8/8 scale · SHA-256 UORC |
| --- | --- |
| 5. LOD4 ultra HD | 16/8 scale · vaulted Citadel |
      
      

## Decode UORC natively inside the browser worker

      

Verdant Engine compiles uorc_decompress to wasm32-unknown-unknown without shipping the five-megabyte search tables.

      

Cold boot stays under the 120 KiB mobile budget while verdant-uorc.wasm decodes scene-stream.uorc on demand.

      

  
    
    Inspect LOD0Inspect LOD1Inspect LOD2Inspect LOD3Inspect LOD4 HD
  
  Live VDNT byte sizes, triangle counts, and node presets measured across LOD0..LOD4 in verdant-wasm.
  Zero-decode UORC and codec tiers
    
      
| Stage or tier | Representation | Scratch memory |
      
| Portable UORC stream | 4 self-delimiting UORC-SHA256-CHAIN-v1 frames | Stateless uorc_decompress::run |
| --- | --- | --- |
| Tier 0 · Geometry | CodedMatrix Grid 16-entry codebook | 0 bytes float scratch |
| Tier 1 · Animation & SLM | Packed 4-bit nibbles | 0 bytes float scratch |
| --- | --- | --- |
| Tier 2 · IBL harmonics | Runs sparse stream | 0 bytes float scratch |
    
  

      

## Shard worlds with Foundry Prism ID and Stitch parity

      

Sign input journals with your device-held Foundry Prism ID and split snapshots into three STCH shards: half A, half B, and parity A XOR B.

      

Any two distinct Stitch shards reconstruct the exact world snapshot and verify its SHA-256 object commitment.

      

```
let [a, b, parity] = shard_bytes_to_stitch_fragments(&uorc_bytes)?; let recovered = recover_bytes_from_stitch_pair(&a, &parity)?;
```

    
      
        Previous: Author scenes, UI compliance, and spatial audio
        Next: Verdant Engine API reference
