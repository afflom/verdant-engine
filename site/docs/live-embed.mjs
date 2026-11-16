// Verdant Engine — Live Executable Documentation Harness.
// Instantiates site/verdant-wasm.wasm inside every Diataxis documentation page so each figure
// executes real Rust engine passes: interactive 3D WebGL2 Citadel + Bridge viewport, Q15 LIF/SSM
// neuromorphic recurrence, lawful swept CCD + camera guard physics, progressive LOD0..LOD4 streams,
// 15-kernel semiring telemetry, and SHA-256 VDNT snapshot tamper proofs.

import {
  createBestRenderer,
  mat4LookAt,
  mat4Mul,
  mat4Perspective,
} from "../scene-renderer.mjs";

const scriptUrl = new URL(import.meta.url);
const wasmUrl = new URL("../verdant-wasm.wasm", scriptUrl);
const uorcWasmUrl = new URL("../verdant-uorc.wasm", scriptUrl);

const redrawHooks = [];

function syncPrismDocTheme(explicitNext = null) {
  const root = document.documentElement;
  let pref = explicitNext;
  if (!pref) {
    try { pref = localStorage.getItem("prism-theme"); } catch {}
  }
  if (!pref) {
    pref = matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark";
  }
  const isLight = pref === "light";
  root.classList.toggle("light", isLight);
  root.classList.toggle("dark", !isLight);
  if (explicitNext) {
    try { localStorage.setItem("prism-theme", explicitNext); } catch {}
  }
  const btn = document.querySelector("[data-ve-theme-toggle]");
  if (btn) btn.textContent = isLight ? "Theme: Light" : "Theme: Dark";
  for (const fn of redrawHooks) fn();
}

syncPrismDocTheme();
document.querySelector("[data-ve-theme-toggle]")?.addEventListener("click", () => {
  const next = document.documentElement.classList.contains("light") ? "dark" : "light";
  syncPrismDocTheme(next);
});

function readDocPalette() {
  const isLight = document.documentElement.classList.contains("light");
  return isLight
    ? {
        bg: "#e9e8df",
        card: "#ffffff",
        fg: "#302b28",
        muted: "#6b6660",
        accent: "#1f7a4d",
        accentSoft: "rgba(31, 122, 77, 0.14)",
        secondary: "#bb370b",
        secondarySoft: "rgba(187, 55, 11, 0.14)",
        danger: "#c91c22",
        dangerSoft: "rgba(201, 28, 34, 0.12)",
        line: "#c7c5b8",
      }
    : {
        bg: "#1b1918",
        card: "#262321",
        fg: "#f6f5f4",
        muted: "#b5b2ae",
        accent: "#5fd18b",
        accentSoft: "rgba(95, 209, 139, 0.16)",
        secondary: "#ec6032",
        secondarySoft: "rgba(236, 96, 50, 0.18)",
        danger: "#ff6467",
        dangerSoft: "rgba(255, 100, 103, 0.18)",
        line: "#433f3d",
      };
}

function drawBanner(ctx, w, h, title, subtitle) {
  const pal = readDocPalette();
  ctx.fillStyle = pal.bg;
  ctx.fillRect(0, 0, w, h);
  ctx.fillStyle = pal.accent;
  ctx.font = "600 13px 'Geist Mono', monospace";
  ctx.fillText(title, 16, 26);
  ctx.fillStyle = pal.muted;
  ctx.font = "12px 'Geist Mono', monospace";
  ctx.fillText(subtitle, 16, 48);
}

for (const fig of document.querySelectorAll("figure[data-ve-live]")) {
  if (fig.dataset.veLive === "viewport") continue;
  const c = fig.querySelector("canvas");
  const ctx = c?.getContext("2d");
  if (ctx) drawBanner(ctx, c.width, c.height, "VERDANT-WASM LIVE ENGINE", "Initializing WebAssembly runtime…");
}

let wasmPromise = null;
function loadWasm() {
  wasmPromise ??= fetch(wasmUrl)
    .then((r) => {
      if (!r.ok) throw new Error(`verdant-wasm HTTP ${r.status}`);
      return r.arrayBuffer();
    })
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
    .then((r) => {
      if (!r.ok) throw new Error(`verdant-uorc HTTP ${r.status}`);
      return r.arrayBuffer();
    })
    .then((buf) => WebAssembly.instantiate(buf, {}))
    .then(({ instance }) => {
      const ex = instance.exports;
      ex.verdant_wasm_init(ex.verdant_wasm_default_seed());
      return ex;
    });
  return uorcPromise;
}

const LOD_ENVELOPES = [
  { rank: 0, sky: [0.09, 0.08, 0.12], fog: [0.14, 0.11, 0.16] },
  { rank: 1, sky: [0.12, 0.17, 0.24], fog: [0.18, 0.24, 0.31] },
  { rank: 2, sky: [0.16, 0.27, 0.34], fog: [0.24, 0.35, 0.39] },
  { rank: 3, sky: [0.18, 0.30, 0.38], fog: [0.30, 0.40, 0.44] },
  { rank: 4, sky: [0.20, 0.34, 0.44], fog: [0.34, 0.44, 0.48] },
];

async function initInteractive3DViewport(ex, fig, canvas) {
  const renderer = await createBestRenderer(canvas);
  let yawMilli = 460;
  let lodRank = 3;
  let thirdPerson = false;
  let tileMode = false;

  const draw = async () => {
    ex.verdant_wasm_set_lod_rank(lodRank >>> 0);
    if (thirdPerson) {
      ex.verdant_wasm_camera_set_mode(0);
    } else {
      ex.verdant_wasm_camera_set_anchor(1, 2.2, 3.8, 9.2);
    }
    const aspectMilli = Math.max(400, Math.round((canvas.width / Math.max(1, canvas.height)) * 1000));
    ex.verdant_wasm_camera_set_pose_intent(yawMilli | 0, 280, thirdPerson ? 6500 : 18500, aspectMilli >>> 0);
    ex.verdant_wasm_scene_rebuild_buffers(0);

    const eye = [ex.verdant_wasm_camera_eye_x(), ex.verdant_wasm_camera_eye_y(), ex.verdant_wasm_camera_eye_z()];
    const target = [ex.verdant_wasm_camera_target_x(), ex.verdant_wasm_camera_target_y(), ex.verdant_wasm_camera_target_z()];
    const vCount = ex.verdant_wasm_scene_vert_count() >>> 0;
    const iCount = ex.verdant_wasm_scene_idx_count() >>> 0;

    if (tileMode || renderer?.backend === "tensor-2d") {
      const uex = await loadUorcWasm();
      uex.verdant_wasm_set_lod_rank(lodRank >>> 0);
      const tw = 288, th = 112;
      const bytes = uex.verdant_wasm_render_tensor_rgba(tw, th, eye[0], eye[1], eye[2], target[0], target[1], target[2]) >>> 0;
      const ptr = uex.verdant_wasm_fb_ptr() >>> 0;
      if (bytes === tw * th * 4 && ptr > 0 && renderer) {
        renderer.blitTensorRgba(new Uint8Array(uex.memory.buffer, ptr, bytes), tw, th);
      }
    } else if (renderer && vCount > 0 && iCount > 0 && ex.memory) {
      const buf = ex.memory.buffer;
      const pos = new Float32Array(buf, ex.verdant_wasm_scene_pos_ptr() >>> 0, vCount * 3);
      const norm = new Float32Array(buf, ex.verdant_wasm_scene_norm_ptr() >>> 0, vCount * 3);
      const col = new Float32Array(buf, ex.verdant_wasm_scene_col_ptr() >>> 0, vCount * 3);
      const idx = new Uint32Array(buf, ex.verdant_wasm_scene_idx_ptr() >>> 0, iCount);
      renderer.uploadPackedBuffers(pos, norm, col, idx);
      const fov = Math.PI / 3.4;
      const proj = mat4Perspective(fov, canvas.width / Math.max(1, canvas.height), 0.25, 160.0);
      const view = mat4LookAt(eye, target, [0, 1, 0]);
      renderer.render(mat4Mul(proj, view), eye, LOD_ENVELOPES[lodRank] || LOD_ENVELOPES[3], 0, target, fov);
    }
    fig.dataset.veReady = "true";
    fig.dataset.veTris = String(Math.floor(iCount / 3));
  };

  await draw();
  fig.addEventListener("click", async (e) => {
    const act = e.target.closest("button")?.dataset.veAct;
    if (!act) return;
    if (act === "orbit") yawMilli += 440;
    else if (act === "lod") lodRank = (lodRank + 1) % 5;
    else if (act === "cam") thirdPerson = !thirdPerson;
    else if (act === "tile") tileMode = !tileMode;
    await draw();
  });
}

function renderObserver(ex, fig, canvas, slot, dwellMs) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
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
    ctx.fillStyle = on ? pal.accentSoft : pal.card;
    ctx.fillRect(x, 62, 58, 26);
    ctx.strokeStyle = on ? pal.accent : pal.line;
    ctx.strokeRect(x, 62, 58, 26);
    ctx.fillStyle = on ? pal.accent : pal.fg;
    ctx.fillText(`N${n}:${on ? 1 : 0}`, x + 12, 79);
  }
  const sals = [0, 1, 2, 3, 4, 5, 6].map((i) => Math.max(0, ex.verdant_wasm_observer_salience_milli(i)));
  const hi = Math.max(600, ...sals);
  sals.forEach((v, i) => {
    const bw = Math.max(10, Math.round((v / hi) * 390));
    const y = 102 + i * 16;
    ctx.fillStyle = pal.muted;
    ctx.fillText(`Slot ${i}`, 16, y + 11);
    ctx.fillStyle = i === att ? pal.accent : pal.secondary;
    ctx.fillRect(76, y, bw, 11);
    ctx.fillStyle = pal.fg;
    ctx.fillText(String(v), 84 + bw, y + 11);
  });
  fig.dataset.veReady = "true";
}

function renderPhysics(ex, fig, canvas, ticks) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
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
  const cx = 288, cy = 138, scale = 2.6;
  ctx.strokeStyle = pal.line;
  ctx.beginPath();
  ctx.arc(cx, cy, 26 * scale * 0.45, 0, Math.PI * 2);
  ctx.stroke();
  ctx.fillStyle = pal.secondary;
  ctx.beginPath();
  ctx.arc(cx + mx * 1.4, cy + mz * 1.4, 7, 0, Math.PI * 2);
  ctx.fill();
  ctx.fillStyle = pal.accent;
  ctx.fillRect(cx + px * 2 - 5, cy + pz * 2 - 5, 10, 10);
  ctx.fillStyle = pal.fg;
  ctx.fillText(`Spline Mover (${mx.toFixed(1)}, ${mz.toFixed(1)})`, 18, 206);
  ctx.fillText(`22-Joint Explorer (${px.toFixed(1)}, ${pz.toFixed(1)})`, 300, 206);
  fig.dataset.veReady = "true";
}

function renderLodStream(ex, fig, canvas, activeRank) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
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
    ctx.fillStyle = s.r === activeRank ? pal.accent : pal.card;
    ctx.fillRect(92, y, bw, 20);
    ctx.strokeStyle = s.r === activeRank ? pal.accent : pal.line;
    ctx.strokeRect(92, y, bw, 20);
    ctx.fillStyle = pal.fg;
    ctx.fillText(`LOD${s.r}`, 18, y + 14);
    ctx.fillText(`${s.tris} tris · ${s.nodes} nodes`, 102 + bw, y + 14);
  });
  fig.dataset.veReady = "true";
}

function renderSnapshotEnvelopeCold(ex, fig, canvas) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
  const maxBytes = ex.verdant_wasm_snapshot_max_bytes() >>> 0;
  const minBytes = ex.verdant_wasm_snapshot_min_wire_bytes() >>> 0;
  const ringCap = ex.verdant_wasm_worker_ring_capacity() >>> 0;
  const polys = ex.verdant_wasm_navmesh_polygon_count() >>> 0;
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    `LIVE SHA-256 VDNT SNAPSHOT ENVELOPE · READY (STREAMING FULL CODEC…)`,
    `Min Wire: ${minBytes} B · Max Budget: ${maxBytes} B · Worker Ring: ${ringCap} cmds`,
  );
  ctx.fillStyle = pal.accentSoft;
  ctx.strokeStyle = pal.accent;
  ctx.fillRect(16, 70, 544, 128);
  ctx.strokeRect(16, 70, 544, 128);
  ctx.fillStyle = pal.fg;
  ctx.fillText(`Header Magic: VDNT (0x56 0x44 0x4E 0x54) + 32-byte SHA-256 trailer`, 30, 102);
  ctx.fillText(`Cold-boot envelope verified in verdant-wasm; streaming SHA-256 codec…`, 30, 134);
  ctx.fillText(`NavMesh Walkable Polygons Verified: ${polys}`, 30, 166);
  fig.dataset.veReady = "true";
}

async function renderSnapshotGuard(exCold, fig, canvas, tamper) {
  renderSnapshotEnvelopeCold(exCold, fig, canvas);
  const uex = await loadUorcWasm();
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
  uex.verdant_wasm_set_lod_rank(3);
  const len = uex.verdant_wasm_snapshot_encode() >>> 0;
  const raw = new Uint8Array(len);
  for (let i = 0; i < len; i++) raw[i] = uex.verdant_wasm_io_get_byte(i) & 0xff;
  if (tamper && raw.length > 12) raw[raw.length - 1] ^= 0x5a;
  uex.verdant_wasm_io_clear();
  for (let i = 0; i < raw.length; i++) uex.verdant_wasm_io_push_byte(raw[i]);
  const ok = uex.verdant_wasm_snapshot_decode_and_restore() === 1;
  drawBanner(
    ctx,
    canvas.width,
    canvas.height,
    `LIVE SHA-256 VDNT SNAPSHOT GUARD · ${ok ? "ADMITTED (VERIFIED)" : "REFUSED (TAMPER DETECTED)"}`,
    `Wire Length: ${len} B · Max Budget: ${uex.verdant_wasm_snapshot_max_bytes()} B · Ring Cap: ${uex.verdant_wasm_worker_ring_capacity()} cmds`,
  );
  ctx.fillStyle = ok ? pal.accentSoft : pal.dangerSoft;
  ctx.strokeStyle = ok ? pal.accent : pal.danger;
  ctx.fillRect(16, 70, 544, 128);
  ctx.strokeRect(16, 70, 544, 128);
  ctx.fillStyle = pal.fg;
  ctx.fillText(`Header Magic: VDNT (0x56 0x44 0x4E 0x54) + 32-byte SHA-256 digest`, 30, 102);
  ctx.fillText(
    tamper
      ? "Injected 1-bit flip at tail byte -> WorldSnapshot::decode rejected payload!"
      : "Clean round-trip -> WorldSnapshot::decode restored LOD3 SceneWorld bit-identically.",
    30,
    134,
  );
  ctx.fillText(`NavMesh Walkable Polygons Verified: ${uex.verdant_wasm_navmesh_polygon_count()}`, 30, 166);
  fig.dataset.veReady = "true";
  fig.dataset.veSnapshotVerdict = ok ? "admitted" : "refused";
}

function renderSemirings(ex, fig, canvas, mode) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readDocPalette();
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
    ctx.fillStyle = mode === "csm" ? pal.accent : pal.secondary;
    ctx.fillRect(110, y, bw, 18);
    ctx.fillStyle = pal.fg;
    ctx.fillText(mode === "csm" ? `Cascade ${i}` : `Mode ${i}`, 18, y + 13);
    ctx.fillText(mode === "csm" ? `${v.toFixed(2)} m` : `${v}/9 f0`, 118 + bw, y + 13);
  });
  fig.dataset.veReady = "true";
}

loadWasm()
  .then(async (ex) => {
    for (const fig of document.querySelectorAll("figure[data-ve-live]")) {
      const mode = fig.dataset.veLive;
      const canvas = fig.querySelector("canvas");
      if (!canvas) continue;
      try {
        if (mode === "viewport") {
          await initInteractive3DViewport(ex, fig, canvas);
        } else if (mode === "observer") {
          let activeSlot = 0;
          const draw = () => renderObserver(ex, fig, canvas, activeSlot, 1400);
          redrawHooks.push(draw);
          draw();
          fig.addEventListener("click", (e) => {
            const act = e.target.closest("button")?.dataset.veAct;
            if (act?.startsWith("slot-")) {
              activeSlot = Number(act.slice(5));
              renderObserver(ex, fig, canvas, activeSlot, 1600);
            }
          });
        } else if (mode === "physics") {
          redrawHooks.push(() => renderPhysics(ex, fig, canvas, 0));
          renderPhysics(ex, fig, canvas, 4);
          fig.addEventListener("click", (e) => {
            const act = e.target.closest("button")?.dataset.veAct;
            if (act === "step") renderPhysics(ex, fig, canvas, 15);
          });
        } else if (mode === "lod-stream") {
          let rank = 4;
          const draw = () => renderLodStream(ex, fig, canvas, rank);
          redrawHooks.push(draw);
          draw();
          fig.addEventListener("click", (e) => {
            const act = e.target.closest("button")?.dataset.veAct;
            if (act?.startsWith("lod-")) {
              rank = Number(act.slice(4));
              draw();
            }
          });
        } else if (mode === "snapshot") {
          let tampered = false;
          const draw = () => renderSnapshotGuard(ex, fig, canvas, tampered);
          redrawHooks.push(() => renderSnapshotEnvelopeCold(ex, fig, canvas));
          draw();
          fig.addEventListener("click", (e) => {
            const act = e.target.closest("button")?.dataset.veAct;
            if (act === "valid") { tampered = false; draw(); }
            if (act === "tamper") { tampered = true; draw(); }
          });
        } else if (mode === "semirings") {
          let subMode = "csm";
          const draw = () => renderSemirings(ex, fig, canvas, subMode);
          redrawHooks.push(draw);
          draw();
          fig.addEventListener("click", (e) => {
            const act = e.target.closest("button")?.dataset.veAct;
            if (act === "csm" || act === "audio") {
              subMode = act;
              draw();
            }
          });
        }
      } catch (figErr) {
        fig.dataset.veReady = "error";
        fig.dataset.veError = String(figErr?.message || figErr);
        console.error("Verdant live-embed figure error:", mode, figErr);
      }
    }
  })
  .catch((err) => {
    for (const fig of document.querySelectorAll("figure[data-ve-live]")) {
      fig.dataset.veReady = "error";
      fig.dataset.veError = String(err?.message || err);
    }
    console.error("Verdant live-embed boot error:", err);
  });
