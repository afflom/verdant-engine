// Stitch 2-of-3 Verdant App Catalog Protocol (`site/catalog-streams-stitch.mjs`),
// extracted from `site/catalog-streams.mjs`.
// Governed by AUTH-ISO-42010-VIEWPOINTS (ISO/IEC/IEEE 12207 ALM + stitchAppCatalogProtocol),
// AUTH-SCHEMA-ORG (VideoGame / SoftwareApplication), AUTH-W3C-SOSA-PROVO, and afflom/stitch.

export const STITCH_APP_GENRES = Object.freeze([
  { id: "first-person-shooter", label: "First-Person Shooter", schemaType: "VideoGame", defaultDomain: "starfall-bastion" },
  { id: "mmo-universe", label: "MMO Universe", schemaType: "VideoGame", defaultDomain: "meridian-observatory" },
  { id: "indie-side-scroller", label: "Indie Side-Scroller", schemaType: "VideoGame", defaultDomain: "glasshouse-chronicle" },
  { id: "coastal-adventure", label: "3D Coastal Adventure", schemaType: "VideoGame", defaultDomain: "sunspire-regatta" },
  { id: "spatial-simulation", label: "Spatial Simulation", schemaType: "SoftwareApplication", defaultDomain: "meridian-observatory" },
]);

export const SEEDED_VERDANT_APPS = Object.freeze([
  {
    appId: "app-starfall-vanguard-fps",
    title: "Starfall Vanguard",
    genre: "first-person-shooter",
    genreLabel: "First-Person Shooter",
    schemaType: "VideoGame",
    domainSlug: "starfall-bastion",
    worldTitle: "Starfall Bastion Arena",
    summary: "Fast-paced 3D arena FPS with 64-triangle Morton meshlets, UHD 4K CAS resolve, and calibrated stone arena acoustics.",
    publisherId: "prism:verdant-curated",
    archiveFormat: ".uorc",
    uorcBytes: 2155,
    payloadSha256: "7f4e9b2a1c8d3e6f0a5b4c3d2e1f9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f",
    stitchShards: [
      { index: 0, role: "Shard A", bytes: 1120, sha256: "a1b2c3d4e5f60718293a4b5c6d7e8f90" },
      { index: 1, role: "Shard B", bytes: 1120, sha256: "b2c3d4e5f60718293a4b5c6d7e8f90a1" },
      { index: 2, role: "Parity (A ^ B)", bytes: 1120, sha256: "c3d4e5f60718293a4b5c6d7e8f90a1b2" },
    ],
    publishedAt: "2026-11-19T00:00:00Z",
    seeded: true,
  },
  {
    appId: "app-meridian-expanse-mmo",
    title: "Meridian Expanse",
    genre: "mmo-universe",
    genreLabel: "MMO Universe",
    schemaType: "VideoGame",
    domainSlug: "meridian-observatory",
    worldTitle: "Meridian Celestial Observatory",
    summary: "Multi-sector floating-origin universe featuring celestial observatories, synchronized world shards, and persistent state recovery.",
    publisherId: "prism:verdant-curated",
    archiveFormat: ".uorc",
    uorcBytes: 2480,
    payloadSha256: "3e8d1a7c9f2b4e6d8a0c2e4f6b8d0a2c4e6f8a0b2c4d6e8f0a1b2c3d4e5f6a7b",
    stitchShards: [
      { index: 0, role: "Shard A", bytes: 1284, sha256: "1f2e3d4c5b6a798897a6b5c4d3e2f100" },
      { index: 1, role: "Shard B", bytes: 1284, sha256: "2e3d4c5b6a798897a6b5c4d3e2f1001f" },
      { index: 2, role: "Parity (A ^ B)", bytes: 1284, sha256: "3d4c5b6a798897a6b5c4d3e2f1001f2e" },
    ],
    publishedAt: "2026-11-19T00:00:00Z",
    seeded: true,
  },
  {
    appId: "app-glasshouse-canopy-runner",
    title: "Glasshouse Canopy Runner",
    genre: "indie-side-scroller",
    genreLabel: "Indie Side-Scroller",
    schemaType: "VideoGame",
    domainSlug: "glasshouse-chronicle",
    worldTitle: "Glasshouse Botanical Chronicle",
    summary: "Atmospheric botanical side-scroller built with 2.5D spline traversal, conservatory glass acoustics, and grounded character dialogue.",
    publisherId: "prism:verdant-curated",
    archiveFormat: ".uorc",
    uorcBytes: 1984,
    payloadSha256: "9c4a2e6f8b0d1e3f5a7c9b1d3e5f7a9c1b3d5e7f9a0c2e4f6a8b0c2d4e6f8a0b",
    stitchShards: [
      { index: 0, role: "Shard A", bytes: 1036, sha256: "4c5b6a798897a6b5c4d3e2f1001f2e3d" },
      { index: 1, role: "Shard B", bytes: 1036, sha256: "5b6a798897a6b5c4d3e2f1001f2e3d4c" },
      { index: 2, role: "Parity (A ^ B)", bytes: 1036, sha256: "6a798897a6b5c4d3e2f1001f2e3d4c5b" },
    ],
    publishedAt: "2026-11-19T00:00:00Z",
    seeded: true,
  },
  {
    appId: "app-sunspire-beacon-trials",
    title: "Sunspire Beacon Trials",
    genre: "coastal-adventure",
    genreLabel: "3D Coastal Adventure",
    schemaType: "VideoGame",
    domainSlug: "sunspire-regatta",
    worldTitle: "Sunspire Harbor Regatta",
    summary: "Coastal 3D action trial set across the Sunspire stone arch basin with beacon harmonization, patrol hulls, and 3rd/1st person cameras.",
    publisherId: "prism:verdant-curated",
    archiveFormat: ".uorc",
    uorcBytes: 2210,
    payloadSha256: "5a1f3c7e9b2d4f6a8c0e2a4c6e8f0b2d4f6a8c0e2b4d6f8a0c2e4f6a8b0d2f4e",
    stitchShards: [
      { index: 0, role: "Shard A", bytes: 1148, sha256: "798897a6b5c4d3e2f1001f2e3d4c5b6a" },
      { index: 1, role: "Shard B", bytes: 1148, sha256: "8897a6b5c4d3e2f1001f2e3d4c5b6a79" },
      { index: 2, role: "Parity (A ^ B)", bytes: 1148, sha256: "97a6b5c4d3e2f1001f2e3d4c5b6a7988" },
    ],
    publishedAt: "2026-11-19T00:00:00Z",
    seeded: true,
  },
]);

const STITCH_APP_STORAGE_KEY = "verdant.stitchAppCatalog.v1";
const STITCH_APP_CHANNEL = "verdant-stitch-app-catalog-v1";

function bytesToHexShort(bytes, maxBytes = 16) {
  const arr = bytes instanceof Uint8Array ? bytes : new Uint8Array(bytes || []);
  let out = "";
  const n = Math.min(maxBytes, arr.byteLength);
  for (let i = 0; i < n; i++) out += arr[i].toString(16).padStart(2, "0");
  return out;
}

async function sha256Hex(bytes) {
  const arr = bytes instanceof Uint8Array ? bytes : new Uint8Array(bytes || []);
  if (globalThis.crypto?.subtle) {
    const dig = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", arr));
    return bytesToHexShort(dig, 32);
  }
  return bytesToHexShort(arr, 16).padEnd(32, "0");
}

function readPublishedStitchApps() {
  try {
    const raw = localStorage.getItem(STITCH_APP_STORAGE_KEY);
    if (!raw) return [];
    const parsed = JSON.parse(raw);
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

let cachedGeneratedStitchApps = null;

export function loadStitchAppCatalog() {
  const published = readPublishedStitchApps();
  const base = Array.isArray(cachedGeneratedStitchApps) && cachedGeneratedStitchApps.length > 0
    ? cachedGeneratedStitchApps
    : SEEDED_VERDANT_APPS;
  const seen = new Set(published.map((a) => a.appId));
  return [...published, ...base.filter((a) => !seen.has(a.appId))];
}

export async function hydrateStitchAppCatalog() {
  try {
    const res = await fetch(new URL("./generated/stitch-app-catalog.json", import.meta.url));
    if (res.ok) {
      const json = await res.json();
      if (Array.isArray(json?.apps) && json.apps.length > 0) {
        cachedGeneratedStitchApps = json.apps;
      }
    }
  } catch {
    // Graceful offline fallback to embedded SEEDED_VERDANT_APPS.
  }
  return loadStitchAppCatalog();
}

export async function publishStitchAppToCatalog({
  title,
  genre,
  domainSlug,
  worldTitle,
  summary,
  uorcBytes,
  stitchShardBuffers,
  publisherId = "prism:studio-creator",
}) {
  const cleanTitle = String(title || "").trim() || "Untitled Verdant App";
  const genreInfo = STITCH_APP_GENRES.find((g) => g.id === genre) || STITCH_APP_GENRES[0];
  const uorc = uorcBytes instanceof Uint8Array ? uorcBytes : new Uint8Array(uorcBytes || []);
  const payloadSha256 = await sha256Hex(uorc);
  const roles = ["Shard A", "Shard B", "Parity (A ^ B)"];
  const stitchShards = [];
  for (let i = 0; i < 3; i++) {
    const buf = stitchShardBuffers?.[i] ? new Uint8Array(stitchShardBuffers[i]) : new Uint8Array(0);
    const dig = await sha256Hex(buf);
    stitchShards.push({
      index: i,
      role: roles[i],
      bytes: buf.byteLength,
      sha256: dig.slice(0, 32),
    });
  }
  const slug = cleanTitle
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "")
    .slice(0, 36) || "app";
  const entry = {
    appId: `app-${slug}-${payloadSha256.slice(0, 8)}`,
    title: cleanTitle,
    genre: genreInfo.id,
    genreLabel: genreInfo.label,
    schemaType: genreInfo.schemaType,
    domainSlug: domainSlug || genreInfo.defaultDomain,
    worldTitle: worldTitle || domainSlug || genreInfo.defaultDomain,
    summary: String(summary || "").trim() || `${genreInfo.label} sealed as .uorc and distributed across 2-of-3 Stitch parity shards.`,
    publisherId,
    archiveFormat: ".uorc",
    uorcBytes: uorc.byteLength,
    payloadSha256,
    stitchShards,
    publishedAt: new Date().toISOString(),
    seeded: false,
  };
  const existing = readPublishedStitchApps().filter((a) => a.appId !== entry.appId);
  const next = [entry, ...existing].slice(0, 24);
  try {
    localStorage.setItem(STITCH_APP_STORAGE_KEY, JSON.stringify(next));
  } catch {
    // Ignore storage quota errors in ephemeral browsing modes.
  }
  try {
    if (typeof BroadcastChannel === "function") {
      const ch = new BroadcastChannel(STITCH_APP_CHANNEL);
      ch.postMessage({ kind: "stitch-app-published", entry });
      ch.close();
    }
  } catch {
    // BroadcastChannel unavailable.
  }
  return entry;
}

export function subscribeStitchAppCatalog(onUpdate) {
  if (typeof onUpdate !== "function") return () => {};
  const onStorage = (e) => {
    if (e.key === STITCH_APP_STORAGE_KEY) onUpdate(loadStitchAppCatalog());
  };
  window.addEventListener("storage", onStorage);
  let ch = null;
  try {
    if (typeof BroadcastChannel === "function") {
      ch = new BroadcastChannel(STITCH_APP_CHANNEL);
      ch.onmessage = () => onUpdate(loadStitchAppCatalog());
    }
  } catch {
    ch = null;
  }
  return () => {
    window.removeEventListener("storage", onStorage);
    try { ch?.close(); } catch {}
  };
}
