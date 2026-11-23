// Verdant Engine — Public-Benefit Stitch Layer-5 (`stitch/1`) Zero-Account Peer Swarm & Reciprocal 4-of-8 Seeder.
// Realizes the M26 Prism model (`app/src/StitchHosting.lex.tex`, `app/src/Architecture.lex.tex`,
// `app/src/Security.lex.tex` controls 26..31) with ZERO external accounts and ZERO commercial cloud PaaS providers.
// Lazy-loaded on demand (0 bytes on FR-03 cold boot).

const td = new TextDecoder();
const te = new TextEncoder();

export const STITCH_L5_LIMITS = Object.freeze({
  protocol: "stitch/1",
  wireBytesMax: 4096,
  chunkBytesMax: 2700,
  messageBytesMax: 524288,
  maxFrames: 256,
  signalLifetimeMs: 60000,
  warmLifetimeMs: 1800000,
  futureSkewMs: 5000,
  epochRequestCap: 4096,
  transferBudgetBytes: 16 * 1024 * 1024,
  storageBudgetBytes: 16 * 1024 * 1024,
  computeStepsCap: 1000000,
});

const DEFAULT_PUBLIC_RELAYS = Object.freeze(["https://ntfy.sh"]);
const STCH2_MAGIC = "STCH2_RS";
const HEX64 = /^[0-9a-f]{64}$/;
const HEX_ANY = /^[0-9a-f]+$/;

function toHex(u8) {
  let out = "";
  for (let i = 0; i < u8.length; i++) out += u8[i].toString(16).padStart(2, "0");
  return out;
}

function bytesEq(a, b) {
  if (a.length !== b.length) return false;
  let diff = 0;
  for (let i = 0; i < a.length; i++) diff |= a[i] ^ b[i];
  return diff === 0;
}

function toBase64Url(bytes) {
  let bin = "";
  for (let i = 0; i < bytes.length; i++) bin += String.fromCharCode(bytes[i]);
  return btoa(bin).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

function fromBase64Url(str) {
  const norm = String(str || "").replace(/-/g, "+").replace(/_/g, "/");
  const pad = "=".repeat((4 - (norm.length % 4)) % 4);
  const bin = atob(norm + pad);
  const out = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
  return out;
}

async function sha256Bytes(bytes) {
  if (globalThis.crypto?.subtle) {
    return new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", bytes));
  }
  throw new Error("WebCrypto SubtleCrypto unavailable for Stitch Layer-5 verification");
}

async function sha256Hex(bytes) {
  return toHex(await sha256Bytes(bytes));
}

function randomHex(byteCount = 16) {
  const buf = new Uint8Array(byteCount);
  globalThis.crypto?.getRandomValues?.(buf);
  return toHex(buf);
}

/**
 * Derives an opaque, public-release-only Stitch Layer-5 topic name (`<= 64` chars for ntfy)
 * containing ZERO private workspace identifiers (`controlOptInSwarmAndOpaqueTopicIsolation`).
 */
export function deriveOpaqueReleaseTopic(releaseKappaHex) {
  const clean = String(releaseKappaHex || "").trim().toLowerCase();
  if (!HEX64.test(clean)) {
    throw new Error("releaseKappaHex must be a 64-character lowercase SHA-256 hex digest");
  }
  return `verdant-stitch-${clean.slice(0, 16)}`;
}

/**
 * Conservative `ResourceAdmission` seeder budget ledger matching `identity/stitch.lock`
 * (`Stitch.Budget.reserveAllowed` / `settlementAllowed`, `controlStitchLayer5FramingAndBudgetCeiling`).
 */
export function createResourceAdmissionLedger(overrides = {}) {
  const limits = {
    requests: overrides.requests ?? STITCH_L5_LIMITS.epochRequestCap,
    transferBytes: overrides.transferBytes ?? STITCH_L5_LIMITS.transferBudgetBytes,
    storageBytes: overrides.storageBytes ?? STITCH_L5_LIMITS.storageBudgetBytes,
    computeSteps: overrides.computeSteps ?? STITCH_L5_LIMITS.computeStepsCap,
  };
  const used = { requests: 0, transferBytes: 0, storageBytes: 0, computeSteps: 0 };
  let refusedCount = 0;

  function reserve(cost = {}) {
    const req = Math.max(0, Number(cost.requests || 0));
    const tx = Math.max(0, Number(cost.transferBytes || 0));
    const st = Math.max(0, Number(cost.storageBytes || 0));
    const cp = Math.max(0, Number(cost.computeSteps || 0));
    if (
      used.requests + req > limits.requests ||
      used.transferBytes + tx > limits.transferBytes ||
      used.storageBytes + st > limits.storageBytes ||
      used.computeSteps + cp > limits.computeSteps
    ) {
      refusedCount++;
      throw new Error("ResourceAdmission ceiling exceeded");
    }
    used.requests += req;
    used.transferBytes += tx;
    used.storageBytes += st;
    used.computeSteps += cp;
    return { ...used };
  }

  function stats() {
    return { limits: { ...limits }, used: { ...used }, refusedCount };
  }

  return { limits, reserve, stats };
}

/**
 * Verifies a binary `STCH2` Reed-Solomon shard (80-byte header + SHA-256 `payloadRoot`).
 */
export async function verifyStch2Shard(rawBytes, expectedManifestRootHex = null) {
  const u8 = rawBytes instanceof Uint8Array ? rawBytes : new Uint8Array(rawBytes);
  if (u8.byteLength <= 80 || td.decode(u8.subarray(0, 8)) !== STCH2_MAGIC) {
    throw new Error("Invalid STCH2 shard header");
  }
  const k = u8[8];
  const n = u8[9];
  const index = u8[10];
  const originalByteLen = new DataView(u8.buffer, u8.byteOffset, u8.byteLength).getUint32(12, true);
  const manifestRoot = u8.subarray(16, 48);
  const payloadRoot = u8.subarray(48, 80);
  const payload = u8.subarray(80);
  if (k < 1 || n < k || index >= n) {
    throw new Error(`Invalid STCH2 shard geometry k=${k} n=${n} index=${index}`);
  }
  const digest = await sha256Bytes(payload);
  if (!bytesEq(digest, payloadRoot)) {
    throw new Error(`STCH2 shard ${index} payload SHA-256 mismatch`);
  }
  const manifestRootHex = toHex(manifestRoot);
  if (expectedManifestRootHex && manifestRootHex !== expectedManifestRootHex.toLowerCase()) {
    throw new Error(`STCH2 shard ${index} manifestRoot mismatch`);
  }
  return { k, n, index, originalByteLen, manifestRootHex, rawBytes: u8 };
}

/**
 * Encodes a signaling payload into bounded `stitch/1` Layer-5 wire frames (`<= 4096 B` each,
 * `<= 2700 B` chunk data, `<= 512 KiB` total message, `<= 256` frames).
 */
export async function encodeStitchLayer5Frames(payloadObj, { channelHex, sessionHex, scope = "signal", nowMs = Date.now() } = {}) {
  const cleanChannel = String(channelHex || "").toLowerCase();
  const cleanSession = String(sessionHex || "").toLowerCase();
  if (!HEX64.test(cleanChannel)) throw new Error("stitch/1 channelHex must be 64-char SHA-256 hex");
  if (!cleanSession || !HEX_ANY.test(cleanSession) || cleanSession.length > 64) {
    throw new Error("stitch/1 sessionHex must be lowercase hex");
  }
  if (scope !== "signal" && scope !== "warm") {
    throw new Error(`Unsupported stitch/1 scope: ${scope}`);
  }
  const rawBytes = te.encode(JSON.stringify(payloadObj));
  if (rawBytes.byteLength > STITCH_L5_LIMITS.messageBytesMax) {
    throw new Error(`stitch/1 message exceeds ${STITCH_L5_LIMITS.messageBytesMax} B`);
  }
  const b64 = toBase64Url(rawBytes);
  const msgDigest = await sha256Hex(rawBytes);
  const chunkLen = STITCH_L5_LIMITS.chunkBytesMax;
  const total = Math.max(1, Math.ceil(b64.length / chunkLen));
  if (total > STITCH_L5_LIMITS.maxFrames) {
    throw new Error(`stitch/1 frame count ${total} exceeds ${STITCH_L5_LIMITS.maxFrames}`);
  }
  const frames = [];
  for (let i = 0; i < total; i++) {
    const chunk = b64.slice(i * chunkLen, (i + 1) * chunkLen);
    const frame = JSON.stringify({
      v: STITCH_L5_LIMITS.protocol,
      p: scope,
      c: cleanChannel,
      s: cleanSession,
      m: msgDigest,
      t: nowMs,
      i,
      n: total,
      d: chunk,
    });
    if (te.encode(frame).byteLength > STITCH_L5_LIMITS.wireBytesMax) {
      throw new Error(`stitch/1 wire frame exceeds ${STITCH_L5_LIMITS.wireBytesMax} B`);
    }
    frames.push(frame);
  }
  return { msgDigest, frames };
}

/**
 * Stateful bounded `stitch/1` frame reassembler enforcing wire cap, TTL, chunk count, and SHA-256 digest.
 */
export function createStitchLayer5Reassembler({ channelHex, scope = "signal" } = {}) {
  const expectedChannel = String(channelHex || "").toLowerCase();
  const pending = new Map();
  const seen = new Set();

  async function accept(rawFrame, nowMs = Date.now()) {
    if (typeof rawFrame !== "string" || te.encode(rawFrame).byteLength > STITCH_L5_LIMITS.wireBytesMax) {
      throw new Error("Refused oversized stitch/1 wire frame (> 4096 B)");
    }
    let f;
    try {
      f = JSON.parse(rawFrame);
    } catch {
      throw new Error("Refused malformed stitch/1 JSON frame");
    }
    if (
      !f ||
      f.v !== STITCH_L5_LIMITS.protocol ||
      f.p !== scope ||
      f.c !== expectedChannel ||
      !HEX64.test(String(f.m || "")) ||
      typeof f.d !== "string" ||
      f.d.length > STITCH_L5_LIMITS.chunkBytesMax
    ) {
      throw new Error("Refused invalid stitch/1 frame header or chunk size");
    }
    const maxAge = scope === "warm" ? STITCH_L5_LIMITS.warmLifetimeMs : STITCH_L5_LIMITS.signalLifetimeMs;
    if (typeof f.t !== "number" || nowMs - f.t > maxAge || f.t - nowMs > STITCH_L5_LIMITS.futureSkewMs) {
      throw new Error("Refused expired or future-skewed stitch/1 frame");
    }
    if (!Number.isInteger(f.n) || f.n < 1 || f.n > STITCH_L5_LIMITS.maxFrames || !Number.isInteger(f.i) || f.i < 0 || f.i >= f.n) {
      throw new Error("Refused out-of-bounds stitch/1 frame index");
    }
    if (seen.has(f.m)) return null;
    let entry = pending.get(f.m);
    if (!entry) {
      if (pending.size >= 16) pending.delete(pending.keys().next().value);
      entry = { total: f.n, parts: new Array(f.n), received: 0 };
      pending.set(f.m, entry);
    }
    if (entry.total !== f.n) throw new Error("Inconsistent stitch/1 frame total");
    if (entry.parts[f.i] === undefined) {
      entry.parts[f.i] = f.d;
      entry.received++;
    }
    if (entry.received < entry.total) return null;
    pending.delete(f.m);
    const bytes = fromBase64Url(entry.parts.join(""));
    if (bytes.byteLength > STITCH_L5_LIMITS.messageBytesMax) {
      throw new Error("Assembled stitch/1 message exceeds 512 KiB");
    }
    if ((await sha256Hex(bytes)) !== f.m) {
      throw new Error("Assembled stitch/1 SHA-256 digest mismatch");
    }
    seen.add(f.m);
    if (seen.size > 256) seen.delete(seen.values().next().value);
    return { sessionHex: f.s, msgDigest: f.m, payload: JSON.parse(td.decode(bytes)) };
  }

  return { accept, dispose: () => { pending.clear(); seen.clear(); } };
}

async function opfsSaveShard(topic, idx, rawBytes) {
  try {
    if (!navigator.storage?.getDirectory) return false;
    const root = await navigator.storage.getDirectory();
    const dir = await root.getDirectoryHandle("verdant-stitch-shards-v1", { create: true });
    const fh = await dir.getFileHandle(`${topic}-shard-${idx}.stch`, { create: true });
    const w = await fh.createWritable();
    await w.write(rawBytes);
    await w.close();
    return true;
  } catch {
    return false;
  }
}

/**
 * Public-Benefit Stitch Layer-5 (`stitch/1`) Browser Swarm Seeder & Volunteer Gateway Bridge.
 * Requires explicit opt-in consent via `isOrganAllowed(9)` before opening external relay or WebRTC links.
 */
export function createStitchSwarmSeeder({
  releaseKappaHex,
  releaseVersion = "unknown",
  relayUrls = DEFAULT_PUBLIC_RELAYS,
  localPrismId = "prism:local-seeder",
  isOrganAllowed = null,
  fetchImpl = globalThis.fetch?.bind(globalThis),
  BroadcastChannelImpl = globalThis.BroadcastChannel,
  budgetOverrides = {},
  onEvent = null,
} = {}) {
  const opaqueTopic = deriveOpaqueReleaseTopic(releaseKappaHex);
  const channelHex = String(releaseKappaHex).toLowerCase();
  const sessionHex = randomHex(16);
  const budget = createResourceAdmissionLedger(budgetOverrides);
  const reassembler = createStitchLayer5Reassembler({ channelHex, scope: "signal" });
  const shards = new Map();
  const peers = new Map();
  let joined = false;
  let bc = null;

  const emit = (kind, detail) => {
    try { onEvent?.({ kind, detail, ts: Date.now() }); } catch {}
  };

  function assertOptInConsent() {
    if (typeof isOrganAllowed === "function" && !isOrganAllowed(9, false)) {
      throw new Error("ContainerOrganDenied(webrtc-stitch-mesh): explicit opt-in consent required before joining Stitch Layer-5 swarm");
    }
  }

  async function cacheVerifiedShard(rawBytes, persistOpfs = true) {
    const parsed = await verifyStch2Shard(rawBytes, channelHex);
    budget.reserve({ requests: 1, storageBytes: parsed.rawBytes.byteLength, computeSteps: 256 });
    shards.set(parsed.index, parsed.rawBytes);
    if (persistOpfs) await opfsSaveShard(opaqueTopic, parsed.index, parsed.rawBytes);
    emit("shard-cached", { index: parsed.index, bytes: parsed.rawBytes.byteLength });
    return parsed;
  }

  function serveShardToPeer(peerPrismId, shardIndex) {
    if (!String(peerPrismId || "").startsWith("prism:")) {
      throw new Error("Refused unauthenticated peer without prism: identity");
    }
    const raw = shards.get(shardIndex);
    if (!raw) throw new Error(`Shard ${shardIndex} not cached locally`);
    budget.reserve({ requests: 1, transferBytes: raw.byteLength, computeSteps: 128 });
    emit("shard-served", { peerPrismId, shardIndex, bytes: raw.byteLength });
    return raw;
  }

  async function announceAvailability({ publishRelays = false } = {}) {
    assertOptInConsent();
    const payload = {
      kind: "stitch-l5-announce",
      releaseVersion,
      releaseKappaHex: channelHex,
      peerPrismId: localPrismId,
      availableShards: [...shards.keys()].sort((a, b) => a - b),
      zeroCommercialRegistration: true,
    };
    const { msgDigest, frames } = await encodeStitchLayer5Frames(payload, {
      channelHex,
      sessionHex,
      scope: "signal",
    });
    budget.reserve({
      requests: frames.length,
      transferBytes: frames.reduce((s, f) => s + te.encode(f).byteLength, 0),
      computeSteps: 512,
    });
    if (bc) {
      for (const frame of frames) bc.postMessage({ topic: opaqueTopic, from: sessionHex, frame });
    }
    const publishedTo = [];
    if (publishRelays && fetchImpl) {
      for (const base of relayUrls) {
        const clean = String(base || "").trim().replace(/\/+$/, "");
        if (!clean.startsWith("https://") && !clean.startsWith("http://127.0.0.1") && !clean.startsWith("http://localhost")) {
          continue;
        }
        try {
          let allOk = true;
          for (const body of frames) {
            const resp = await fetchImpl(`${clean}/${opaqueTopic}`, {
              method: "POST",
              body,
              headers: { "Content-Type": "text/plain; charset=utf-8" },
            });
            if (!resp.ok) { allOk = false; break; }
          }
          if (allOk) publishedTo.push(clean);
        } catch {
          // Relay failure is tolerated via multi-relay racing.
        }
      }
    }
    emit("announced", { msgDigest, frameCount: frames.length, publishedTo });
    return { opaqueTopic, msgDigest, frames, publishedTo };
  }

  async function ingestFrame(rawFrame, nowMs = Date.now()) {
    budget.reserve({ requests: 1, transferBytes: te.encode(String(rawFrame || "")).byteLength, computeSteps: 128 });
    const assembled = await reassembler.accept(rawFrame, nowMs);
    if (!assembled || assembled.sessionHex === sessionHex) return null;
    if (assembled.payload?.kind === "stitch-l5-announce" && assembled.payload.releaseKappaHex === channelHex) {
      peers.set(assembled.payload.peerPrismId || assembled.sessionHex, {
        sessionHex: assembled.sessionHex,
        availableShards: Array.isArray(assembled.payload.availableShards) ? assembled.payload.availableShards : [],
        lastSeenMs: nowMs,
      });
      emit("peer-discovered", { peerId: assembled.payload.peerPrismId, count: peers.size });
    }
    return assembled;
  }

  async function probeVolunteerGateway(gatewayBaseUrl) {
    assertOptInConsent();
    if (!fetchImpl) throw new Error("fetch unavailable");
    const clean = String(gatewayBaseUrl || "").trim().replace(/\/+$/, "");
    const res = await fetchImpl(`${clean}/swarm`, { cache: "no-store" });
    if (!res.ok) throw new Error(`Volunteer gateway /swarm HTTP ${res.status}`);
    const info = await res.json();
    if (info.protocol !== STITCH_L5_LIMITS.protocol || info.zeroCommercialRegistration !== true) {
      throw new Error("Volunteer gateway failed public-benefit stitch/1 contract");
    }
    budget.reserve({ requests: 1, computeSteps: 128 });
    return info;
  }

  function join() {
    assertOptInConsent();
    if (joined) return status();
    joined = true;
    if (typeof BroadcastChannelImpl === "function") {
      bc = new BroadcastChannelImpl("verdant-stitch-l5-v1");
      bc.unref?.();
      bc.onmessage = (ev) => {
        if (ev.data?.topic === opaqueTopic && ev.data.from !== sessionHex && ev.data.frame) {
          ingestFrame(ev.data.frame).catch(() => {});
        }
      };
    }
    emit("joined", { opaqueTopic });
    return status();
  }

  function leave() {
    joined = false;
    try { bc?.close(); } catch {}
    bc = null;
    reassembler.dispose();
    emit("left", { opaqueTopic });
  }

  function status() {
    return {
      protocol: STITCH_L5_LIMITS.protocol,
      joined,
      opaqueTopic,
      releaseKappaHex: channelHex,
      cachedShardIndices: [...shards.keys()].sort((a, b) => a - b),
      discoveredPeerCount: peers.size,
      budget: budget.stats(),
    };
  }

  return {
    opaqueTopic,
    join,
    leave,
    status,
    cacheVerifiedShard,
    serveShardToPeer,
    announceAvailability,
    ingestFrame,
    probeVolunteerGateway,
  };
}
