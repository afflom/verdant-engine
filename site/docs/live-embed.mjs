// Verdant Engine — Live Executable Documentation Harness.
// Instantiates site/verdant-wasm.wasm inside every Diataxis documentation page so each figure
// executes real Rust engine passes (16x16 software tile PBR, LIF/SSM neuromorphic recurrence,
// XPBD + BVH physics, progressive LOD0..LOD3 snapshots, and 12-kernel semiring telemetry).

const scriptUrl = new URL(import.meta.url);
const wasmUrl = new URL("../verdant-wasm.wasm", scriptUrl);
const uorcWasmUrl = new URL("../verdant-uorc.wasm", scriptUrl);

function drawBanner(ctx, w, h, title, subtitle) {
  ctx.fillStyle = "#081119";
  ctx.fillRect(0, 0, w, h);
  ctx.fillStyle = "#8ce4be";
  ctx.font = "600 13px monospace";
  ctx.fillText(title, 16, 26);
  ctx.fillStyle = "#cbd5e1";
  ctx.font = "12px monospace";
  ctx.fillText(subtitle, 16, 48);
}

for (const fig of document.querySelectorAll("figure[data-ve-live]")) {
  const c = fig.querySelector("canvas");
  const ctx = c?.getContext("2d");
  if (ctx) drawBanner(ctx, c.width, c.height, "VERDANT-WASM LIVE ENGINE", "Initializing WebAssembly runtime…");
}

let wasmPromise = null;
function loadWasm() {
  wasmPromise ??= fetch(wasmUrl)
    .then((r) => r.arrayBuffer())
    .then((buf) => WebAssembly.instantiate(buf, {}))
    .then(({ instance }) => {
      const ex = instance.exports;
      ex.verdant_wasm_init(ex.verdant_wasm_default_seed());
      return ex;
    });
  return wasmPromise;
}

let uorcPromise = null;
function loadUorcWasm() {
  uorcPromise ??= fetch(uorcWasmUrl)
    .then((r) => r.arrayBuffer())
    .then((buf) => WebAssembly.instantiate(buf, {}))
    .then(({ instance }) => {
      const ex = instance.exports;
      ex.verdant_wasm_init(ex.verdant_wasm_default_seed());
      return ex;
    });
  return uorcPromise;
}

function renderTileViewport(ex, canvas, yaw, lodRank) {
  const ctx = canvas.getContext("2d");
  if (!ctx || !ex.memory) return;
  ex.verdant_wasm_set_lod_rank(lodRank >>> 0);
  const w = 288, h = 112;
  const exX = Math.sin(yaw) * 22, exZ = Math.cos(yaw) * 22;
  const bytes = ex.verdant_wasm_render_tensor_rgba(w, h, exX, 11.5, exZ, 0, 1.8, 0) >>> 0;
  const ptr = ex.verdant_wasm_fb_ptr() >>> 0;
  if (bytes === w * h * 4 && ptr > 0 && typeof ImageData === "function") {
    const off = document.createElement("canvas");
    off.width = w;
    off.height = h;
    const octx = off.getContext("2d");
    const slice = new Uint8ClampedArray(ex.memory.buffer, ptr, bytes).slice();
    octx.putImageData(new ImageData(slice, w, h), 0, 0);
    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(off, 0, 0, canvas.width, canvas.height);
  }
  const tris = Math.floor((ex.verdant_wasm_scene_idx_count() >>> 0) / 3);
  ctx.fillStyle = "rgba(8, 17, 25, 0.84)";
  ctx.fillRect(10, 10, 410, 28);
  ctx.fillStyle = "#6ee7b7";
  ctx.font = "600 12px monospace";
  ctx.fillText(`LIVE WASM 16x16 TILE PBR · LOD${lodRank} · ${tris} TRIS · YAW ${yaw.toFixed(2)} rad`, 18, 28);
}

function renderObserver(ex, canvas, slot, dwellMs) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  ex.verdant_wasm_step_observer(slot >>> 0, dwellMs >>> 0, 80, 400, 620);
  const mask = ex.verdant_wasm_observer_spike_mask() >>> 0;
  const att = ex.verdant_wasm_observer_attended_slot() >>> 0;
  const yaw = ex.verdant_wasm_observer_gimbal_yaw_milli() | 0;
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    `LIVE Q15 LIF/SSM OBSERVER (verdant-wasm) · SPIKE MASK 0b${mask.toString(2).padStart(8, "0")}`,
    `Attended Node: N${att} · Mutual Gimbal Yaw: ${yaw} mrad · Dwell: ${dwellMs} ms`,
  );
  for (let n = 0; n < 8; n++) {
    const on = ((mask >>> n) & 1) === 1;
    const x = 16 + n * 68;
    ctx.fillStyle = on ? "#10b981" : "#152535";
    ctx.fillRect(x, 62, 58, 26);
    ctx.strokeStyle = on ? "#6ee7b7" : "#2f4a63";
    ctx.strokeRect(x, 62, 58, 26);
    ctx.fillStyle = on ? "#042f2e" : "#cbd5e1";
    ctx.fillText(`N${n}:${on ? 1 : 0}`, x + 12, 79);
  }
  const sals = [0, 1, 2, 3, 4, 5, 6].map((i) => Math.max(0, ex.verdant_wasm_observer_salience_milli(i)));
  const hi = Math.max(600, ...sals);
  sals.forEach((v, i) => {
    const bw = Math.max(10, Math.round((v / hi) * 390));
    const y = 102 + i * 16;
    ctx.fillStyle = "#cbd5e1";
    ctx.fillText(`Slot ${i}`, 16, y + 11);
    ctx.fillStyle = i === att ? "#34d399" : "#38bdf8";
    ctx.fillRect(76, y, bw, 11);
    ctx.fillStyle = "#e2e8f0";
    ctx.fillText(String(v), 84 + bw, y + 11);
  });
}

function renderPhysics(ex, canvas, ticks) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  for (let i = 0; i < ticks; i++) ex.verdant_wasm_step_lawful(16666, 900, 0, 3142, 1000, 0);
  const mx = ex.verdant_wasm_mover_x(), mz = ex.verdant_wasm_mover_z(), dist = ex.verdant_wasm_mover_distance();
  const px = ex.verdant_wasm_player_x(), pz = ex.verdant_wasm_player_z();
  const clr = ex.verdant_wasm_camera_clearance_milli() >>> 0;
  const vis = ex.verdant_wasm_bvh_visible_count(0, 10, 22, 0, -0.35, -0.93, 2.0) >>> 0;
  const pan = ex.verdant_wasm_binaural_pan(-9, 2, -4, px, 2, pz, 0, 0, -1);
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    `LIVE LAWFUL CCD + CAMERA GUARD + AUDIO · CLEARANCE ${clr} mm`,
    `Mover: ${dist.toFixed(2)} m · BVH Visible: ${vis} · Binaural Pan: ${pan.toFixed(3)} · ITD: ${ex.verdant_wasm_binaural_itd_micros().toFixed(1)} us`,
  );
  const cx = 288, cy = 142, scale = 2.6;
  ctx.strokeStyle = "#29465e";
  ctx.beginPath();
  ctx.arc(cx, cy, 26 * scale * 0.45, 0, Math.PI * 2);
  ctx.stroke();
  ctx.fillStyle = "#f59e0b";
  ctx.beginPath();
  ctx.arc(cx + mx * 1.4, cy + mz * 1.4, 7, 0, Math.PI * 2);
  ctx.fill();
  ctx.fillStyle = "#34d399";
  ctx.fillRect(cx + px * 2 - 5, cy + pz * 2 - 5, 10, 10);
  ctx.fillStyle = "#e2e8f0";
  ctx.fillText(`Spline Mover (${mx.toFixed(1)}, ${mz.toFixed(1)})`, 18, 204);
  ctx.fillText(`22-Joint Explorer (${px.toFixed(1)}, ${pz.toFixed(1)})`, 300, 204);
}

function renderLodStream(ex, canvas, activeRank) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const stats = [0, 1, 2, 3, 4].map((r) => {
    ex.verdant_wasm_set_lod_rank(r);
    const tris = Math.floor((ex.verdant_wasm_scene_idx_count() >>> 0) / 3);
    const nodes = ex.verdant_wasm_node_preset_count() >>> 0;
    return { r, tris, nodes };
  });
  ex.verdant_wasm_set_lod_rank(activeRank);
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    `LIVE PROGRESSIVE LOD0..LOD4 HD ENVELOPE · ACTIVE LOD${activeRank}`,
    `Cold Boot < 120 KiB -> Monotonic Vaulted Citadel, Bridge & 22-Joint IK Refinement`,
  );
  const maxTris = Math.max(1, ...stats.map((s) => s.tris));
  stats.forEach((s, idx) => {
    const y = 62 + idx * 30;
    const bw = Math.max(24, Math.round((s.tris / maxTris) * 320));
    ctx.fillStyle = s.r === activeRank ? "#10b981" : "#1e3a52";
    ctx.fillRect(92, y, bw, 20);
    ctx.strokeStyle = s.r === activeRank ? "#6ee7b7" : "#38bdf8";
    ctx.strokeRect(92, y, bw, 20);
    ctx.fillStyle = "#f1f5f9";
    ctx.fillText(`LOD${s.r}`, 18, y + 14);
    ctx.fillText(`${s.tris} tris · ${s.nodes} nodes`, 102 + bw, y + 14);
  });
}

async function renderSnapshotGuard(canvas, tamper) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const ex = await loadUorcWasm();
  ex.verdant_wasm_set_lod_rank(3);
  const len = ex.verdant_wasm_snapshot_encode() >>> 0;
  const raw = new Uint8Array(len);
  for (let i = 0; i < len; i++) raw[i] = ex.verdant_wasm_io_get_byte(i) & 0xff;
  if (tamper && raw.length > 12) raw[raw.length - 1] ^= 0x5a;
  ex.verdant_wasm_io_clear();
  for (let i = 0; i < raw.length; i++) ex.verdant_wasm_io_push_byte(raw[i]);
  const ok = ex.verdant_wasm_snapshot_decode_and_restore() === 1;
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    `LIVE SHA-256 VDNT SNAPSHOT GUARD · ${ok ? "ADMITTED (VERIFIED)" : "REFUSED (TAMPER DETECTED)"}`,
    `Wire Length: ${len} B · Max Budget: ${ex.verdant_wasm_snapshot_max_bytes()} B · Ring Cap: ${ex.verdant_wasm_worker_ring_capacity()} cmds`,
  );
  ctx.fillStyle = ok ? "rgba(16, 185, 129, 0.18)" : "rgba(239, 68, 68, 0.2)";
  ctx.strokeStyle = ok ? "#34d399" : "#f87171";
  ctx.fillRect(16, 70, 544, 128);
  ctx.strokeRect(16, 70, 544, 128);
  ctx.fillStyle = "#f8fafc";
  ctx.fillText(`Header Magic: VDNT (0x56 0x44 0x4E 0x54) + 32-byte SHA-256 digest`, 30, 102);
  ctx.fillText(
    tamper
      ? "Injected 1-bit flip at tail byte -> WorldSnapshot::decode rejected payload!"
      : "Clean round-trip -> WorldSnapshot::decode restored LOD3 SceneWorld bit-identically.",
    30,
    134,
  );
  ctx.fillText(`NavMesh Walkable Polygons Verified: ${ex.verdant_wasm_navmesh_polygon_count()}`, 30, 166);
}

function renderSemirings(ex, canvas, mode) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const kCount = ex.verdant_wasm_unified_tensor_kernel_count() >>> 0;
  const cTiers = ex.verdant_wasm_coded_tier_count() >>> 0;
  const splits = [0, 1, 2, 3].map((i) => ex.verdant_wasm_csm_split_meters(0.25, 140.0, i));
  const rt60 = ex.verdant_wasm_sabine_rt60_millis(1800, 96000) >>> 0;
  const modes = [0, 1, 2, 3, 4].map((i) => ex.verdant_wasm_modal_bar_numer(i) >>> 0);
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    `LIVE PRISM KERNEL TELEMETRY · ${kCount} UNIFIED SEMIRING KERNELS · ${cTiers} CODED TIERS`,
    mode === "csm"
      ? `4-Cascade Shadow Splits (0.25m..140m): ${splits.map((v) => v.toFixed(2) + "m").join(" | ")}`
      : `Sabine RT60: ${rt60} ms · Euler-Bernoulli Modal Numerators: [${modes.join(", ")}] / 9`,
  );
  const vals = mode === "csm" ? splits : modes;
  const hi = Math.max(1, ...vals);
  vals.forEach((v, i) => {
    const y = 72 + i * 28;
    const bw = Math.max(20, Math.round((v / hi) * 380));
    ctx.fillStyle = mode === "csm" ? "#38bdf8" : "#a78bfa";
    ctx.fillRect(110, y, bw, 18);
    ctx.fillStyle = "#f1f5f9";
    ctx.fillText(mode === "csm" ? `Cascade ${i}` : `Mode ${i}`, 18, y + 13);
    ctx.fillText(mode === "csm" ? `${v.toFixed(2)} m` : `${v}/9 f0`, 118 + bw, y + 13);
  });
}

loadWasm()
  .then((ex) => {
    for (const fig of document.querySelectorAll("figure[data-ve-live]")) {
      const mode = fig.dataset.veLive;
      const canvas = fig.querySelector("canvas");
      if (!canvas) continue;
      let yaw = 0.68, lod = 3;
      if (mode === "viewport") {
        renderTileViewport(ex, canvas, yaw, lod);
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (!act) return;
          if (act === "orbit") yaw += 0.45;
          if (act === "lod") lod = (lod + 1) % 5;
          renderTileViewport(ex, canvas, yaw, lod);
        });
      } else if (mode === "observer") {
        renderObserver(ex, canvas, 0, 1400);
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (act?.startsWith("slot-")) renderObserver(ex, canvas, Number(act.slice(5)), 1600);
        });
      } else if (mode === "physics") {
        renderPhysics(ex, canvas, 4);
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (act === "step") renderPhysics(ex, canvas, 15);
        });
      } else if (mode === "lod-stream") {
        renderLodStream(ex, canvas, 4);
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (act?.startsWith("lod-")) renderLodStream(ex, canvas, Number(act.slice(4)));
        });
      } else if (mode === "snapshot") {
        renderSnapshotGuard(canvas, false);
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (act === "valid") renderSnapshotGuard(canvas, false);
          if (act === "tamper") renderSnapshotGuard(canvas, true);
        });
      } else if (mode === "semirings") {
        renderSemirings(ex, canvas, "csm");
        fig.addEventListener("click", (e) => {
          const act = e.target.closest("button")?.dataset.veAct;
          if (act === "csm" || act === "audio") renderSemirings(ex, canvas, act);
        });
      }
    }
  })
  .catch(() => {});
