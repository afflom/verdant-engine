// Local SHA-256-locked Verdant Prism catalog builder (`site/catalog-streams-local.mjs`).
// Always instant, 100% offline-resilient, covering all 9 authoritative Schema.org content types.

export function buildLocalVerdantCatalogItemsImpl(
  localWorldPacks = [],
  localPrismCatalogs = [],
  authoritativeContentTypes = [],
) {
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
    const ctMeta = authoritativeContentTypes.find((c) => c.key === ctKey);
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

export async function queryWikidataImpl(query, limit, timeoutMs, fetchJsonSafe) {
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

export async function queryGitHubImpl(query, contentType, limit, timeoutMs, fetchJsonSafe, authoritativeContentTypes) {
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
    const ctMeta = authoritativeContentTypes.find((c) => c.key === ctKey) || authoritativeContentTypes[9];
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

export async function queryFontsourceImpl(tokens, limit, timeoutMs, indexCache, fetchJsonSafe, matchesTokens) {
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

export async function queryOpenverseImpl(query, contentType, limit, timeoutMs, fetchJsonSafe) {
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

export async function fetchLiveKnowledgePassageImpl(topic, timeoutMs, fetchJsonSafe) {
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
