// Verdant Engine — Authoritative Live Internet Catalog Stream Discovery & Auto-Tagging Engine.
// Extrapolates content types strictly from authoritative standards (Schema.org v28.1, IANA Media
// Types, Khronos glTF 2.0, W3C DTCG, W3C CSS Fonts 4, W3C Web Audio, OGC GeoJSON, ISO 24617-2,
// and SPDX) and fans out unified live discovery searches across CORS-enabled public internet
// catalog streams alongside Verdant's SHA-256-locked local Prism authority catalog.

import { PRISM_MODEL_CATALOGS } from "./prism-catalogs.mjs";
import { buildLocalVerdantCatalogItemsImpl } from "./catalog-streams-local.mjs";

export const AUTHORITATIVE_CONTENT_TYPES = Object.freeze([
  {
    key: "all",
    label: "All Types",
    schemaType: "Thing",
    mediaType: "*/*",
    targetSlot: "studio:any",
    targetRole: "dashboard",
    standard: "Schema.org v28.1 + IANA",
  },
  {
    key: "3DModel",
    label: "3D Models & Rigs",
    schemaType: "3DModel",
    mediaType: "model/gltf-binary",
    targetSlot: "design:3d-model",
    targetRole: "design",
    standard: "Khronos glTF 2.0 · Schema.org/3DModel",
  },
  {
    key: "PBRMaterial",
    label: "Materials, HDRIs & Art",
    schemaType: "CreativeWork",
    mediaType: "image/png",
    targetSlot: "design:material",
    targetRole: "design",
    standard: "Khronos PBR / MaterialX · IANA image/*",
  },
  {
    key: "DefinedTermSet",
    label: "Themes & DTCG Tokens",
    schemaType: "DefinedTermSet",
    mediaType: "application/tokens+json",
    targetSlot: "design:theme",
    targetRole: "design",
    standard: "W3C DTCG 1.0 · Schema.org/DefinedTermSet",
  },
  {
    key: "Font",
    label: "Fonts & Typography",
    schemaType: "CreativeWork",
    mediaType: "font/woff2",
    targetSlot: "design:typography",
    targetRole: "design",
    standard: "W3C CSS Fonts Level 4 · IANA font/woff2",
  },
  {
    key: "AudioObject",
    label: "Audio & Acoustics",
    schemaType: "AudioObject",
    mediaType: "audio/mpeg",
    targetSlot: "audio:soundscape",
    targetRole: "audio",
    standard: "W3C Web Audio · ISO 3382-1 · Schema.org/AudioObject",
  },
  {
    key: "Place",
    label: "Worlds, Maps & Dungeons",
    schemaType: "Place",
    mediaType: "application/geo+json",
    targetSlot: "gameplay:world-map",
    targetRole: "gameplay",
    standard: "OGC GeoJSON · Schema.org/Place",
  },
  {
    key: "Dataset",
    label: "Ontologies & Reasoning",
    schemaType: "Dataset",
    mediaType: "application/ld+json",
    targetSlot: "narrative:ontology",
    targetRole: "narrative",
    standard: "W3C JSON-LD · Wikidata · OMG DMN 1.5",
  },
  {
    key: "Story",
    label: "Narrative & Characters",
    schemaType: "CreativeWork",
    mediaType: "text/markdown",
    targetSlot: "narrative:story",
    targetRole: "narrative",
    standard: "ISO 24617-2 · Schema.org/CreativeWork",
  },
  {
    key: "SoftwareSourceCode",
    label: "Engines, Shaders & Code",
    schemaType: "SoftwareSourceCode",
    mediaType: "application/wasm",
    targetSlot: "code:behavior",
    targetRole: "code",
    standard: "W3C WGSL · WebAssembly · SPDX",
  },
]);

export const LIVE_CATALOG_STREAMS = Object.freeze([
  {
    id: "khronos-gltf",
    label: "Khronos glTF-Sample-Assets",
    authority: "Khronos Group",
    basisPrime: 13,
    organIri: "urn:verdant:sensory:khronos-gltf",
    sensoryModality: "Spatial 3D Geometry, PBR & Articulated Rigs",
    schemaTypes: ["3DModel"],
    mediaTypes: ["model/gltf-binary"],
    endpoint: "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/model-index.json",
    contentTypes: ["3DModel"],
    targetParts: ["character-rigs", "hero-props"],
  },
  {
    id: "nasa-3d",
    label: "NASA 3D Resources (257+ GLB)",
    authority: "NASA Open Data · Public Domain",
    basisPrime: 17,
    organIri: "urn:verdant:sensory:nasa-3d",
    sensoryModality: "Aerospace Craft, Rovers, Astronauts & Planetary Models",
    schemaTypes: ["3DModel"],
    mediaTypes: ["model/gltf-binary"],
    endpoint: "https://api.github.com/repos/nasa/NASA-3D-Resources/git/trees/master?recursive=1",
    contentTypes: ["3DModel", "Place"],
    targetParts: ["hero-props", "stage-environment"],
  },
  {
    id: "threejs-gltf",
    label: "Three.js Open Rigged Characters & GLB",
    authority: "Open Web glTF Showcase",
    basisPrime: 19,
    organIri: "urn:verdant:sensory:threejs-gltf",
    sensoryModality: "Rigged Humanoid Characters, Creatures & Look-Dev Assets",
    schemaTypes: ["3DModel"],
    mediaTypes: ["model/gltf-binary"],
    endpoint: "https://api.github.com/repos/mrdoob/three.js/contents/examples/models/gltf",
    contentTypes: ["3DModel"],
    targetParts: ["character-rigs", "hero-props"],
  },
  {
    id: "babylon-assets",
    label: "BabylonJS Open 3D Characters & Props",
    authority: "BabylonJS Open Assets",
    basisPrime: 23,
    organIri: "urn:verdant:sensory:babylon-assets",
    sensoryModality: "Animated Characters, Creatures, Aircraft & PBR Props",
    schemaTypes: ["3DModel"],
    mediaTypes: ["model/gltf-binary"],
    endpoint: "https://api.github.com/repos/BabylonJS/Assets/contents/meshes",
    contentTypes: ["3DModel"],
    targetParts: ["character-rigs", "hero-props"],
  },
  {
    id: "polyhaven",
    label: "Poly Haven CC0 (4K PBR, HDRIs & 3D)",
    authority: "Poly Haven",
    basisPrime: 29,
    organIri: "urn:verdant:sensory:polyhaven",
    sensoryModality: "Photorealistic Panoramic Sky Domes (HDRIs), PBR & Props",
    schemaTypes: ["3DModel", "CreativeWork"],
    mediaTypes: ["model/gltf+json", "image/png"],
    endpoint: "https://api.polyhaven.com/assets",
    contentTypes: ["3DModel", "PBRMaterial", "Place"],
    targetParts: ["stage-environment", "surface-materials", "hero-props"],
  },
  {
    id: "fontsource",
    label: "Fontsource Open Fonts",
    authority: "Fontsource / Google Fonts",
    basisPrime: 31,
    organIri: "urn:verdant:sensory:fontsource",
    sensoryModality: "W3C CSS Fonts Level 4 Typography & Legibility",
    schemaTypes: ["CreativeWork"],
    mediaTypes: ["font/woff2"],
    endpoint: "https://api.fontsource.org/v1/fonts",
    contentTypes: ["Font"],
    targetParts: ["visual-identity"],
  },
  {
    id: "openverse",
    label: "Openverse Creative Commons",
    authority: "Openverse / WordPress Foundation",
    basisPrime: 37,
    organIri: "urn:verdant:sensory:openverse",
    sensoryModality: "Spatial Audio Soundscapes, Foley & Visual Artwork",
    schemaTypes: ["AudioObject", "CreativeWork"],
    mediaTypes: ["audio/mpeg", "audio/ogg", "image/jpeg"],
    endpoint: "https://api.openverse.org/v1/",
    contentTypes: ["AudioObject", "PBRMaterial"],
    targetParts: ["spatial-acoustics", "surface-materials"],
  },
  {
    id: "wikidata",
    label: "Wikidata + Wikipedia Knowledge Streams",
    authority: "Wikimedia Foundation",
    basisPrime: 41,
    organIri: "urn:verdant:sensory:wikidata",
    sensoryModality: "Structured World & Natural-Language Epistemic Passages (ISO 24617-2)",
    schemaTypes: ["Dataset", "Place"],
    mediaTypes: ["application/ld+json"],
    endpoint: "https://www.wikidata.org/w/api.php",
    contentTypes: ["Dataset", "Place", "Story"],
    targetParts: ["cognitive-language", "stage-environment"],
  },
  {
    id: "github",
    label: "GitHub Open Source Registry",
    authority: "GitHub Public Index",
    basisPrime: 43,
    organIri: "urn:verdant:sensory:github",
    sensoryModality: "Executable Engines, WGSL Shaders, Maps & DTCG Themes",
    schemaTypes: ["SoftwareSourceCode", "DefinedTermSet", "Place"],
    mediaTypes: ["application/wasm", "application/tokens+json", "application/geo+json"],
    endpoint: "https://api.github.com/search/repositories",
    contentTypes: ["SoftwareSourceCode", "DefinedTermSet", "Place"],
    targetParts: ["runtime-frameworks", "visual-identity"],
  },
  {
    id: "verdant-prism",
    label: "Verdant Locked Prism Catalogs",
    authority: "UOR Foundation · SHA-256 Locked",
    basisPrime: 47,
    organIri: "urn:verdant:sensory:verdant-prism",
    sensoryModality: "SHA-256-Locked World Packs, Standards & Proof Boundary",
    schemaTypes: ["3DModel", "DefinedTermSet", "AudioObject", "Place", "Dataset", "SoftwareSourceCode"],
    mediaTypes: ["application/octet-stream", "application/ld+json"],
    endpoint: "../catalogs/world-packs.json",
    contentTypes: ["3DModel", "PBRMaterial", "DefinedTermSet", "Font", "AudioObject", "Place", "Dataset", "Story", "SoftwareSourceCode"],
    targetParts: ["stage-environment", "runtime-frameworks", "visual-identity", "spatial-acoustics"],
  },
]);

const FEATURED_KHRONOS_ORDER = [
  "DamagedHelmet",
  "Fox",
  "FlightHelmet",
  "CesiumMan",
  "Lantern",
  "AntiqueCamera",
  "BoomBox",
  "WaterBottle",
  "Avocado",
  "Duck",
  "ToyCar",
  "ABeautifulGame",
];

const FEATURED_NASA_KEYWORDS = [
  "Apollo Lunar Module",
  "Advanced Crew Escape Suit",
  "Perseverance",
  "Curiosity",
  "James Webb",
  "International Space Station",
  "Ingenuity",
  "Voyager",
  "70 meter dish",
  "Hubble",
];

const FEATURED_THREEJS_ORDER = [
  "Soldier.glb",
  "RobotExpressive.glb",
  "Xbot.glb",
  "Michelle.glb",
  "Horse.glb",
  "Flamingo.glb",
  "Parrot.glb",
  "CarbonFrameBike.glb",
  "PrimaryIonDrive.glb",
  "DragonAttenuation.glb",
  "AnisotropyBarnLamp.glb",
];

const FEATURED_BABYLON_ORDER = [
  "HVGirl.glb",
  "YetiSmall.glb",
  "aerobatic_plane.glb",
  "Bee.glb",
  "SheenChair.glb",
  "D20_Animation.glb",
  "CarbonFiberWheel.glb",
  "ExplodingBarrel.glb",
];

const KHRONOS_BASE = "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/";

const indexCache = {
  khronos: null,
  nasa3d: null,
  threejs: null,
  babylon: null,
  polyhavenModels: null,
  polyhavenTextures: null,
  polyhavenHdris: null,
  fontsource: null,
  custom: new Map(),
};

function matchesTokens(haystack, queryTokens) {
  if (queryTokens.length === 0) return true;
  const low = haystack.toLowerCase();
  return queryTokens.every((t) => low.includes(t));
}

async function fetchJsonSafe(url, timeoutMs = 5500) {
  const res = await fetch(url, {
    signal: AbortSignal.timeout(timeoutMs),
    headers: { Accept: "application/json" },
  });
  if (!res.ok) throw new Error(`HTTP ${res.status}`);
  return res.json();
}

// ==================== STREAM ADAPTERS ====================

async function queryKhronosGltf(tokens, limit, timeoutMs) {
  if (!indexCache.khronos) {
    const raw = await fetchJsonSafe(
      "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/model-index.json",
      timeoutMs,
    );
    const list = Array.isArray(raw) ? raw : [];
    const rank = (name) => {
      const i = FEATURED_KHRONOS_ORDER.indexOf(name);
      return i === -1 ? 999 : i;
    };
    list.sort((a, b) => rank(a?.name) - rank(b?.name));
    indexCache.khronos = list;
  }
  const out = [];
  for (const m of indexCache.khronos) {
    const glbFile = m?.variants?.["glTF-Binary"];
    if (!glbFile) continue;
    const hay = `${m.label || ""} ${m.name || ""} ${(m.tags || []).join(" ")}`;
    if (!matchesTokens(hay, tokens)) continue;
    const glbUrl = `${KHRONOS_BASE}${encodeURIComponent(m.name)}/glTF-Binary/${encodeURIComponent(glbFile)}`;
    const thumbUrl = m.screenshot ? `${KHRONOS_BASE}${encodeURIComponent(m.name)}/${m.screenshot}` : "";
    out.push({
      id: `khronos:${m.name}`,
      title: m.label || m.name,
      description: `Khronos Group glTF 2.0 reference 3D model (${Object.keys(m.variants || {}).join(", ")}). Live-streamable directly into Verdant Engine WebGL2 + UORC.`,
      contentTypeKey: "3DModel",
      schemaType: "3DModel",
      mediaType: "model/gltf-binary",
      targetSlot: "design:3d-model",
      targetRole: "design",
      streamId: "khronos-gltf",
      streamLabel: "Khronos glTF",
      sourceUri: glbUrl,
      previewUrl: thumbUrl,
      glbUrl,
      licenseSpdx: "CC-BY-4.0",
      tags: ["khronos", "gltf-2.0", "glb", ...(m.tags || []).slice(0, 5)],
      properties: {
        assetName: m.name,
        scale: 1,
        lodTarget: 3,
        variants: Object.keys(m.variants || {}),
        glbStreamUrl: glbUrl,
      },
    });
    if (out.length >= limit) break;
  }
  return out;
}

async function queryNasa3d(tokens, limit, timeoutMs) {
  if (!indexCache.nasa3d) {
    const raw = await fetchJsonSafe(
      "https://api.github.com/repos/nasa/NASA-3D-Resources/git/trees/master?recursive=1",
      timeoutMs,
    );
    const glbs = (raw?.tree || []).filter((e) => typeof e?.path === "string" && e.path.toLowerCase().endsWith(".glb"));
    const rank = (p) => {
      const i = FEATURED_NASA_KEYWORDS.findIndex((kw) => p.toLowerCase().includes(kw.toLowerCase()));
      return i === -1 ? 999 : i;
    };
    glbs.sort((a, b) => rank(a.path) - rank(b.path));
    indexCache.nasa3d = glbs;
  }
  const out = [];
  for (const e of indexCache.nasa3d) {
    const baseName = e.path.split("/").pop().replace(/\.glb$/i, "");
    const hay = `${baseName} ${e.path} nasa aerospace spacecraft rover`;
    if (!matchesTokens(hay, tokens)) continue;
    const encodedPath = e.path.split("/").map((seg) => encodeURIComponent(seg)).join("/");
    const glbUrl = `https://raw.githubusercontent.com/nasa/NASA-3D-Resources/master/${encodedPath}`;
    const sizeKb = Math.max(1, Math.round((e.size || 0) / 1024));
    out.push({
      id: `nasa:${baseName.toLowerCase().replace(/[^a-z0-9]+/g, "-")}`,
      title: `${baseName} (NASA)`,
      description: `NASA Public Domain 3D model (${sizeKb} KB .glb). Directly streamable & texture-mapped inside Verdant Engine.`,
      contentTypeKey: "3DModel",
      schemaType: "3DModel",
      mediaType: "model/gltf-binary",
      targetSlot: /suit|astronaut/i.test(baseName) ? "character:rig" : "design:3d-model",
      targetRole: "design",
      streamId: "nasa-3d",
      streamLabel: "NASA 3D",
      sourceUri: glbUrl,
      previewUrl: "",
      glbUrl,
      sizeBytes: e.size || 0,
      licenseSpdx: "CC0-1.0",
      tags: ["nasa", "public-domain", "glb", "aerospace"],
      properties: {
        assetPath: e.path,
        sizeBytes: e.size || 0,
        glbStreamUrl: glbUrl,
        wikipediaTopic: baseName,
      },
    });
    if (out.length >= limit) break;
  }
  return out;
}

async function queryThreeJsGltf(tokens, limit, timeoutMs) {
  if (!indexCache.threejs) {
    const raw = await fetchJsonSafe(
      "https://api.github.com/repos/mrdoob/three.js/contents/examples/models/gltf",
      timeoutMs,
    );
    const glbs = (Array.isArray(raw) ? raw : []).filter((e) => typeof e?.name === "string" && e.name.toLowerCase().endsWith(".glb"));
    const rank = (name) => {
      const i = FEATURED_THREEJS_ORDER.indexOf(name);
      return i === -1 ? 999 : i;
    };
    glbs.sort((a, b) => rank(a.name) - rank(b.name));
    indexCache.threejs = glbs;
  }
  const out = [];
  for (const e of indexCache.threejs) {
    const clean = e.name.replace(/\.glb$/i, "").replace(/([a-z])([A-Z])/g, "$1 $2");
    const isChar = /soldier|robot|xbot|michelle|horse|flamingo|parrot|stork/i.test(e.name);
    const hay = `${clean} ${e.name} ${isChar ? "character rig creature animated" : "prop pbr"}`;
    if (!matchesTokens(hay, tokens)) continue;
    const glbUrl = e.download_url || `https://raw.githubusercontent.com/mrdoob/three.js/dev/examples/models/gltf/${encodeURIComponent(e.name)}`;
    const sizeKb = Math.max(1, Math.round((e.size || 0) / 1024));
    out.push({
      id: `threejs:${e.name.replace(/\.glb$/i, "").toLowerCase()}`,
      title: `${clean}${isChar ? " (Rigged)" : ""}`,
      description: `Open Web glTF 2.0 ${isChar ? "articulated character/creature rig" : "high-definition PBR asset"} (${sizeKb} KB .glb).`,
      contentTypeKey: "3DModel",
      schemaType: "3DModel",
      mediaType: "model/gltf-binary",
      targetSlot: isChar ? "character:rig" : "design:3d-model",
      targetRole: isChar ? "gameplay" : "design",
      streamId: "threejs-gltf",
      streamLabel: "Three.js glTF",
      sourceUri: glbUrl,
      previewUrl: "",
      glbUrl,
      sizeBytes: e.size || 0,
      licenseSpdx: "MIT",
      tags: ["gltf-2.0", "glb", isChar ? "character-rig" : "pbr-prop", "open-web"],
      properties: {
        fileName: e.name,
        sizeBytes: e.size || 0,
        glbStreamUrl: glbUrl,
        riggedCharacter: isChar,
      },
    });
    if (out.length >= limit) break;
  }
  return out;
}

async function queryBabylonAssets(tokens, limit, timeoutMs) {
  if (!indexCache.babylon) {
    const raw = await fetchJsonSafe(
      "https://api.github.com/repos/BabylonJS/Assets/contents/meshes",
      timeoutMs,
    );
    const glbs = (Array.isArray(raw) ? raw : []).filter((e) => typeof e?.name === "string" && e.name.toLowerCase().endsWith(".glb"));
    const rank = (name) => {
      const i = FEATURED_BABYLON_ORDER.indexOf(name);
      return i === -1 ? 999 : i;
    };
    glbs.sort((a, b) => rank(a.name) - rank(b.name));
    indexCache.babylon = glbs;
  }
  const out = [];
  for (const e of indexCache.babylon) {
    const clean = e.name.replace(/\.glb$/i, "").replace(/[_-]+/g, " ").replace(/([a-z])([A-Z])/g, "$1 $2");
    const isChar = /girl|yeti|xbot|bee|dummy|alien/i.test(e.name);
    const hay = `${clean} ${e.name} ${isChar ? "character creature rig" : "prop vehicle pbr"} babylon`;
    if (!matchesTokens(hay, tokens)) continue;
    const glbUrl = e.download_url || `https://raw.githubusercontent.com/BabylonJS/Assets/master/meshes/${encodeURIComponent(e.name)}`;
    const sizeKb = Math.max(1, Math.round((e.size || 0) / 1024));
    out.push({
      id: `babylon:${e.name.replace(/\.glb$/i, "").toLowerCase()}`,
      title: `${clean}${isChar ? " (Character)" : ""}`,
      description: `BabylonJS Open 3D ${isChar ? "character/creature .glb" : "PBR model .glb"} (${sizeKb} KB) streamable natively in Verdant Engine.`,
      contentTypeKey: "3DModel",
      schemaType: "3DModel",
      mediaType: "model/gltf-binary",
      targetSlot: isChar ? "character:rig" : "design:3d-model",
      targetRole: isChar ? "gameplay" : "design",
      streamId: "babylon-assets",
      streamLabel: "BabylonJS 3D",
      sourceUri: glbUrl,
      previewUrl: "",
      glbUrl,
      sizeBytes: e.size || 0,
      licenseSpdx: "Apache-2.0",
      tags: ["babylon", "glb", isChar ? "character-rig" : "3d-asset"],
      properties: {
        fileName: e.name,
        sizeBytes: e.size || 0,
        glbStreamUrl: glbUrl,
        riggedCharacter: isChar,
      },
    });
    if (out.length >= limit) break;
  }
  return out;
}

async function queryCustomStream(streamDef, tokens, limit, timeoutMs) {
  const key = `${streamDef.streamId}:${streamDef.endpointUrl}`;
  if (!indexCache.custom.has(key)) {
    const raw = await fetchJsonSafe(streamDef.endpointUrl, timeoutMs);
    indexCache.custom.set(key, raw);
  }
  const raw = indexCache.custom.get(key);
  const out = [];
  const adapter = String(streamDef.adapterKind || "").toLowerCase();

  if (adapter === "khronos-index" && Array.isArray(raw)) {
    const baseDir = streamDef.endpointUrl.replace(/\/[^/]+$/, "/");
    for (const m of raw) {
      const glbFile = m?.variants?.["glTF-Binary"];
      if (!glbFile) continue;
      const hay = `${m.label || ""} ${m.name || ""} ${(m.tags || []).join(" ")}`;
      if (!matchesTokens(hay, tokens)) continue;
      const glbUrl = `${baseDir}${encodeURIComponent(m.name)}/glTF-Binary/${encodeURIComponent(glbFile)}`;
      out.push({
        id: `custom:${streamDef.streamId}:${m.name}`,
        title: m.label || m.name,
        description: `Custom Observer stream "${streamDef.label}" glTF 2.0 model.`,
        contentTypeKey: "3DModel",
        schemaType: "3DModel",
        mediaType: "model/gltf-binary",
        targetSlot: "design:3d-model",
        targetRole: "design",
        streamId: streamDef.streamId,
        streamLabel: streamDef.label,
        sourceUri: glbUrl,
        previewUrl: m.screenshot ? `${baseDir}${encodeURIComponent(m.name)}/${m.screenshot}` : "",
        glbUrl,
        licenseSpdx: "CC-BY-4.0",
        tags: ["custom-stream", streamDef.streamId, "glb"],
        properties: { glbStreamUrl: glbUrl },
      });
      if (out.length >= limit) break;
    }
  } else if (adapter === "github-glb-tree") {
    const entries = Array.isArray(raw) ? raw : Array.isArray(raw?.tree) ? raw.tree : [];
    for (const e of entries) {
      const pathStr = e.path || e.name || "";
      if (!pathStr.toLowerCase().endsWith(".glb")) continue;
      const clean = pathStr.split("/").pop().replace(/\.glb$/i, "");
      if (!matchesTokens(`${clean} ${pathStr}`, tokens)) continue;
      const glbUrl = e.download_url || streamDef.endpointUrl;
      out.push({
        id: `custom:${streamDef.streamId}:${clean.toLowerCase().replace(/[^a-z0-9]+/g, "-")}`,
        title: clean,
        description: `Custom GitHub GLB stream "${streamDef.label}" (${Math.round((e.size || 0) / 1024)} KB).`,
        contentTypeKey: "3DModel",
        schemaType: "3DModel",
        mediaType: "model/gltf-binary",
        targetSlot: "design:3d-model",
        targetRole: "design",
        streamId: streamDef.streamId,
        streamLabel: streamDef.label,
        sourceUri: glbUrl,
        previewUrl: "",
        glbUrl,
        licenseSpdx: "CC0-1.0",
        tags: ["custom-stream", streamDef.streamId, "glb"],
        properties: { glbStreamUrl: glbUrl },
      });
      if (out.length >= limit) break;
    }
  } else {
    const list = Array.isArray(raw)
      ? raw
      : Array.isArray(raw?.itemListElement)
        ? raw.itemListElement.map((x) => x.item || x)
        : Array.isArray(raw?.dataset)
          ? raw.dataset
          : Array.isArray(raw?.items)
            ? raw.items
            : [];
    for (const item of list) {
      const name = item?.name || item?.title || item?.label || "";
      if (!name || !matchesTokens(`${name} ${item?.description || ""}`, tokens)) continue;
      const url = item?.contentUrl || item?.url || streamDef.endpointUrl;
      const isGlb = typeof url === "string" && url.toLowerCase().endsWith(".glb");
      out.push({
        id: `custom:${streamDef.streamId}:${name.toLowerCase().replace(/[^a-z0-9]+/g, "-")}`,
        title: name,
        description: item?.description || `Schema.org catalog entry from "${streamDef.label}".`,
        contentTypeKey: isGlb ? "3DModel" : "Dataset",
        schemaType: item?.["@type"] || (isGlb ? "3DModel" : "Dataset"),
        mediaType: item?.encodingFormat || (isGlb ? "model/gltf-binary" : "application/ld+json"),
        targetSlot: isGlb ? "design:3d-model" : "narrative:ontology",
        targetRole: isGlb ? "design" : "narrative",
        streamId: streamDef.streamId,
        streamLabel: streamDef.label,
        sourceUri: url,
        previewUrl: item?.thumbnailUrl || item?.image || "",
        glbUrl: isGlb ? url : "",
        licenseSpdx: "CC-BY-4.0",
        tags: ["custom-stream", streamDef.streamId],
        properties: isGlb ? { glbStreamUrl: url } : { groundedFact: item?.description || name },
      });
      if (out.length >= limit) break;
    }
  }
  return out;
}

/**
 * Fetches a live natural-language encyclopedic summary passage from Wikipedia REST API (`ACAO=*`)
 * so `ObserverPrismHolon` can ground ISO 24617-2 NPC cognition on real internet knowledge.
 */
export async function fetchLiveKnowledgePassage(topic, timeoutMs = 5000) {
  const clean = String(topic || "").trim().replace(/\s+/g, "_");
  if (!clean) return null;
  const url = `https://en.wikipedia.org/api/rest_v1/page/summary/${encodeURIComponent(clean)}`;
  const data = await fetchJsonSafe(url, timeoutMs);
  if (!data?.extract) return null;
  return {
    sourceStream: "wikidata",
    sourceUri: data?.content_urls?.desktop?.page || url,
    topic: data.title || topic,
    passage: String(data.extract).trim(),
  };
}

async function queryPolyHaven(tokens, contentType, limit, timeoutMs) {
  const out = [];
  const wantModels = contentType === "all" || contentType === "3DModel";
  const wantTextures = contentType === "all" || contentType === "PBRMaterial";
  const wantHdris = contentType === "all" || contentType === "Place" || contentType === "PBRMaterial";

  if (wantHdris) {
    if (!indexCache.polyhavenHdris) {
      indexCache.polyhavenHdris = await fetchJsonSafe("https://api.polyhaven.com/assets?t=hdris", timeoutMs);
    }
    let hdriCount = 0;
    const hdriLimit = contentType === "Place" ? limit : Math.max(2, Math.floor(limit / 3));
    for (const [slug, a] of Object.entries(indexCache.polyhavenHdris || {})) {
      const hay = `${slug} ${a.name || ""} ${a.description || ""} ${(a.tags || []).join(" ")} ${(a.categories || []).join(" ")} hdri sky dome panorama`;
      if (!matchesTokens(hay, tokens)) continue;
      const envSkyUrl = `https://cdn.polyhaven.com/asset_img/primary/${encodeURIComponent(slug)}.png?width=1024`;
      out.push({
        id: `polyhaven:hdri:${slug}`,
        title: `${a.name || slug} (Sky Dome)`,
        description: a.description || `Poly Haven CC0 equirectangular HDRI sky dome (${(a.categories || []).join(" / ")}). Streamable straight into Verdant's WebGL2 skybox.`,
        contentTypeKey: "Place",
        schemaType: "Place",
        mediaType: "image/png",
        targetSlot: "gameplay:world-map",
        targetRole: "design",
        streamId: "polyhaven",
        streamLabel: "Poly Haven HDRI",
        sourceUri: `https://polyhaven.com/a/${encodeURIComponent(slug)}`,
        previewUrl: a.thumbnail_url || envSkyUrl,
        envSkyUrl,
        glbUrl: "",
        licenseSpdx: "CC0-1.0",
        tags: ["polyhaven", "cc0", "hdri-sky", ...(a.tags || []).slice(0, 4)],
        properties: {
          polyhavenSlug: slug,
          envSkyUrl,
          filesEndpoint: `https://api.polyhaven.com/files/${slug}`,
        },
      });
      hdriCount++;
      if (hdriCount >= hdriLimit) break;
    }
  }

  if (wantModels) {
    if (!indexCache.polyhavenModels) {
      indexCache.polyhavenModels = await fetchJsonSafe("https://api.polyhaven.com/assets?t=models", timeoutMs);
    }
    for (const [slug, a] of Object.entries(indexCache.polyhavenModels || {})) {
      const hay = `${slug} ${a.name || ""} ${a.description || ""} ${(a.tags || []).join(" ")} ${(a.categories || []).join(" ")}`;
      if (!matchesTokens(hay, tokens)) continue;
      out.push({
        id: `polyhaven:model:${slug}`,
        title: a.name || slug,
        description: a.description || `Poly Haven CC0 HD 3D model (${a.polycount || 0} tris, ${(a.categories || []).join(" / ")}).`,
        contentTypeKey: "3DModel",
        schemaType: "3DModel",
        mediaType: "model/gltf+json",
        targetSlot: "design:3d-model",
        targetRole: "design",
        streamId: "polyhaven",
        streamLabel: "Poly Haven CC0",
        sourceUri: `https://polyhaven.com/a/${encodeURIComponent(slug)}`,
        previewUrl: a.thumbnail_url || "",
        glbUrl: "",
        licenseSpdx: "CC0-1.0",
        tags: ["polyhaven", "cc0", "3d-model", ...(a.tags || []).slice(0, 4)],
        properties: {
          polyhavenSlug: slug,
          polycount: a.polycount || 0,
          maxResolution: a.max_resolution || [2048, 2048],
          filesEndpoint: `https://api.polyhaven.com/files/${slug}`,
          scale: 1,
        },
      });
      if (out.length >= Math.ceil(limit / (wantTextures ? 2 : 1))) break;
    }
  }

  if (wantTextures) {
    if (!indexCache.polyhavenTextures) {
      indexCache.polyhavenTextures = await fetchJsonSafe("https://api.polyhaven.com/assets?t=textures", timeoutMs);
    }
    for (const [slug, a] of Object.entries(indexCache.polyhavenTextures || {})) {
      const hay = `${slug} ${a.name || ""} ${a.description || ""} ${(a.tags || []).join(" ")} ${(a.categories || []).join(" ")}`;
      if (!matchesTokens(hay, tokens)) continue;
      out.push({
        id: `polyhaven:pbr:${slug}`,
        title: `${a.name || slug} (PBR)`,
        description: a.description || `Poly Haven CC0 PBR material set (${(a.max_resolution || [4096, 4096]).join("×")}px albedo/normal/roughness).`,
        contentTypeKey: "PBRMaterial",
        schemaType: "CreativeWork",
        mediaType: "image/png",
        targetSlot: "design:material",
        targetRole: "design",
        streamId: "polyhaven",
        streamLabel: "Poly Haven CC0",
        sourceUri: `https://polyhaven.com/a/${encodeURIComponent(slug)}`,
        previewUrl: a.thumbnail_url || "",
        glbUrl: "",
        licenseSpdx: "CC0-1.0",
        tags: ["polyhaven", "cc0", "pbr-material", ...(a.tags || []).slice(0, 4)],
        properties: {
          polyhavenSlug: slug,
          maxResolution: a.max_resolution || [4096, 4096],
          roughnessScale: 1,
          metallicScale: 0,
          filesEndpoint: `https://api.polyhaven.com/files/${slug}`,
        },
      });
      if (out.length >= limit) break;
    }
  }
  return out;
}

async function queryFontsource(tokens, limit, timeoutMs) {
  if (!indexCache.fontsource) {
    const raw = await fetchJsonSafe("https://api.fontsource.org/v1/fonts", timeoutMs);
    indexCache.fontsource = Array.isArray(raw) ? raw : [];
  }
  const out = [];
  for (const f of indexCache.fontsource) {
    const hay = `${f.id || ""} ${f.family || ""} ${f.category || ""} ${(f.subsets || []).join(" ")}`;
    if (!matchesTokens(hay, tokens)) continue;
    const cssUrl = `https://cdn.jsdelivr.net/npm/@fontsource/${encodeURIComponent(f.id)}/index.css`;
    out.push({
      id: `fontsource:${f.id}`,
      title: `${f.family} (${f.category || "sans"})`,
      description: `Open-source ${f.variable ? "variable " : ""}${f.category || "font"} family · weights ${(f.weights || []).join(", ")} · live CSS Fonts Level 4 stream.`,
      contentTypeKey: "Font",
      schemaType: "CreativeWork",
      mediaType: "font/woff2",
      targetSlot: "design:typography",
      targetRole: "design",
      streamId: "fontsource",
      streamLabel: "Fontsource",
      sourceUri: `https://fontsource.org/fonts/${encodeURIComponent(f.id)}`,
      previewUrl: cssUrl,
      glbUrl: "",
      licenseSpdx: f.license || "OFL-1.1",
      tags: ["font", f.category || "sans-serif", f.variable ? "variable" : "static"],
      properties: {
        fontId: f.id,
        fontFamily: f.family,
        category: f.category || "sans-serif",
        weights: f.weights || [400, 600],
        stylesheetUrl: cssUrl,
      },
    });
    if (out.length >= limit) break;
  }
  return out;
}

async function queryOpenverse(query, contentType, limit, timeoutMs) {
  const q = encodeURIComponent(query || "architecture");
  const out = [];
  const wantAudio = contentType === "all" || contentType === "AudioObject";
  const wantImages = contentType === "all" || contentType === "PBRMaterial";

  if (wantAudio) {
    const data = await fetchJsonSafe(`https://api.openverse.org/v1/audio/?q=${q}&page_size=${Math.min(limit, 6)}`, timeoutMs);
    for (const a of data?.results || []) {
      out.push({
        id: `openverse:audio:${a.id}`,
        title: a.title || "Openverse Audio",
        description: `Creative Commons audio asset by ${a.creator || "Community"} (${String(a.license || "cc0").toUpperCase()} · ${a.filetype || "mp3"}). Live WebAudio preview.`,
        contentTypeKey: "AudioObject",
        schemaType: "AudioObject",
        mediaType: a.filetype === "ogg" ? "audio/ogg" : "audio/mpeg",
        targetSlot: "audio:soundscape",
        targetRole: "audio",
        streamId: "openverse",
        streamLabel: "Openverse CC",
        sourceUri: a.foreign_landing_url || a.url || "",
        previewUrl: a.url || "",
        glbUrl: "",
        licenseSpdx: String(a.license || "CC0").toUpperCase(),
        tags: ["audio", "openverse", ...(a.tags || []).map((t) => t.name).filter(Boolean).slice(0, 4)],
        properties: {
          audioUrl: a.url || "",
          creator: a.creator || "",
          gain: 0.75,
          loop: true,
        },
      });
    }
  }

  if (wantImages) {
    const data = await fetchJsonSafe(`https://api.openverse.org/v1/images/?q=${q}&page_size=${Math.min(limit, 4)}`, timeoutMs);
    for (const img of data?.results || []) {
      out.push({
        id: `openverse:image:${img.id}`,
        title: img.title || "Openverse Artwork",
        description: `Creative Commons visual artwork / reference by ${img.creator || "Community"} (${String(img.license || "cc-by").toUpperCase()}).`,
        contentTypeKey: "PBRMaterial",
        schemaType: "CreativeWork",
        mediaType: "image/jpeg",
        targetSlot: "design:material",
        targetRole: "design",
        streamId: "openverse",
        streamLabel: "Openverse CC",
        sourceUri: img.foreign_landing_url || img.url || "",
        previewUrl: img.thumbnail || img.url || "",
        glbUrl: "",
        licenseSpdx: String(img.license || "CC-BY").toUpperCase(),
        tags: ["artwork", "openverse", ...(img.tags || []).map((t) => t.name).filter(Boolean).slice(0, 4)],
        properties: {
          imageUrl: img.url || "",
          creator: img.creator || "",
          tiling: [1, 1],
        },
      });
    }
  }
  return out;
}

async function queryWikidata(query, limit, timeoutMs) {
  const q = encodeURIComponent(query || "observatory");
  const data = await fetchJsonSafe(
    `https://www.wikidata.org/w/api.php?action=wbsearchentities&search=${q}&language=en&format=json&limit=${Math.min(limit, 8)}&origin=*`,
    timeoutMs,
  );
  return (data?.search || []).map((ent) => ({
    id: `wikidata:${ent.id}`,
    title: `${ent.label || ent.id} (${ent.id})`,
    description: ent.description || "Structured Wikidata ontology entity ready for ISO 24617-2 grounded NPC dialogue & narrative lore.",
    contentTypeKey: "Dataset",
    schemaType: "Dataset",
    mediaType: "application/ld+json",
    targetSlot: "narrative:ontology",
    targetRole: "narrative",
    streamId: "wikidata",
    streamLabel: "Wikidata",
    sourceUri: ent.concepturi || `https://www.wikidata.org/wiki/${ent.id}`,
    previewUrl: "",
    glbUrl: "",
    licenseSpdx: "CC0-1.0",
    tags: ["wikidata", "ontology", "iso-24617-2", ent.id],
    properties: {
      qid: ent.id,
      conceptUri: ent.concepturi || `https://www.wikidata.org/wiki/${ent.id}`,
      groundedFact: `${ent.label || ent.id}: ${ent.description || "Authoritative Wikidata concept."}`,
    },
  }));
}

async function queryGitHub(query, contentType, limit, timeoutMs) {
  const topicHint =
    contentType === "DefinedTermSet"
      ? "design-tokens"
      : contentType === "Place"
        ? "dungeon OR geojson map"
        : "wgsl OR gltf OR game-engine";
  const q = encodeURIComponent(query ? `${query} ${topicHint}` : topicHint);
  const data = await fetchJsonSafe(
    `https://api.github.com/search/repositories?q=${q}&sort=stars&per_page=${Math.min(limit, 6)}`,
    timeoutMs,
  );
  return (data?.items || []).map((repo) => {
    const isTheme = contentType === "DefinedTermSet" || /token|theme|palette/i.test(`${repo.name} ${repo.description || ""}`);
    const isMap = contentType === "Place" || /dungeon|map|geojson|terrain/i.test(`${repo.name} ${repo.description || ""}`);
    const ctKey = isTheme ? "DefinedTermSet" : isMap ? "Place" : "SoftwareSourceCode";
    const ctMeta = AUTHORITATIVE_CONTENT_TYPES.find((c) => c.key === ctKey) || AUTHORITATIVE_CONTENT_TYPES[9];
    return {
      id: `github:${repo.full_name}`,
      title: repo.full_name,
      description: `${repo.description || "Open-source repository"} · ★ ${(repo.stargazers_count || 0).toLocaleString()} · ${repo.language || "Multi-language"}`,
      contentTypeKey: ctKey,
      schemaType: ctMeta.schemaType,
      mediaType: ctMeta.mediaType,
      targetSlot: ctMeta.targetSlot,
      targetRole: ctMeta.targetRole,
      streamId: "github",
      streamLabel: "GitHub OSS",
      sourceUri: repo.html_url,
      previewUrl: "",
      glbUrl: "",
      licenseSpdx: repo.license?.spdx_id || "Apache-2.0",
      tags: ["github", ...(repo.topics || []).slice(0, 4), repo.language || "oss"].filter(Boolean),
      properties: {
        repository: repo.full_name,
        defaultBranch: repo.default_branch || "main",
        stars: repo.stargazers_count || 0,
        language: repo.language || "Rust",
      },
    };
  });
}

// Local SHA-256-locked Verdant Prism catalog stream (always instant, 100% offline-resilient, covers all 9 types).
export function buildLocalVerdantCatalogItems(localWorldPacks = [], localPrismCatalogs = PRISM_MODEL_CATALOGS) {
  return buildLocalVerdantCatalogItemsImpl(localWorldPacks, localPrismCatalogs, AUTHORITATIVE_CONTENT_TYPES);
}

/**
 * Executes a unified concurrent discovery search across all selected live internet streams
 * and Verdant's locked local Prism catalog. Gracefully absorbs offline/sandboxed external
 * fetch failures without throwing.
 */
export async function searchLiveCatalogStreams({
  query = "",
  contentType = "all",
  streamId = "all",
  localWorldPacks = [],
  localPrismCatalogs = PRISM_MODEL_CATALOGS,
  timeoutMs = 5500,
  includeRemote = true,
  disabledStreamIds = [],
  customStreams = [],
} = {}) {
  const cleanQuery = String(query || "").trim();
  const tokens = cleanQuery
    .toLowerCase()
    .split(/\s+/)
    .filter((t) => t.length > 0);

  const disabledSet = new Set(disabledStreamIds || []);
  const streamStatus = {};
  const tasks = [];

  const wantStream = (id, supportedTypes) =>
    !disabledSet.has(id) &&
    (streamId === "all" || streamId === id) &&
    (contentType === "all" || supportedTypes.includes(contentType));

  const prismMeta = LIVE_CATALOG_STREAMS.find((s) => s.id === "verdant-prism");
  // Always evaluate local SHA-256-locked Verdant Prism catalog synchronously (<2ms):
  const localItems = wantStream("verdant-prism", prismMeta.contentTypes)
    ? buildLocalVerdantCatalogItems(localWorldPacks, localPrismCatalogs).filter((item) => {
        if (contentType !== "all" && item.contentTypeKey !== contentType) return false;
        const hay = `${item.id} ${item.title} ${item.description} ${item.tags.join(" ")} ${item.contentTypeKey}`;
        return matchesTokens(hay, tokens);
      })
    : [];
  streamStatus["verdant-prism"] = {
    status: disabledSet.has("verdant-prism") ? "disabled" : "live",
    count: localItems.length,
  };

  const addRemoteTask = (id, fn) => {
    const t0 = typeof performance !== "undefined" ? performance.now() : Date.now();
    tasks.push(
      fn()
        .then((res) => {
          const ms = Math.max(1, Math.round((typeof performance !== "undefined" ? performance.now() : Date.now()) - t0));
          streamStatus[id] = { status: "live", count: res.length, latencyMs: ms };
          return res;
        })
        .catch(() => {
          const ms = Math.max(1, Math.round((typeof performance !== "undefined" ? performance.now() : Date.now()) - t0));
          streamStatus[id] = { status: "offline", count: 0, latencyMs: ms };
          return [];
        }),
    );
  };

  if (includeRemote) {
    if (wantStream("khronos-gltf", ["3DModel"])) {
      addRemoteTask("khronos-gltf", () => queryKhronosGltf(tokens, 6, timeoutMs));
    }
    if (wantStream("nasa-3d", ["3DModel", "Place"])) {
      addRemoteTask("nasa-3d", () => queryNasa3d(tokens, 6, timeoutMs));
    }
    if (wantStream("threejs-gltf", ["3DModel"])) {
      addRemoteTask("threejs-gltf", () => queryThreeJsGltf(tokens, 6, timeoutMs));
    }
    if (wantStream("babylon-assets", ["3DModel"])) {
      addRemoteTask("babylon-assets", () => queryBabylonAssets(tokens, 6, timeoutMs));
    }
    if (wantStream("polyhaven", ["3DModel", "PBRMaterial", "Place"])) {
      addRemoteTask("polyhaven", () => queryPolyHaven(tokens, contentType, 6, timeoutMs));
    }
    if (wantStream("fontsource", ["Font"])) {
      addRemoteTask("fontsource", () => queryFontsource(tokens, 5, timeoutMs));
    }
    if (wantStream("openverse", ["AudioObject", "PBRMaterial"])) {
      addRemoteTask("openverse", () => queryOpenverse(cleanQuery, contentType, 5, timeoutMs));
    }
    if (wantStream("wikidata", ["Dataset", "Place", "Story"])) {
      addRemoteTask("wikidata", () => queryWikidata(cleanQuery, 5, timeoutMs));
    }
    if (wantStream("github", ["SoftwareSourceCode", "DefinedTermSet", "Place"])) {
      addRemoteTask("github", () => queryGitHub(cleanQuery, contentType, 5, timeoutMs));
    }
    for (const cs of customStreams || []) {
      if (!cs?.enabled || disabledSet.has(cs.streamId)) continue;
      if (streamId !== "all" && streamId !== cs.streamId) continue;
      addRemoteTask(cs.streamId, () => queryCustomStream(cs, tokens, 6, timeoutMs));
    }
  }

  const remoteBatches = await Promise.all(tasks);
  const remoteItems = remoteBatches.flat();
  // Round-robin interleave across active streams when browsing "all" so every content type shines immediately:
  let combined;
  if (streamId === "all" && remoteBatches.length > 1) {
    const queues = [...remoteBatches.map((b) => [...b]), [...localItems]].filter((q) => q.length > 0);
    combined = [];
    while (queues.some((q) => q.length > 0)) {
      for (const q of queues) {
        if (q.length > 0) combined.push(q.shift());
      }
    }
  } else {
    combined = [...remoteItems, ...localItems];
  }
  const filtered = contentType === "all" ? combined : combined.filter((i) => i.contentTypeKey === contentType);

  return {
    items: filtered,
    streamStatus,
    totalCount: filtered.length,
    liveRemoteCount: remoteItems.length,
    localLockedCount: localItems.length,
  };
}

// ==================== STITCH 2-OF-3 VERDANT APP CATALOG PROTOCOL ====================
export {
  SEEDED_VERDANT_APPS,
  STITCH_APP_GENRES,
  hydrateStitchAppCatalog,
  loadStitchAppCatalog,
  publishStitchAppToCatalog,
  subscribeStitchAppCatalog,
} from "./catalog-streams-stitch.mjs";
