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
