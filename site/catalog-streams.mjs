// Verdant Engine — Authoritative Live Internet Catalog Stream Discovery & Auto-Tagging Engine.
// Extrapolates content types strictly from authoritative standards (Schema.org v28.1, IANA Media
// Types, Khronos glTF 2.0, W3C DTCG, W3C CSS Fonts 4, W3C Web Audio, OGC GeoJSON, ISO 24617-2,
// and SPDX) and fans out unified live discovery searches across CORS-enabled public internet
// catalog streams alongside Verdant's SHA-256-locked local Prism authority catalog.

import { PRISM_MODEL_CATALOGS } from "./prism-catalogs.mjs";

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
  },
  {
    id: "polyhaven",
    label: "Poly Haven CC0",
    authority: "Poly Haven",
    basisPrime: 17,
    organIri: "urn:verdant:sensory:polyhaven",
    sensoryModality: "Photorealistic 4K PBR Materials, HDRIs & Props",
    schemaTypes: ["3DModel", "CreativeWork"],
    mediaTypes: ["model/gltf+json", "image/png"],
    endpoint: "https://api.polyhaven.com/assets",
    contentTypes: ["3DModel", "PBRMaterial"],
  },
  {
    id: "fontsource",
    label: "Fontsource Open Fonts",
    authority: "Fontsource / Google Fonts",
    basisPrime: 19,
    organIri: "urn:verdant:sensory:fontsource",
    sensoryModality: "W3C CSS Fonts Level 4 Typography & Legibility",
    schemaTypes: ["CreativeWork"],
    mediaTypes: ["font/woff2"],
    endpoint: "https://api.fontsource.org/v1/fonts",
    contentTypes: ["Font"],
  },
  {
    id: "openverse",
    label: "Openverse Creative Commons",
    authority: "Openverse / WordPress Foundation",
    basisPrime: 23,
    organIri: "urn:verdant:sensory:openverse",
    sensoryModality: "Spatial Audio Soundscapes, Foley & Visual Artwork",
    schemaTypes: ["AudioObject", "CreativeWork"],
    mediaTypes: ["audio/mpeg", "audio/ogg", "image/jpeg"],
    endpoint: "https://api.openverse.org/v1/",
    contentTypes: ["AudioObject", "PBRMaterial"],
  },
  {
    id: "wikidata",
    label: "Wikidata Knowledge Graph",
    authority: "Wikimedia Foundation",
    basisPrime: 29,
    organIri: "urn:verdant:sensory:wikidata",
    sensoryModality: "Structured World & Epistemic Ontologies (ISO 24617-2)",
    schemaTypes: ["Dataset", "Place"],
    mediaTypes: ["application/ld+json"],
    endpoint: "https://www.wikidata.org/w/api.php",
    contentTypes: ["Dataset", "Place", "Story"],
  },
  {
    id: "github",
    label: "GitHub Open Source Registry",
    authority: "GitHub Public Index",
    basisPrime: 31,
    organIri: "urn:verdant:sensory:github",
    sensoryModality: "Executable Engines, WGSL Shaders, Maps & DTCG Themes",
    schemaTypes: ["SoftwareSourceCode", "DefinedTermSet", "Place"],
    mediaTypes: ["application/wasm", "application/tokens+json", "application/geo+json"],
    endpoint: "https://api.github.com/search/repositories",
    contentTypes: ["SoftwareSourceCode", "DefinedTermSet", "Place"],
  },
  {
    id: "verdant-prism",
    label: "Verdant Locked Prism Catalogs",
    authority: "UOR Foundation · SHA-256 Locked",
    basisPrime: 37,
    organIri: "urn:verdant:sensory:verdant-prism",
    sensoryModality: "SHA-256-Locked World Packs, Standards & Proof Boundary",
    schemaTypes: ["3DModel", "DefinedTermSet", "AudioObject", "Place", "Dataset", "SoftwareSourceCode"],
    mediaTypes: ["application/octet-stream", "application/ld+json"],
    endpoint: "../catalogs/world-packs.json",
    contentTypes: ["3DModel", "PBRMaterial", "DefinedTermSet", "Font", "AudioObject", "Place", "Dataset", "Story", "SoftwareSourceCode"],
  },
]);

const FEATURED_KHRONOS_ORDER = [
  "DamagedHelmet",
  "Lantern",
  "AntiqueCamera",
  "BoomBox",
  "WaterBottle",
  "Fox",
  "Avocado",
  "Duck",
  "ToyCar",
  "FlightHelmet",
  "ABeautifulGame",
  "CesiumMan",
];

const KHRONOS_BASE = "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/";

const indexCache = {
  khronos: null,
  polyhavenModels: null,
  polyhavenTextures: null,
  fontsource: null,
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

async function queryPolyHaven(tokens, contentType, limit, timeoutMs) {
  const out = [];
  const wantModels = contentType === "all" || contentType === "3DModel";
  const wantTextures = contentType === "all" || contentType === "PBRMaterial";

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
  const items = [];

  // Authoritative W3C DTCG Themes & Typography specimens grounded in Hologram Brand Kit
  items.push(
    {
      id: "prism:theme:hologram-warm-dark",
      title: "Hologram Warm Dark (W3C DTCG)",
      description: "Authoritative W3C DTCG 1.0 dark design-token theme pinned by brand.lock with WCAG AA contrast guarantees.",
      contentTypeKey: "DefinedTermSet",
      schemaType: "DefinedTermSet",
      mediaType: "application/tokens+json",
      targetSlot: "design:theme",
      targetRole: "design",
      streamId: "verdant-prism",
      streamLabel: "Verdant Prism",
      sourceUri: "urn:verdant:theme:hologram-warm-dark",
      previewUrl: "",
      glbUrl: "",
      licenseSpdx: "Apache-2.0",
      tags: ["theme", "dtcg", "dark", "wcag-aa"],
      properties: { themeClass: "dark", brandKit: "hologram-warm", wcagContrast: "AA" },
    },
    {
      id: "prism:theme:hologram-warm-light",
      title: "Hologram Warm Daylight (W3C DTCG)",
      description: "High-legibility W3C DTCG daylight design-token system calibrated for bright authoring environments.",
      contentTypeKey: "DefinedTermSet",
      schemaType: "DefinedTermSet",
      mediaType: "application/tokens+json",
      targetSlot: "design:theme",
      targetRole: "design",
      streamId: "verdant-prism",
      streamLabel: "Verdant Prism",
      sourceUri: "urn:verdant:theme:hologram-warm-light",
      previewUrl: "",
      glbUrl: "",
      licenseSpdx: "Apache-2.0",
      tags: ["theme", "dtcg", "light", "daylight"],
      properties: { themeClass: "", brandKit: "hologram-warm", wcagContrast: "AA" },
    },
    {
      id: "prism:font:archivo-geist",
      title: "Archivo + Geist + Geist Mono (Brand Lock)",
      description: "Metric-matched W3C CSS Fonts Level 4 editorial display, UI sans, and tabular monospace typography stack.",
      contentTypeKey: "Font",
      schemaType: "CreativeWork",
      mediaType: "font/woff2",
      targetSlot: "design:typography",
      targetRole: "design",
      streamId: "verdant-prism",
      streamLabel: "Verdant Prism",
      sourceUri: "urn:verdant:font:archivo-geist",
      previewUrl: "",
      glbUrl: "",
      licenseSpdx: "OFL-1.1",
      tags: ["font", "typography", "geist", "archivo"],
      properties: { fontFamily: "Archivo, Geist, sans-serif", monoFamily: "Geist Mono", weights: [400, 500, 600, 700] },
    },
  );

  // Domain Packs -> Worlds/Maps, 3D Models, Audio Sabine Rooms, Narrative Characters
  for (const pack of localWorldPacks) {
    items.push({
      id: `prism:domain:${pack.slug}`,
      title: `${pack.title} (World Pack)`,
      description: `SHA-256-locked ${pack.environment?.name || "World"} (${pack.id}, Seed ${pack.seed}, LOD${pack.defaultLodRank}) with ${(pack.models || []).length} HD models & ${(pack.characters || []).length} 22-joint rigs.`,
      contentTypeKey: "Place",
      schemaType: "Place",
      mediaType: "application/geo+json",
      targetSlot: "gameplay:world-map",
      targetRole: "gameplay",
      streamId: "verdant-prism",
      streamLabel: "Verdant Prism",
      sourceUri: `urn:verdant:domain:${pack.slug}`,
      previewUrl: "",
      glbUrl: "",
      licenseSpdx: "Apache-2.0",
      tags: ["world", "domain-pack", pack.slug, pack.schemaType || "VideoGame"],
      properties: { domainSlug: pack.slug, domainId: pack.id, seed: pack.seed, defaultLodRank: pack.defaultLodRank },
    });

    for (const mdl of pack.models || []) {
      items.push({
        id: `prism:model:${mdl.id}`,
        title: `${mdl.name} (${pack.title})`,
        description: `Procedural HD ${mdl.primitive} architectural mesh using ${mdl.materialId} inside ${pack.title}.`,
        contentTypeKey: "3DModel",
        schemaType: "3DModel",
        mediaType: "model/gltf-binary",
        targetSlot: "design:3d-model",
        targetRole: "design",
        streamId: "verdant-prism",
        streamLabel: "Verdant Prism",
        sourceUri: `urn:verdant:model:${mdl.id}`,
        previewUrl: "",
        glbUrl: "",
        licenseSpdx: "Apache-2.0",
        tags: ["3d-model", mdl.primitive, mdl.materialId, pack.slug],
        properties: { modelId: mdl.id, primitive: mdl.primitive, materialId: mdl.materialId, scalePermille: mdl.scalePermille },
      });
    }

    if (pack.environment?.acousticVolumeM3) {
      items.push({
        id: `prism:audio:${pack.slug}`,
        title: `${pack.environment.name} · Sabine Acoustic Room`,
        description: `ISO 3382-1 Sabine acoustic enclosure (${pack.environment.acousticVolumeM3} m³) with 128-frame AudioWorklet modal synthesis.`,
        contentTypeKey: "AudioObject",
        schemaType: "AudioObject",
        mediaType: "audio/mpeg",
        targetSlot: "audio:soundscape",
        targetRole: "audio",
        streamId: "verdant-prism",
        streamLabel: "Verdant Prism",
        sourceUri: `urn:verdant:acoustics:${pack.slug}`,
        previewUrl: "",
        glbUrl: "",
        licenseSpdx: "Apache-2.0",
        tags: ["audio", "sabine", "acoustics", pack.slug],
        properties: {
          volumeM3: pack.environment.acousticVolumeM3,
          absorptionMilliM2: pack.environment.acousticAbsorptionMilliM2,
        },
      });
    }

    for (const ch of pack.characters || []) {
      items.push({
        id: `prism:character:${ch.id}`,
        title: `${ch.name} · ${ch.role}`,
        description: `22-joint ${ch.rigId} character profile with ${(ch.directives || []).length} NavMesh directives & ${(ch.knowledgeFacts || []).length} ISO 24617-2 grounded facts.`,
        contentTypeKey: "Story",
        schemaType: "CreativeWork",
        mediaType: "text/markdown",
        targetSlot: "narrative:story",
        targetRole: "narrative",
        streamId: "verdant-prism",
        streamLabel: "Verdant Prism",
        sourceUri: `urn:verdant:character:${ch.id}`,
        previewUrl: "",
        glbUrl: "",
        licenseSpdx: "Apache-2.0",
        tags: ["character", "narrative", "iso-24617-2", ch.rigId, pack.slug],
        properties: { characterId: ch.id, role: ch.role, rigId: ch.rigId, directives: ch.directives, knowledgeFacts: ch.knowledgeFacts },
      });
    }
  }

  // Authoritative Prism Standards -> Ontologies / Code / Materials
  for (const std of localPrismCatalogs.slice(0, 16)) {
    const isCode = std.domain === "CAT-ARCH" || std.domain === "CAT-QUALITY" || std.studioRoles.includes("code");
    const ctKey = isCode ? "SoftwareSourceCode" : "Dataset";
    const ctMeta = AUTHORITATIVE_CONTENT_TYPES.find((c) => c.key === ctKey);
    items.push({
      id: `prism:std:${std.id}`,
      title: `${std.title} (${std.authority.split("·")[0].trim()})`,
      description: std.invariant,
      contentTypeKey: ctKey,
      schemaType: ctMeta.schemaType,
      mediaType: ctMeta.mediaType,
      targetSlot: ctMeta.targetSlot,
      targetRole: std.studioRoles[0] || ctMeta.targetRole,
      streamId: "verdant-prism",
      streamLabel: "Verdant Prism",
      sourceUri: `urn:verdant:standard:${std.id}`,
      previewUrl: "",
      glbUrl: "",
      licenseSpdx: "Apache-2.0",
      tags: ["prism-standard", std.domain, ...(std.taxon || []).slice(0, 3)],
      properties: { standardId: std.id, lexModule: std.lexModule, holoQueries: std.holoQueries },
    });
  }
  return items;
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
} = {}) {
  const cleanQuery = String(query || "").trim();
  const tokens = cleanQuery
    .toLowerCase()
    .split(/\s+/)
    .filter((t) => t.length > 0);

  const streamStatus = {};
  const tasks = [];

  const wantStream = (id, supportedTypes) =>
    (streamId === "all" || streamId === id) &&
    (contentType === "all" || supportedTypes.includes(contentType));

  // Always evaluate local SHA-256-locked Verdant Prism catalog synchronously (<2ms):
  const localItems = wantStream("verdant-prism", LIVE_CATALOG_STREAMS[6].contentTypes)
    ? buildLocalVerdantCatalogItems(localWorldPacks, localPrismCatalogs).filter((item) => {
        if (contentType !== "all" && item.contentTypeKey !== contentType) return false;
        const hay = `${item.id} ${item.title} ${item.description} ${item.tags.join(" ")} ${item.contentTypeKey}`;
        return matchesTokens(hay, tokens);
      })
    : [];
  streamStatus["verdant-prism"] = { status: "live", count: localItems.length };

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
      addRemoteTask("khronos-gltf", () => queryKhronosGltf(tokens, 8, timeoutMs));
    }
    if (wantStream("polyhaven", ["3DModel", "PBRMaterial"])) {
      addRemoteTask("polyhaven", () => queryPolyHaven(tokens, contentType, 8, timeoutMs));
    }
    if (wantStream("fontsource", ["Font"])) {
      addRemoteTask("fontsource", () => queryFontsource(tokens, 6, timeoutMs));
    }
    if (wantStream("openverse", ["AudioObject", "PBRMaterial"])) {
      addRemoteTask("openverse", () => queryOpenverse(cleanQuery, contentType, 6, timeoutMs));
    }
    if (wantStream("wikidata", ["Dataset", "Place", "Story"])) {
      addRemoteTask("wikidata", () => queryWikidata(cleanQuery, 6, timeoutMs));
    }
    if (wantStream("github", ["SoftwareSourceCode", "DefinedTermSet", "Place"])) {
      addRemoteTask("github", () => queryGitHub(cleanQuery, contentType, 6, timeoutMs));
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
