// Verdant Studio Media, OPFS Storage, Canvas Telemetry, Modal Audio & Prompt Router
// (`site/studio-media.mjs`), extracted from `site/studio.mjs`.

import {
  PRISM_CHAMBER_SABINE_RT60_MS,
  PRISM_HD_MAX_LOD_RANK,
  PRISM_MAX_LOD_RANK,
  PRISM_MODAL_BAR_DENOM,
  PRISM_MODAL_BAR_NUMERS,
} from "./prism-bridge-meta.mjs";
import { bytesToBase64Url } from "./studio-scaffolder.mjs";

export function readThemePalette() {
  const cs = getComputedStyle(document.documentElement);
  const v = (name, fb) => cs.getPropertyValue(name).trim() || fb;
  return {
    bg: v("--muted", "Canvas"),
    card: v("--card", "Canvas"),
    fg: v("--foreground", "CanvasText"),
    mutedFg: v("--muted-foreground", "GrayText"),
    border: v("--border-strong", "GrayText"),
    primary: v("--primary", "CanvasText"),
    primaryFg: v("--primary-foreground", "Canvas"),
    brand: v("--brand", "Highlight"),
    success: v("--success", "Highlight"),
    successSubtle: v("--success-subtle", "Canvas"),
    destructive: v("--destructive", "Mark"),
    destructiveSubtle: v("--destructive-subtle", "Canvas"),
    fontMono: v("--font-mono", "monospace"),
  };
}

const themeListeners = new Set();
let themeObserverStarted = false;
export function subscribeThemeChanges(fn) {
  themeListeners.add(fn);
  if (!themeObserverStarted && typeof MutationObserver !== "undefined") {
    themeObserverStarted = true;
    new MutationObserver(() => {
      for (const cb of themeListeners) cb();
    }).observe(document.documentElement, { attributes: true, attributeFilter: ["class"] });
  }
  return () => themeListeners.delete(fn);
}

export async function cacheBytesInOpfs(filename, bytes) {
  if (!navigator.storage || typeof navigator.storage.getDirectory !== "function") {
    return { cached: false, reason: "OPFS unavailable in this browser context" };
  }
  try {
    const root = await navigator.storage.getDirectory();
    const fh = await root.getFileHandle(filename, { create: true });
    const writable = await fh.createWritable();
    await writable.write(bytes);
    await writable.close();
    return { cached: true, filename, byteLength: bytes.byteLength };
  } catch (err) {
    return { cached: false, reason: String(err?.message || err) };
  }
}

export async function listOpfsAssets() {
  if (!navigator.storage || typeof navigator.storage.getDirectory !== "function") return [];
  try {
    const root = await navigator.storage.getDirectory();
    const entries = [];
    for await (const [name, handle] of root.entries()) {
      if (handle.kind === "file") {
        const f = await handle.getFile();
        entries.push({ name, bytes: f.size });
      }
    }
    return entries;
  } catch {
    return [];
  }
}

export async function readOpfsAssetBytes(filename) {
  if (!navigator.storage || typeof navigator.storage.getDirectory !== "function") return null;
  try {
    const root = await navigator.storage.getDirectory();
    const fh = await root.getFileHandle(filename);
    const f = await fh.getFile();
    return new Uint8Array(await f.arrayBuffer());
  } catch {
    return null;
  }
}

export async function deleteOpfsAsset(filename) {
  if (!navigator.storage || typeof navigator.storage.getDirectory !== "function") return false;
  try {
    const root = await navigator.storage.getDirectory();
    await root.removeEntry(filename);
    return true;
  } catch {
    return false;
  }
}

let studioDialog = null;
let studioCtx = null;
let selectStudioTab = null;
let latestObserverTelemetry = null;
let observerDrawHook = null;
let audioDrawHook = null;
let uiDrawHook = null;
let sequenceTimer = 0;

let studioHostRoot = null;


export function drawObserverCanvas(canvas, tel) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readThemePalette();
  const w = canvas.width;
  const h = canvas.height;
  ctx.fillStyle = pal.bg;
  ctx.fillRect(0, 0, w, h);

  const mask = tel?.spikeMask ?? 5;
  const salience = tel?.salience ?? [420, 180, 260, 310, 190, 220, 390];
  const attended = tel?.attendedSlot ?? 0;

  ctx.fillStyle = pal.fg;
  ctx.font = `600 12px ${pal.fontMono}`;
  ctx.fillText(`Q15 LIF SPIKE RASTER (8 NEURONS) · MASK 0b${mask.toString(2).padStart(8, "0")}`, 16, 24);

  for (let n = 0; n < 8; n++) {
    const spiked = ((mask >>> n) & 1) === 1;
    const bx = 16 + n * 68;
    ctx.fillStyle = spiked ? pal.success : pal.card;
    ctx.fillRect(bx, 34, 58, 28);
    ctx.strokeStyle = spiked ? pal.success : pal.border;
    ctx.strokeRect(bx, 34, 58, 28);
    ctx.fillStyle = spiked ? pal.primaryFg : pal.fg;
    ctx.fillText(`N${n}:${spiked ? "1" : "0"}`, bx + 12, 52);
  }

  ctx.fillStyle = pal.mutedFg;
  ctx.fillText(
    `NPC & PLAYER MUTUAL-ATTENTION SALIENCE (ATTENDED SLOT ${attended} · GIMBAL ${tel?.gimbalYawMilli ?? 0} mrad)`,
    16,
    88,
  );

  const maxSal = Math.max(500, ...salience.map((v) => Math.abs(v)));
  for (let s = 0; s < salience.length; s++) {
    const val = Math.max(0, salience[s]);
    const barW = Math.max(12, Math.round((val / maxSal) * (w - 170)));
    const y = 100 + s * 22;
    ctx.fillStyle = pal.mutedFg;
    ctx.fillText(`Slot ${s}`, 16, y + 13);
    ctx.fillStyle = s === attended ? pal.success : pal.brand;
    ctx.fillRect(82, y, barW, 15);
    ctx.fillStyle = pal.fg;
    ctx.fillText(`${val}`, 90 + barW, y + 13);
  }
}

export function drawAudioSpectrumCanvas(canvas, fundamentalHz, rt60Ms, tel) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readThemePalette();
  const w = canvas.width;
  const h = canvas.height;
  ctx.fillStyle = pal.bg;
  ctx.fillRect(0, 0, w, h);

  const pan = tel?.binauralPan ?? 0;
  const gain = tel?.binauralGain ?? 0.62;
  const itd = tel?.binauralItdMicros ?? 0;

  ctx.fillStyle = pal.fg;
  ctx.font = `600 12px ${pal.fontMono}`;
  ctx.fillText(
    `EULER-BERNOULLI 5-MODE HARMONIC SPECTRUM · RT60 ${(rt60Ms / 1000).toFixed(2)}s · ITD ${Math.round(itd)} µs`,
    14,
    22,
  );

  const numers = PRISM_MODAL_BAR_NUMERS;
  const denom = PRISM_MODAL_BAR_DENOM;
  const maxFreq = (fundamentalHz * numers[numers.length - 1]) / denom;

  for (let i = 0; i < numers.length; i++) {
    const freq = (fundamentalHz * numers[i]) / denom;
    const x = 22 + i * 108;
    const barH = Math.max(24, Math.round((freq / Math.max(1, maxFreq)) * 96));
    const y = 140 - barH;
    ctx.fillStyle = i === 0 ? pal.success : pal.brand;
    ctx.fillRect(x, y, 76, barH);
    ctx.strokeStyle = pal.border;
    ctx.strokeRect(x, y, 76, barH);
    ctx.fillStyle = pal.fg;
    ctx.font = `600 11px ${pal.fontMono}`;
    ctx.fillText(`M${i} (${numers[i]}/${denom})`, x + 4, 156);
    ctx.fillStyle = pal.mutedFg;
    ctx.fillText(`${freq.toFixed(1)} Hz`, x + 8, y - 6);
  }

  const cx = Math.round(w / 2);
  ctx.strokeStyle = pal.border;
  ctx.strokeRect(22, 168, w - 44, 14);
  const px = Math.max(26, Math.min(w - 30, Math.round(cx + pan * ((w - 60) / 2))));
  ctx.fillStyle = pal.primary;
  ctx.fillRect(px - 6, 169, 12, 12);
  ctx.fillStyle = pal.mutedFg;
  ctx.fillText(`L`, 10, 179);
  ctx.fillText(`R`, w - 18, 179);
  ctx.fillText(`Gain ${gain.toFixed(2)}`, cx - 30, 196);
}

export function drawUiComplianceCanvas(canvas, vw, vh, reportText) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pal = readThemePalette();
  const cw = canvas.width;
  const ch = canvas.height;
  ctx.fillStyle = pal.bg;
  ctx.fillRect(0, 0, cw, ch);

  const lines = String(reportText || "").split("\n").slice(1);
  const sx = (cw - 24) / Math.max(1, vw);
  const sy = (ch - 36) / Math.max(1, vh);

  ctx.fillStyle = pal.fg;
  ctx.font = `600 12px ${pal.fontMono}`;
  ctx.fillText(`TROPICAL (max,+) + EXACT LINEAR UI PROOF · ${vw}x${vh} CSS PX`, 12, 20);

  lines.forEach((line, idx) => {
    const [label, rest] = line.split(":");
    if (!rest) return;
    const parts = rest.split(",");
    const x = Number(parts[0]) || 0;
    const y = Number(parts[1]) || 0;
    const [rw, rh] = (parts[2] || "48x48").split("x").map(Number);
    const rx = 12 + x * sx;
    const ry = 28 + y * sy;
    const pw = Math.max(18, rw * sx);
    const ph = Math.max(14, rh * sy);
    ctx.fillStyle = pal.card;
    ctx.fillRect(rx, ry, pw, ph);
    ctx.strokeStyle = idx % 2 === 0 ? pal.success : pal.brand;
    ctx.strokeRect(rx, ry, pw, ph);
    ctx.fillStyle = pal.fg;
    ctx.font = `11px ${pal.fontMono}`;
    ctx.fillText(`${label} (${rw}x${rh})`, rx + 4, ry + 12);
  });
}

export function downloadTextOrBytes(filename, content, mime = "text/plain;charset=utf-8") {
  const blob = content instanceof Uint8Array ? new Blob([content], { type: mime }) : new Blob([String(content)], { type: mime });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1500);
}


export async function exportGlbDownload(callWorker, lodShort) {
  const r = await callWorker("export-glb");
  if (!r?.buffer) return;
  const url = URL.createObjectURL(new Blob([r.buffer], { type: "model/gltf-binary" }));
  const a = document.createElement("a");
  a.href = url;
  a.download = `sunspire-scene-${String(lodShort || "lod3").toLowerCase()}.glb`;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1500);
}

export async function shareUorcSnapshot(callWorker, chip, statusEl) {
  const r = await callWorker("encode-snapshot");
  if (!r?.ok || !r.bytes) {
    chip(statusEl, "Snapshot unavailable", "warn");
    return;
  }
  const bytes = r.bytes instanceof Uint8Array ? r.bytes : new Uint8Array(r.bytes);
  history.replaceState(null, "", `${location.pathname}${location.search}#uorc=${bytesToBase64Url(bytes)}`);
  try {
    await navigator.clipboard?.writeText(location.href);
  } catch {}
  chip(statusEl, `Saved UORC · ${bytes.byteLength}B SHA-256 (#uorc=)`, "good");
}

let audioState = { ctx: null, worklet: null, timer: 0, on: false, strikeIdx: 0 };

export function strikeSingleModePreview(modeIdx, fundamentalHz = 110.0, rt60Ms = PRISM_CHAMBER_SABINE_RT60_MS) {
  const Ctx = window.AudioContext || window.webkitAudioContext;
  if (!Ctx) return;
  audioState.ctx ??= new Ctx();
  audioState.ctx.resume?.();
  if (audioState.worklet && audioState.on) {
    audioState.worklet.port.postMessage({ kind: "strike", modeIndex: modeIdx });
    return;
  }
  const numer = PRISM_MODAL_BAR_NUMERS[Math.abs(modeIdx) % PRISM_MODAL_BAR_NUMERS.length];
  const freq = Math.min(1480.0, (fundamentalHz * numer) / PRISM_MODAL_BAR_DENOM);
  const rt = Math.max(0.35, Math.min(3.5, rt60Ms / 1000.0));
  const now = audioState.ctx.currentTime;
  const osc = audioState.ctx.createOscillator();
  const gain = audioState.ctx.createGain();
  osc.frequency.setValueAtTime(freq, now);
  gain.gain.setValueAtTime(0.0001, now);
  gain.gain.exponentialRampToValueAtTime(0.11, now + 0.018);
  gain.gain.exponentialRampToValueAtTime(0.0001, now + rt);
  osc.connect(gain).connect(audioState.ctx.destination);
  osc.start(now);
  osc.stop(now + rt + 0.05);
}

export function syncModalAudioSpatial(pan, gain, itdMicros = 0, pavilionContact = false, attendedSlot = 0) {
  if (audioState.worklet && audioState.on) {
    audioState.worklet.port.postMessage({ kind: "spatial", pan, gain, itdMicros, pavilionContact, attendedSlot });
  }
}

export function suspendModalAudio(audioBtn) {
  if (!audioState.on) return;
  audioState.on = false;
  clearInterval(audioState.timer);
  audioState.ctx?.suspend?.();
  if (audioBtn) {
    audioBtn.setAttribute("aria-pressed", "false");
    audioBtn.textContent = "Audio: Off";
  }
}

export async function toggleModalAudio(audioBtn, acousticSpec, getSolvedEye) {
  const Ctx = window.AudioContext || window.webkitAudioContext;
  if (!Ctx) return;
  audioState.on = !audioState.on;
  if (audioBtn) {
    audioBtn.setAttribute("aria-pressed", String(audioState.on));
    audioBtn.textContent = audioState.on ? "Audio: On" : "Audio: Off";
  }
  const strike = () => {
    if (!audioState.ctx || !audioState.on) return;
    if (audioState.worklet) {
      audioState.worklet.port.postMessage({ kind: "strike", modeIndex: audioState.strikeIdx++ });
      return;
    }
    const eye = getSolvedEye();
    const dist = Math.hypot(eye[0] + 9.0, eye[1] - 2.0, eye[2] + 4.0);
    const atten = Math.min(0.14, 0.9 / (1.0 + 0.18 * dist));
    const idx = (audioState.strikeIdx++) % acousticSpec.modeNumers.length;
    const freq = Math.min(1320.0, (acousticSpec.fundamentalHz * acousticSpec.modeNumers[idx]) / acousticSpec.modeDenom);
    const rt = Math.max(0.5, acousticSpec.rt60Ms / 1000.0);
    const now = audioState.ctx.currentTime;
    const osc = audioState.ctx.createOscillator();
    const gain = audioState.ctx.createGain();
    osc.frequency.setValueAtTime(freq, now);
    gain.gain.setValueAtTime(0.0001, now);
    gain.gain.exponentialRampToValueAtTime(atten, now + 0.02);
    gain.gain.exponentialRampToValueAtTime(0.0001, now + rt);
    osc.connect(gain).connect(audioState.ctx.destination);
    osc.start(now);
    osc.stop(now + rt + 0.05);
  };
  if (audioState.on) {
    audioState.ctx ??= new Ctx();
    await audioState.ctx.resume?.();
    if (!audioState.worklet && audioState.ctx.audioWorklet && typeof AudioWorkletNode === "function") {
      try {
        await audioState.ctx.audioWorklet.addModule(new URL("./audio-worklet.mjs", import.meta.url).href);
        audioState.worklet = new AudioWorkletNode(audioState.ctx, "verdant-modal-processor", {
          numberOfOutputs: 1,
          outputChannelCount: [acousticSpec.stereoChannels],
          processorOptions: {
            quantumFrames: acousticSpec.quantumFrames,
            modeNumers: acousticSpec.modeNumers,
            modeDenom: acousticSpec.modeDenom,
            rt60Ms: acousticSpec.rt60Ms,
            fundamentalHz: acousticSpec.fundamentalHz,
          },
        });
        audioState.worklet.connect(audioState.ctx.destination);
      } catch {
        audioState.worklet = null;
      }
    }
    clearInterval(audioState.timer);
    audioState.timer = window.setInterval(strike, 1350);
    strike();
  } else {
    clearInterval(audioState.timer);
  }
}

export async function routeStudioPrompt(text, ctx) {
  const prompt = String(text || "").trim();
  if (!prompt) {
    ctx.chip?.(ctx.statusEl, "Refused empty intent", "warn");
    return null;
  }
  const [r] = await Promise.all([
    ctx.callWorker("route-prompt", { prompt }),
    ctx.queryVerifier?.("scene:cybernetics"),
  ]);
  if (!r?.ok) {
    const reason = r?.organDenied
      ? `Refused: ContainerOrganDenied(${r.organDenied})`
      : "Refused ungrounded intent";
    ctx.chip?.(ctx.statusEl, reason, "warn");
    return r;
  }

  switch (r.slug) {
    case "open-role-code":
      await ctx.launchStudioModal("code");
      break;
    case "open-role-dashboard":
      await ctx.launchStudioModal("dashboard");
      break;
    case "open-role-design":
    case "inspect-formal-ui":
      await ctx.launchStudioModal("design");
      break;
    case "open-role-audio":
      await ctx.launchStudioModal("audio");
      break;
    case "open-role-director":
      await ctx.launchStudioModal("director");
      break;
    case "open-role-gameplay":
      await ctx.launchStudioModal("gameplay");
      break;
    case "narrative-interact":
      await ctx.interactChronicle();
      break;
    case "focus-observer":
      await ctx.focusNodePreset(0);
      ctx.setSimLive(true);
      break;
    case "drive-vehicle":
      await ctx.focusNodePreset(6);
      ctx.setSimLive(true);
      break;
    case "render-tile-wasm":
      await ctx.toggleRenderBackend("tile");
      break;
    case "render-webgl2":
      await ctx.toggleRenderBackend("webgl2");
      break;
    case "stream-uorc-lod3":
      await ctx.streamUorcToRank(PRISM_MAX_LOD_RANK);
      break;
    case "stream-uorc-lod4":
      await ctx.streamUorcToRank(PRISM_HD_MAX_LOD_RANK);
      break;
    case "stitch-shard-sync":
      if (ctx.openStitchModal) await ctx.openStitchModal();
      else (await import("./stitch-mesh.mjs")).openStitchMeshModal({ callWorker: ctx.callWorker, onRestoreSnapshot: ctx.restoreRawSnapshot });
      break;
    case "step-xpbd-physics":
      await ctx.focusNodePreset(3);
      ctx.setSimLive(true);
      break;
    default:
      break;
  }

  ctx.chip?.(ctx.statusEl, `Director -> ${r.slug} (${(r.scoreMilli / 10).toFixed(0)}% coded_gemm)`, "good");
  return r;
}
