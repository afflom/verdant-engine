// UHD 4K/8K/16K, Morton Meshlets, Hierarchical LOD Pyramid, Frame Stability Oracle & Paged Tensor Arena benchmark (`demo-uhd.mjs`).

export function runUhdStabilityBenchWithContext({ $, wasm, renderer, readAuxString }) {
  const out = $("log-bench-uhd");
  if (!wasm || !out) return;
  const t0 = performance.now();
  const isSw = renderer?.isSoftware ? 1 : 0;
  const wgpu = typeof navigator !== "undefined" && Boolean(navigator.gpu) && !isSw ? 1 : 0;
  let p4k = null;
  let p8k = null;
  let p16k = null;
  let meshlets = null;
  let pyr = null;
  let stab = null;
  let paged = null;
  if (typeof wasm.verdant_wasm_uhd_profile_aux === "function") {
    if (wasm.verdant_wasm_uhd_profile_aux(3840, 2160, isSw, wgpu, 7500) > 0) {
      try { p4k = JSON.parse(readAuxString(wasm)); } catch {}
    }
    if (wasm.verdant_wasm_uhd_profile_aux(7680, 4320, isSw, wgpu, 9000) > 0) {
      try { p8k = JSON.parse(readAuxString(wasm)); } catch {}
    }
    if (wasm.verdant_wasm_uhd_profile_aux(15360, 8640, isSw, wgpu, 11000) > 0) {
      try { p16k = JSON.parse(readAuxString(wasm)); } catch {}
    }
  }
  if (typeof wasm.verdant_wasm_tensor_paged_benchmark_aux === "function") {
    if (wasm.verdant_wasm_tensor_paged_benchmark_aux(64, 512) > 0) {
      try { paged = JSON.parse(readAuxString(wasm)); } catch {}
    }
  }
  if (typeof wasm.verdant_wasm_uhd_cull_meshlets === "function") {
    const ex = wasm.verdant_wasm_camera_eye_x();
    const ey = wasm.verdant_wasm_camera_eye_y();
    const ez = wasm.verdant_wasm_camera_eye_z();
    const tx = wasm.verdant_wasm_camera_target_x();
    const ty = wasm.verdant_wasm_camera_target_y();
    const tz = wasm.verdant_wasm_camera_target_z();
    if (wasm.verdant_wasm_uhd_cull_meshlets(ex, ey, ez, tx, ty, tz, 16 / 9, 2160, 10) > 0) {
      try { meshlets = JSON.parse(readAuxString(wasm)); } catch {}
    }
  }
  if (typeof wasm.verdant_wasm_uhd_lod_pyramid_aux === "function") {
    if (wasm.verdant_wasm_uhd_lod_pyramid_aux() > 0) {
      try { pyr = JSON.parse(readAuxString(wasm)); } catch {}
    }
  }
  if (typeof wasm.verdant_wasm_record_frame_time_us === "function" && typeof wasm.verdant_wasm_frame_stability_aux === "function") {
    for (let i = 0; i < 64; i += 1) {
      wasm.verdant_wasm_record_frame_time_us(i % 2 === 0 ? 7400 : 8100);
    }
    if (wasm.verdant_wasm_frame_stability_aux() > 0) {
      try { stab = JSON.parse(readAuxString(wasm)); } catch {}
    }
  }
  const elapsed = (performance.now() - t0).toFixed(2);
  const l4 = p4k ? `4K (${p4k.widthPx}×${p4k.heightPx} · ${(p4k.megapixelsX100 / 100).toFixed(2)} MP): DRS ${Math.round(p4k.drsScalePermille / 10)}% + CAS ${Math.round(p4k.casSharpnessPermille / 10)}% · ${p4k.preferredBackend}` : "4K: unavailable";
  const l8 = p8k ? `8K (${p8k.widthPx}×${p8k.heightPx} · ${(p8k.megapixelsX100 / 100).toFixed(2)} MP): DRS ${Math.round(p8k.drsScalePermille / 10)}% + CAS ${Math.round(p8k.casSharpnessPermille / 10)}% · SSE ${(p8k.meshletSseTenthsPx / 10).toFixed(1)}px` : "8K: unavailable";
  const l16 = p16k ? `16K Beyond-8K (${p16k.widthPx}×${p16k.heightPx} · ${(p16k.megapixelsX100 / 100).toFixed(2)} MP): DRS ${Math.round(p16k.drsScalePermille / 10)}% + CAS ${Math.round(p16k.casSharpnessPermille / 10)}% · ${p16k.displayClass}` : "16K: unavailable";
  const lm = meshlets ? `Morton Meshlets: ${meshlets.visibleMeshlets}/${meshlets.totalMeshlets} visible (${meshlets.culledFrustum} frustum, ${meshlets.culledBackfaceCone} cone -> ${(meshlets.cullRatioPermille / 10).toFixed(1)}% culled)` : "Morton Meshlets: unavailable";
  const lp = pyr ? `Hierarchical LOD Pyramid: LOD0 ${pyr.lod0Triangles.toLocaleString()} -> LOD1 ${pyr.lod1Triangles.toLocaleString()} -> LOD2 ${pyr.lod2Triangles.toLocaleString()} tris (${(pyr.lod2ReductionPermille / 10).toFixed(1)}% reduction)` : "LOD Pyramid: unavailable";
  const ls = stab ? `Frame Stability Oracle (${stab.sampleCount}f): P50 ${(stab.p50Us / 1000).toFixed(2)}ms · P95 ${(stab.p95Us / 1000).toFixed(2)}ms · P99 ${(stab.p99Us / 1000).toFixed(2)}ms · Jitter ${(stab.jitterUs / 1000).toFixed(2)}ms · DRS ${Math.round(stab.drsScalePermille / 10)}% (${stab.drsTransitions} transitions · ${stab.stable ? "STABLE" : "BOUNDED"})` : "Frame Oracle: unavailable";
  const lmem = paged ? `Paged Tensor Arena (Packed<Grid,2>): ${(paged.logicalWorkloadBytes / 1048576).toFixed(1)} MB logical in ${(paged.budgetBytes / 1024).toFixed(0)} KiB cap (${(paged.virtualAmplificationPermille / 1000).toFixed(1)}× amp · ${paged.evictions} CLOCK evictions · bitExact=${paged.bitExact} · invariant=${paged.invariantHeld})` : "Paged Tensor Arena: unavailable";
  out.textContent = `VE-41 UHD & Paged Tensor Core (${elapsed} ms):\n${l4}\n${l8}\n${l16}\n${lm}\n${lp}\n${ls}\n${lmem}`;
}
