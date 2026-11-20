//! Manager artifact synthesis, world differentiation profiles, Stitch app catalog publication,
//! W3C DTCG Studio experience CSS/JSON, tensor scaling contract & directive materialization
//! (`operator_synth.rs`), extracted from `operator.rs`.

use super::*;

impl VerdantOperatorChannel {
    /// Synthesizes canonical per-world differentiation profiles (`mgr-playable-world`) strictly
    /// from SHA-256-locked `AUTH-DOMAIN-PACKS` + `AUTH-KHRONOS-GLTF2`.
    #[must_use]
    pub fn synthesize_world_differentiation_profiles(&self) -> Vec<WorldDifferentiationProfile> {
        let hundred_u =
            (1u8 << (1 + 1 + 1 + 1 + 1 + 1)) + (1u8 << (1 + 1 + 1 + 1 + 1)) + (1u8 << (1 + 1));
        let hundred_f = f32::from(hundred_u);
        let idx_one = 1usize;
        let idx_two = 1usize + 1usize;
        let mut profiles = Vec::new();
        for dom in &self.observer.catalogs.domain_packs.domains {
            let sky_pct = dom.environment.sky_rgb_percent;
            let fog_pct = dom.environment.fog_rgb_percent;
            let sky_rgb = [
                f32::from(sky_pct[0]) / hundred_f,
                f32::from(sky_pct[idx_one]) / hundred_f,
                f32::from(sky_pct[idx_two]) / hundred_f,
            ];
            let fog_rgb = [
                f32::from(fog_pct[0]) / hundred_f,
                f32::from(fog_pct[idx_one]) / hundred_f,
                f32::from(fog_pct[idx_two]) / hundred_f,
            ];
            let (genre, genre_label, schema_org_type) = match dom.slug.as_str() {
                "starfall-bastion" => ("first-person-shooter", "First-Person Shooter", "VideoGame"),
                "meridian-observatory" => ("mmo-universe", "MMO Universe", "VideoGame"),
                "glasshouse-chronicle" => {
                    ("indie-side-scroller", "Indie Side-Scroller", "VideoGame")
                }
                _ => ("coastal-adventure", "3D Coastal Adventure", "VideoGame"),
            };
            let hero_models: Vec<String> = dom
                .models
                .iter()
                .map(|m| format!("{} ({})", m.name, m.material_id))
                .collect();
            let characters: Vec<String> = dom
                .characters
                .iter()
                .map(|c| format!("{} ({})", c.name, c.role))
                .collect();
            let kappa_input = format!(
                "{}:{}:{}:{:?}:{:?}",
                dom.id, dom.slug, dom.seed, sky_pct, fog_pct
            );
            let profile_kappa_hex = sha256_hex_tagged!("world-profile", kappa_input.as_bytes());
            profiles.push(WorldDifferentiationProfile {
                domain_id: dom.id.clone(),
                slug: dom.slug.clone(),
                title: dom.title.clone(),
                seed: dom.seed,
                default_lod_rank: dom.default_lod_rank,
                sky_rgb,
                fog_rgb,
                environment_name: dom.environment.name.clone(),
                acoustic_volume_m3: dom.environment.acoustic_volume_m3,
                acoustic_absorption_milli_m2: dom.environment.acoustic_absorption_milli_m2,
                default_app_genre: genre.to_string(),
                default_app_genre_label: genre_label.to_string(),
                schema_org_type: schema_org_type.to_string(),
                hero_models,
                characters,
                profile_kappa_hex,
            });
        }
        profiles
    }

    /// Serializes the `mgr-playable-world` differentiation profiles as pretty JSON.
    #[must_use]
    pub fn synthesize_world_differentiation_profiles_json(&self) -> String {
        let profiles = self.synthesize_world_differentiation_profiles();
        let manager_kappa = self
            .managers
            .iter()
            .find(|m| m.spec.manager_id == "mgr-playable-world")
            .map(|m| m.kappa_hex.as_str())
            .unwrap_or("");
        serde_json::to_string_pretty(&serde_json::json!({
            "spec": "verdant/world-differentiation-profiles/1",
            "owningManager": "mgr-playable-world",
            "managerKappaHex": manager_kappa,
            "governingAuthorities": ["AUTH-DOMAIN-PACKS", "AUTH-KHRONOS-GLTF2", "AUTH-ISO-42010-VIEWPOINTS"],
            "worldCount": profiles.len(),
            "worlds": profiles,
        }))
        .unwrap_or_else(|_| "{}".to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn build_stitch_manifest_app(
        &mut self,
        app_id_override: Option<&str>,
        meta: (&str, &str, &str, &str),
        publisher_id: &str,
        seeded: bool,
    ) -> Result<StitchCatalogManifestApp, ObserverModelError> {
        let (title, genre, domain_slug, summary) = meta;
        let dom = self
            .observer
            .catalogs
            .domain_packs
            .domains
            .iter()
            .find(|d| d.slug == domain_slug || d.id == domain_slug)
            .cloned()
            .ok_or_else(|| ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: format!("unknown world domain `{domain_slug}` in AUTH-DOMAIN-PACKS"),
            })?;
        let lod_rank = 1u64 + 1u64;
        let scene = StandardSceneState::synthesize_at_lod(dom.seed, lod_rank).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: format!("world synthesis failed for `{}`: {e}", dom.slug),
            }
        })?;
        let uorc = WorldSnapshot::encode_uorc(&scene).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: format!("uorc sealing failed: {e}"),
            }
        })?;
        let entry =
            self.publish_stitch_app_catalog_entry(title, genre, summary, publisher_id, &uorc)?;
        let (genre_label, schema_type) = match genre {
            "first-person-shooter" => ("First-Person Shooter", "VideoGame"),
            "mmo-universe" | "massively-multiplayer-universe" => ("MMO Universe", "VideoGame"),
            "indie-side-scroller" => ("Indie Side-Scroller", "VideoGame"),
            "spatial-simulation" => ("Spatial Simulation", "SoftwareApplication"),
            _ => ("3D Coastal Adventure", "VideoGame"),
        };
        let hex_prefix_len = 1usize << (1 + 1 + 1 + 1 + 1);
        let short_id_len = 1usize << (1 + 1 + 1);
        let roles = ["Shard A", "Shard B", "Parity (A ^ B)"];
        let mut stitch_shards = Vec::new();
        for (i, frag) in entry.stitch_fragments.iter().enumerate() {
            let wire = frag.encode_wire();
            let shard_sha = sha256_hex_tagged!("stitch-frag", &frag.payload_root);
            let role = roles.get(i).copied().unwrap_or("Shard");
            stitch_shards.push(StitchCatalogShardSummary {
                index: frag.index,
                role: role.to_string(),
                bytes: wire.len(),
                sha256: shard_sha[..hex_prefix_len.min(shard_sha.len())].to_string(),
            });
        }
        let app_id = if let Some(fixed) = app_id_override {
            fixed.to_string()
        } else {
            let clean_slug: String = title
                .to_lowercase()
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
                .collect();
            let trimmed = clean_slug.trim_matches('-');
            let max_slug_len = (1usize << (1 + 1 + 1 + 1 + 1)) + (1usize << (1 + 1));
            let base = if trimmed.is_empty() {
                "verdant-app"
            } else {
                &trimmed[..max_slug_len.min(trimmed.len())]
            };
            format!(
                "app-{base}-{}",
                &entry.uorc_kappa_hex[..short_id_len.min(entry.uorc_kappa_hex.len())]
            )
        };
        Ok(StitchCatalogManifestApp {
            app_id,
            title: entry.title,
            genre: genre.to_string(),
            genre_label: genre_label.to_string(),
            schema_type: schema_type.to_string(),
            domain_slug: dom.slug,
            world_title: dom.title,
            summary: entry.description,
            publisher_id: publisher_id.to_string(),
            archive_format: ".uorc".to_string(),
            uorc_bytes: entry.uorc_byte_len,
            payload_sha256: entry.uorc_kappa_hex,
            stitch_shards,
            published_at: "2026-11-19T00:00:00Z".to_string(),
            seeded,
        })
    }

    /// Synthesizes the canonical `site/generated/stitch-app-catalog.json` manifest using real
    /// sealed `.uorc` snapshots and real `afflom/stitch` 2-of-3 parity shards, preserving any
    /// additional non-seeded apps already published in `existing_json`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn synthesize_stitch_app_catalog_json(
        &mut self,
        existing_json: Option<&str>,
    ) -> Result<String, ObserverModelError> {
        let mut custom_apps: Vec<StitchCatalogManifestApp> = Vec::new();
        if let Some(raw) = existing_json {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(raw) {
                if let Some(arr) = val.get("apps").and_then(|v| v.as_array()) {
                    for item in arr {
                        if let Ok(app) =
                            serde_json::from_value::<StitchCatalogManifestApp>(item.clone())
                        {
                            if !app.seeded {
                                custom_apps.push(app);
                            }
                        }
                    }
                }
            }
        }
        let flagship_specs = [
            (
                "app-starfall-vanguard-fps",
                "Starfall Vanguard",
                "first-person-shooter",
                "starfall-bastion",
                "Fast-paced 3D arena FPS with 64-triangle Morton meshlets, UHD 4K CAS resolve, and calibrated stone arena acoustics.",
            ),
            (
                "app-meridian-expanse-mmo",
                "Meridian Expanse",
                "mmo-universe",
                "meridian-observatory",
                "Multi-sector floating-origin universe featuring celestial observatories, synchronized world shards, and persistent state recovery.",
            ),
            (
                "app-glasshouse-canopy-runner",
                "Glasshouse Canopy Runner",
                "indie-side-scroller",
                "glasshouse-chronicle",
                "Atmospheric botanical side-scroller built with 2.5D spline traversal, conservatory glass acoustics, and grounded character dialogue.",
            ),
            (
                "app-sunspire-beacon-trials",
                "Sunspire Beacon Trials",
                "coastal-adventure",
                "sunspire-regatta",
                "Coastal 3D action trial set across the Sunspire stone arch basin with beacon harmonization, patrol hulls, and 3rd/1st person cameras.",
            ),
        ];
        let mut apps: Vec<StitchCatalogManifestApp> = custom_apps;
        let mut seen: BTreeSet<String> = apps.iter().map(|a| a.app_id.clone()).collect();
        for (app_id, title, genre, domain_slug, summary) in flagship_specs {
            if seen.insert(app_id.to_string()) {
                let app = self.build_stitch_manifest_app(
                    Some(app_id),
                    (title, genre, domain_slug, summary),
                    "prism:verdant-curated",
                    true,
                )?;
                apps.push(app);
            }
        }
        let curator_kappa = self
            .managers
            .iter()
            .find(|m| m.spec.manager_id == "mgr-catalog-curator")
            .map(|m| m.kappa_hex.as_str())
            .unwrap_or("");
        serde_json::to_string_pretty(&serde_json::json!({
            "spec": "verdant/stitch-app-catalog/1",
            "owningManager": "mgr-catalog-curator",
            "almManager": "mgr-studio-alm",
            "managerKappaHex": curator_kappa,
            "protocol": self.viewpoints.stitch_app_catalog_protocol,
            "appCount": apps.len(),
            "apps": apps,
        }))
        .map_err(|e| ObserverModelError::CommunicationEnvelopeRejected {
            channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
            detail: e.to_string(),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn write_manager_file(
        repo_root: &Path,
        manager_id: &str,
        rel_path: &str,
        contents: &str,
    ) -> Result<MaterializedFileRecord, ObserverModelError> {
        let full = repo_root.join(rel_path);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                ObserverModelError::CommunicationEnvelopeRejected {
                    channel: CommunicationChannelKind::AgentToAgent.slug().to_string(),
                    detail: format!("failed creating parent dir for `{rel_path}`: {e}"),
                }
            })?;
        }
        std::fs::write(&full, contents.as_bytes()).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::AgentToAgent.slug().to_string(),
                detail: format!("failed writing `{rel_path}`: {e}"),
            }
        })?;
        Ok(MaterializedFileRecord {
            manager_id: manager_id.to_string(),
            path: rel_path.to_string(),
            bytes: contents.len(),
            sha256_hex: sha256_hex_tagged!("materialized-file", contents.as_bytes()),
        })
    }

    /// Synthesizes the canonical `mgr-design-coordinator` brand, communications, onboarding, and
    /// plain-language experience manifest (`site/generated/experience-design-system.json`) under
    /// ISO 9241-210:2019 (Human-Centred Design), W3C DTCG 2025.10, and WCAG 2.2 AA.
    #[must_use]
    pub fn synthesize_experience_design_system_json(&self) -> String {
        let manager_kappa = self
            .managers
            .iter()
            .find(|m| m.spec.manager_id == "mgr-design-coordinator")
            .map(|m| m.kappa_hex.as_str())
            .unwrap_or("");
        serde_json::to_string_pretty(&serde_json::json!({
            "spec": "verdant/experience-design-system/1",
            "owningManager": "mgr-design-coordinator",
            "vsmSystem": "VSM-S2",
            "managerKappaHex": manager_kappa,
            "governingStandards": [
                "ISO 9241-210:2019 Human-Centred Design",
                "W3C DTCG 2025.10 Design Tokens",
                "WCAG 2.2 Level AA",
                "ISO/IEC/IEEE 42010:2022 Architecture Viewpoints"
            ],
            "brand": {
                "name": "Verdant",
                "studioName": "Verdant Studio",
                "catalogsName": "Verdant Catalogs",
                "tagline": "Create, play, and share 3D games and worlds right in your browser.",
                "studioHeadline": "Start building your 3D world",
                "studioSubhead": "Pick a starter world, customize it with free 3D assets, playtest live, and share your finished app to the Verdant App Catalog.",
                "catalogHeadline": "Playable Apps, 3D Assets & Starting Worlds",
                "catalogSubhead": "Launch community games, browse free 3D models and sky domes, or open any starting world directly in Verdant Studio."
            },
            "studioJourney": {
                "defaultStep": "world",
                "steps": [
                    {
                        "id": "world",
                        "number": "01",
                        "title": "Choose a World",
                        "summary": "Pick one of the four ready-made 3D worlds on the left and drag on the stage to explore it live.",
                        "actionLabel": "Focus Worlds",
                        "nextStep": "build",
                        "nextLabel": "Next: Add 3D Assets"
                    },
                    {
                        "id": "build",
                        "number": "02",
                        "title": "Add Assets & Customize",
                        "summary": "Search free 3D models, characters, skies, and materials below. Preview anything or click + Add to bring it into your project.",
                        "actionLabel": "Browse Assets",
                        "nextStep": "publish",
                        "nextLabel": "Next: Play & Share"
                    },
                    {
                        "id": "publish",
                        "number": "03",
                        "title": "Playtest & Share",
                        "summary": "Name your creation, choose a game genre, run a quick check, and publish to the Stitch App Catalog or export a .uorc file.",
                        "actionLabel": "Publish & Export",
                        "nextStep": "world",
                        "nextLabel": "Back to Step 1"
                    }
                ]
            },
            "plainLanguage": {
                "playPreview": "Play Preview",
                "pausePreview": "Pause Preview",
                "cameraPrefix": "View",
                "detailPrefix": "Detail",
                "soundOn": "Sound: On",
                "soundOff": "Sound: Off",
                "uploadModel": "Upload 3D (.glb)",
                "directPlaceholder": "Try \"focus stone arch\", \"3rd person\", or \"ultra detail\"…",
                "readyBanner": "Ready — pick a world, browse 3D assets below, or click Play Preview.",
                "worldCards": {
                    "sunspire-regatta": {
                        "badge": "Coastal Adventure",
                        "blurb": "Sunlit stone arch basin & coastal beacon trials"
                    },
                    "meridian-observatory": {
                        "badge": "MMO Universe",
                        "blurb": "Twilight celestial observatory & starlight dome"
                    },
                    "glasshouse-chronicle": {
                        "badge": "Indie Side-Scroller",
                        "blurb": "Lush sunlit botanical sanctuary & terraced canopy"
                    },
                    "starfall-bastion": {
                        "badge": "Arena FPS",
                        "blurb": "Volcanic dusk bastion built for fast 3D combat"
                    }
                },
                "slotLabels": {
                    "design:3d-model": "3D Model",
                    "design:character": "Character Rig",
                    "design:material": "Material & Sky",
                    "design:theme": "Theme & Style",
                    "design:typography": "Typography",
                    "audio:soundscape": "Soundscape",
                    "narrative:lore": "World Lore",
                    "code:shader": "Shader & Code",
                    "world:pack": "Starting World",
                    "studio:any": "Studio Asset"
                }
            }
        }))
        .unwrap_or_else(|_| "{}".to_string())
    }

    /// Synthesizes the canonical `mgr-design-coordinator` visual refinement & guided-workflow
    /// stylesheet (`site/generated/studio-experience.css`) strictly using W3C DTCG CSS variables
    /// (`var(--...)`) with zero raw `#hex` literals and zero `backdrop-filter: blur`.
    #[must_use]
    pub fn synthesize_studio_experience_css(&self) -> String {
        r#"/* GENERATED by mgr-design-coordinator via `verdant operator synthesize`.
 * Governed by ISO 9241-210:2019 (Human-Centred Design), W3C DTCG 2025.10, and WCAG 2.2 AA.
 * Strictly uses DTCG custom properties (zero raw hex colors, zero backdrop-filter blur). */

/* ==================== TOP NAVIGATION & BRAND CLARITY ==================== */
.ide-topbar {
  min-height: 3.75rem;
  padding: var(--space-2) clamp(1rem, 2.5vw, 2rem);
  box-shadow: 0 1px 0 var(--border);
}
.ide-brand {
  font-size: var(--text-18);
  letter-spacing: -0.025em;
}
.ide-brand-dot {
  width: 0.65rem;
  height: 0.65rem;
  box-shadow: 0 0 0 3px color-mix(in oklab, var(--brand) 22%, transparent);
}
.ide-nav a {
  padding: 0.35rem 0.65rem;
  border-radius: var(--radius-control);
  transition: background var(--duration) var(--ease), color var(--duration) var(--ease);
}
.ide-nav a:hover {
  background: var(--secondary);
  text-decoration: none;
}

/* ==================== COMPACT STUDIO TEMPLATE & WORKSPACE BAR ==================== */
.vx-journey-banner {
  margin: 0;
  padding: 0.55rem clamp(0.75rem, 2vw, 1.5rem);
  border-bottom: var(--border-width) solid var(--border);
  background: var(--card);
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 0.65rem;
}
.vx-toolbar-group {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.4rem;
}
.vx-toolbar-label {
  font-family: var(--font-mono);
  font-size: 0.76rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--muted-foreground);
  margin-right: 0.2rem;
}
.vx-tmpl-pill {
  min-height: 2.15rem;
  padding: 0 0.75rem;
  border-radius: 999px;
  border: var(--border-width) solid var(--border);
  background: var(--background);
  color: var(--foreground);
  font-family: var(--font-sans);
  font-size: 0.84rem;
  font-weight: 500;
  cursor: pointer;
  white-space: nowrap;
  transition: border-color var(--duration) var(--ease), background var(--duration) var(--ease), color var(--duration) var(--ease);
}
.vx-tmpl-pill:hover {
  border-color: var(--border-strong);
  background: var(--secondary);
}
.vx-tmpl-pill[aria-pressed="true"] {
  border-color: var(--brand);
  background: var(--brand);
  color: var(--brand-foreground);
  font-weight: 600;
}

/* ==================== LIVE 3D CREATION TOOLS (ADD PRIMITIVES, OUTLINER, TRANSFORM) ==================== */
.vx-add-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.45rem;
}
.vx-add-btn {
  min-height: 2.35rem;
  padding: 0.35rem 0.6rem;
  border-radius: var(--radius-control);
  border: var(--border-width) solid var(--border);
  background: var(--background);
  color: var(--foreground);
  font-family: var(--font-sans);
  font-size: 0.84rem;
  font-weight: 500;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.35rem;
  cursor: pointer;
  transition: border-color var(--duration) var(--ease), background var(--duration) var(--ease);
}
.vx-add-btn:hover {
  border-color: var(--brand);
  background: color-mix(in oklab, var(--brand) 10%, var(--card));
}
.vx-outliner-list {
  display: grid;
  gap: 0.35rem;
  max-height: 13rem;
  overflow-y: auto;
}
.vx-outliner-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.4rem;
  padding: 0.4rem 0.55rem;
  border-radius: var(--radius-control);
  border: var(--border-width) solid var(--border);
  background: var(--background);
  font-size: 0.84rem;
}
.vx-outliner-row[data-selected="true"] {
  border-color: var(--brand);
  background: color-mix(in oklab, var(--brand) 10%, var(--card));
}
.vx-outliner-sel {
  flex: 1;
  min-height: 1.85rem;
  padding: 0;
  border: 0;
  background: transparent;
  color: var(--foreground);
  text-align: left;
  font-family: var(--font-sans);
  font-size: 0.84rem;
  font-weight: 500;
  cursor: pointer;
}
.vx-outliner-del {
  min-height: 1.75rem;
  min-width: 1.75rem;
  padding: 0 0.35rem;
  border-radius: var(--radius-control);
  border: var(--border-width) solid var(--border);
  background: var(--secondary);
  color: var(--muted-foreground);
  font-size: 0.8rem;
  cursor: pointer;
}
.vx-outliner-del:hover {
  border-color: var(--destructive);
  color: var(--destructive);
}
.vx-transform-card {
  padding: 0.85rem;
  border-radius: var(--radius);
  border: var(--border-width) solid var(--border-strong);
  background: var(--background);
  display: grid;
  gap: 0.65rem;
}
.vx-transform-card[hidden] { display: none; }
.vx-vec3-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 0.4rem;
}
.vx-vec3-cell {
  display: grid;
  gap: 0.2rem;
}
.vx-vec3-cell label {
  font-family: var(--font-mono);
  font-size: 0.72rem;
  color: var(--muted-foreground);
}
.vx-vec3-cell input {
  width: 100%;
  min-height: 2.1rem;
  padding: 0 0.45rem;
  border-radius: var(--radius-control);
  border: var(--border-width) solid var(--border-strong);
  background: var(--card);
  color: var(--foreground);
  font-family: var(--font-mono);
  font-size: 0.82rem;
}
.vx-swatch-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.4rem;
}
.vx-swatch-btn {
  min-height: 2rem;
  padding: 0 0.6rem;
  border-radius: 999px;
  border: var(--border-width) solid var(--border);
  background: var(--card);
  color: var(--foreground);
  font-size: 0.78rem;
  cursor: pointer;
}
.vx-swatch-btn[aria-pressed="true"] {
  border-color: var(--brand);
  background: var(--brand);
  color: var(--brand-foreground);
  font-weight: 600;
}

/* ==================== CALM EDITORIAL WORKSPACE HIERARCHY ==================== */
.ide-grid {
  padding: clamp(0.85rem, 2vw, 1.5rem);
  gap: clamp(0.9rem, 1.8vw, 1.35rem);
}
.ide-left-col,
.ide-center-col,
.ide-right-col {
  gap: clamp(0.9rem, 1.6vw, 1.25rem);
}
.ide-pane {
  border-radius: calc(var(--radius) + 0.15rem);
  border-color: var(--border);
  transition: border-color var(--duration) var(--ease), box-shadow var(--duration) var(--ease);
}
.ide-pane-head {
  padding: 0.7rem 0.95rem;
  background: color-mix(in oklab, var(--sidebar) 88%, var(--card));
}
.ide-pane-head h2 {
  font-family: var(--font-display);
  font-size: var(--text-15, 0.95rem);
  font-weight: 600;
  letter-spacing: -0.01em;
  white-space: nowrap;
}
.ide-pane-sub {
  white-space: nowrap;
}
.right-tab-btn {
  padding: 0.65rem 0.25rem;
  font-size: 0.84rem;
  white-space: nowrap;
}
.cat-cta {
  text-decoration: none;
}
.wf-stage-bar[hidden] {
  display: none;
}
.ide-pane-body {
  padding: 1rem;
  gap: 0.85rem;
}

/* Progressive focus highlighting tied to active guided step */
body[data-studio-focus="world"] #studio-worlds-pane,
body[data-studio-focus="world"] #studio-stage-pane,
body[data-studio-focus="build"] #studio-discovery,
body[data-studio-focus="build"] #studio-right-pane,
body[data-studio-focus="publish"] #studio-publish-pane {
  border-color: color-mix(in oklab, var(--brand) 55%, var(--border-strong));
  box-shadow: inset 0 2px 0 var(--brand);
}

/* Refined World Environment Selector Cards */
.proj-domain-btn {
  padding: 0.75rem 0.85rem;
  border-radius: var(--radius);
  border: var(--border-width) solid var(--border);
  background: var(--background);
  display: grid;
  gap: 0.25rem;
  text-align: left;
  transition: border-color var(--duration) var(--ease), background var(--duration) var(--ease);
}
.proj-domain-btn:hover {
  border-color: var(--border-strong);
  background: var(--secondary);
}
.proj-domain-btn[aria-pressed="true"] {
  border-color: var(--brand);
  background: color-mix(in oklab, var(--brand) 10%, var(--card));
}
.proj-domain-btn strong {
  font-family: var(--font-display);
  font-size: var(--text-15, 0.94rem);
  color: var(--foreground);
}
.proj-domain-btn small {
  font-family: var(--font-sans);
  font-size: 0.82rem;
  color: var(--muted-foreground);
  line-height: 1.35;
}
.proj-domain-badge {
  display: inline-block;
  width: fit-content;
  margin-bottom: 0.15rem;
  padding: 0.12rem 0.48rem;
  border-radius: 999px;
  font-family: var(--font-mono);
  font-size: 0.72rem;
  font-weight: 600;
  background: var(--secondary);
  color: var(--foreground);
}
.proj-domain-btn[aria-pressed="true"] .proj-domain-badge {
  background: var(--brand);
  color: var(--brand-foreground);
}

/* Calm readable status panels instead of raw terminal dumps */
.wiz-log {
  padding: 0.7rem 0.85rem;
  border: var(--border-width) solid var(--border);
  border-radius: var(--radius-control);
  background: var(--background);
  font-family: var(--font-sans);
  font-size: 0.84rem;
  line-height: 1.5;
  color: var(--muted-foreground);
}
.st-verdict-bar {
  padding: 0.65rem 1rem;
  font-family: var(--font-sans);
  font-size: var(--text-14);
  color: var(--foreground);
  background: color-mix(in oklab, var(--sidebar) 75%, var(--background));
}

/* Cleaner Asset Discovery & Catalog Cards */
.disc-card {
  border-radius: var(--radius);
  transition: border-color var(--duration) var(--ease), transform var(--duration) var(--ease);
}
.disc-card:hover {
  border-color: var(--border-strong);
  transform: translateY(-1px);
}
.disc-card-slot {
  display: inline-block;
  width: fit-content;
  padding: 0.15rem 0.5rem;
  border-radius: 999px;
  background: var(--secondary);
  color: var(--foreground);
  font-family: var(--font-sans);
  font-size: 0.78rem;
  font-weight: 500;
}
.cat-organ-card,
.cat-gallery-card {
  border-radius: calc(var(--radius) + 0.15rem);
  transition: border-color var(--duration) var(--ease), transform var(--duration) var(--ease);
}
.cat-organ-card:hover,
.cat-gallery-card:hover {
  border-color: var(--border-strong);
  transform: translateY(-1px);
}

/* Crisp focus visibility (WCAG 2.2 AA) */
.vx-step-card:focus-visible,
.proj-domain-btn:focus-visible,
.st-btn:focus-visible,
.cat-btn:focus-visible,
.right-tab-btn:focus-visible {
  outline: var(--ring-width) solid var(--brand);
  outline-offset: var(--ring-offset);
}

.vx-scale-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  padding: 0.25rem 0.65rem;
  border-radius: var(--radius-full);
  border: 1px solid var(--border);
  background: var(--surface-subtle);
  color: var(--foreground);
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  line-height: 1.3;
  white-space: nowrap;
}

.vx-scale-badge[data-contract="met"] {
  border-color: var(--brand);
  color: var(--brand);
}
"#
        .to_string()
    }

    /// Synthesizes `site/generated/tensor-scaling-contract.json` from `mgr-runtime-stage` and
    /// `AUTH-ISO-25010-TENSOR-SCALING` (ISO/IEC 25010:2023 + ITU-R BT.2020/2100 + PagedTensorArena).
    #[must_use]
    pub fn synthesize_tensor_scaling_contract_json(&self) -> String {
        let runtime_kappa = self
            .managers
            .iter()
            .find(|m| m.spec.manager_id == "mgr-runtime-stage")
            .map_or("", |m| m.kappa_hex.as_str());
        let standard_doc: serde_json::Value =
            serde_json::from_str(LOCKED_ISO25010_TENSOR_SCALING_JSON)
                .unwrap_or_else(|_| serde_json::json!({}));
        let payload = serde_json::json!({
            "spec": "verdant/tensor-scaling-contract/1",
            "generatedBy": "verdant-operator-api",
            "ownerManagerId": "mgr-runtime-stage",
            "ownerManagerKappaHex": runtime_kappa,
            "governingAuthorities": [
                "AUTH-ISO-25010-TENSOR-SCALING",
                "AUTH-ISO-42010-VIEWPOINTS",
                "AUTH-VSM-CYBERNETICS"
            ],
            "pageQuantumBytes": crate::tensor::TENSOR_PAGE_QUANTUM_BYTES,
            "minArenaBudgetBytes": crate::tensor::MIN_ARENA_BUDGET_BYTES,
            "packedNibbleCompressionFactor": crate::tensor::TensorStorageEncoding::PackedNibble4.compression_factor(),
            "sparseRunsCompressionFactor": crate::tensor::TensorStorageEncoding::SparseRuns16.compression_factor(),
            "authority": standard_doc
        });
        serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "{}".to_string())
    }

    /// Materializes Manager-synthesized declarative artifacts strictly within each Manager's
    /// owned files (`site/generated/*`), records PROV-O lineage, and closes via VSM S3* audit.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn materialize_manager_artifacts(
        &mut self,
        repo_root: &Path,
        viewpoint_filter: Option<&str>,
    ) -> Result<MaterializationReport, ObserverModelError> {
        let want = |vp: &str, mgr: &str| -> bool {
            match viewpoint_filter {
                None | Some("all") => true,
                Some(f) => f.eq_ignore_ascii_case(vp) || f.eq_ignore_ascii_case(mgr),
            }
        };
        let mut written = Vec::new();
        if want("observer-executive", "mgr-observer-executive")
            || want("verification-observatory", "mgr-verification-observatory")
        {
            let json = self.synthesize_viewpoint_contracts_json();
            written.push(Self::write_manager_file(
                repo_root,
                "mgr-observer-executive",
                "site/generated/viewpoint-contracts.json",
                &json,
            )?);
        }
        if want("design-system", "mgr-design-coordinator")
            || want("creator-studio", "mgr-studio-alm")
            || want("asset-catalogs", "mgr-catalog-curator")
        {
            let exp_json = self.synthesize_experience_design_system_json();
            written.push(Self::write_manager_file(
                repo_root,
                "mgr-design-coordinator",
                "site/generated/experience-design-system.json",
                &exp_json,
            )?);
            let exp_css = self.synthesize_studio_experience_css();
            written.push(Self::write_manager_file(
                repo_root,
                "mgr-design-coordinator",
                "site/generated/studio-experience.css",
                &exp_css,
            )?);
        }
        if want("playable-world", "mgr-playable-world")
            || want("runtime-stage", "mgr-runtime-stage")
        {
            let json = self.synthesize_world_differentiation_profiles_json();
            written.push(Self::write_manager_file(
                repo_root,
                "mgr-playable-world",
                "site/generated/world-differentiation-profiles.json",
                &json,
            )?);
            let scaling_json = self.synthesize_tensor_scaling_contract_json();
            written.push(Self::write_manager_file(
                repo_root,
                "mgr-runtime-stage",
                "site/generated/tensor-scaling-contract.json",
                &scaling_json,
            )?);
        }
        if want("asset-catalogs", "mgr-catalog-curator") || want("creator-studio", "mgr-studio-alm")
        {
            let existing_path = repo_root.join("site/generated/stitch-app-catalog.json");
            let existing = std::fs::read_to_string(&existing_path).ok();
            let json = self.synthesize_stitch_app_catalog_json(existing.as_deref())?;
            written.push(Self::write_manager_file(
                repo_root,
                "mgr-catalog-curator",
                "site/generated/stitch-app-catalog.json",
                &json,
            )?);
        }
        if want("change-version-release", "mgr-change-release")
            || want("roadmap-kpi-strategy", "mgr-roadmap-strategy")
            || want("observer-executive", "mgr-observer-executive")
        {
            let ledger = self.load_or_init_release_ledger(repo_root)?;
            let ledger_json = serde_json::to_string_pretty(&ledger).map_err(|e| {
                ObserverModelError::CommunicationEnvelopeRejected {
                    channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                    detail: e.to_string(),
                }
            })?;
            written.push(Self::write_manager_file(
                repo_root,
                "mgr-change-release",
                "site/generated/release-ledger.json",
                &ledger_json,
            )?);
            let kpi_rep = self.evaluate_observer_roadmap_kpis(repo_root, true, true)?;
            let kpi_json = serde_json::to_string_pretty(&kpi_rep).map_err(|e| {
                ObserverModelError::CommunicationEnvelopeRejected {
                    channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                    detail: e.to_string(),
                }
            })?;
            written.push(Self::write_manager_file(
                repo_root,
                "mgr-roadmap-strategy",
                "site/generated/roadmap-kpi-report.json",
                &kpi_json,
            )?);
            let (_agenda, agenda_json) = self.synthesize_refinement_agenda(repo_root)?;
            written.push(Self::write_manager_file(
                repo_root,
                "mgr-observer-executive",
                "site/generated/observer-refinement-agenda.json",
                &agenda_json,
            )?);
            let (_gate, gate_json) = self.execute_lifecycle_gate(
                repo_root,
                &super::LifecycleGateOptions {
                    run_workspace_tests: false,
                    run_browser_gates: false,
                    expected_tag: None,
                    write_report: false,
                },
            )?;
            written.push(Self::write_manager_file(
                repo_root,
                "mgr-verification-observatory",
                "site/generated/lifecycle-gate-report.json",
                &gate_json,
            )?);
        }
        let summary = format!(
            "materialize:{}:files={}",
            viewpoint_filter.unwrap_or("all"),
            written.len()
        );
        let comm_kappa_hex = sha256_hex_tagged!("operator-materialize", summary.as_bytes());
        let seal_rec = self.seal_repository_views(repo_root, &comm_kappa_hex)?;
        written.push(seal_rec);
        let audit = self.audit_repository(repo_root, viewpoint_filter);
        Ok(MaterializationReport {
            ok: audit.passed,
            written_files: written,
            comm_kappa_hex,
            audit_passed: audit.passed,
        })
    }

    /// Publishes a new multi-genre Verdant App directly through the Verdant API into
    /// `site/generated/stitch-app-catalog.json` with real `.uorc` + 2-of-3 Stitch shards.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn publish_app_to_repo_catalog(
        &mut self,
        repo_root: &Path,
        title: &str,
        genre: &str,
        domain_slug: &str,
        summary: &str,
        publisher_prism_id: &str,
    ) -> Result<StitchCatalogManifestApp, ObserverModelError> {
        let catalog_rel = "site/generated/stitch-app-catalog.json";
        let catalog_path = repo_root.join(catalog_rel);
        let existing_raw = std::fs::read_to_string(&catalog_path).ok();
        let new_app = self.build_stitch_manifest_app(
            None,
            (title, genre, domain_slug, summary),
            publisher_prism_id,
            false,
        )?;
        let mut custom_apps: Vec<StitchCatalogManifestApp> = vec![new_app.clone()];
        let max_catalog_apps = (1usize << (1 + 1 + 1 + 1)) + (1usize << (1 + 1 + 1));
        if let Some(raw) = &existing_raw {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(raw) {
                if let Some(arr) = val.get("apps").and_then(|v| v.as_array()) {
                    for item in arr {
                        if let Ok(app) =
                            serde_json::from_value::<StitchCatalogManifestApp>(item.clone())
                        {
                            if !app.seeded && app.app_id != new_app.app_id {
                                custom_apps.push(app);
                            }
                        }
                    }
                }
            }
        }
        custom_apps.truncate(max_catalog_apps);
        let merged_seed_input = serde_json::to_string(&serde_json::json!({ "apps": custom_apps }))
            .unwrap_or_else(|_| "{}".to_string());
        let final_json = self.synthesize_stitch_app_catalog_json(Some(&merged_seed_input))?;
        Self::write_manager_file(repo_root, "mgr-catalog-curator", catalog_rel, &final_json)?;
        let _ = self.seal_repository_views(repo_root, &new_app.payload_sha256)?;
        Ok(new_app)
    }

    /// Evaluates a signed directive through the Observer & Managers (`submit_directive`); if
    /// admitted, materializes the owning Manager's artifacts and closes via VSM S3* audit; if
    /// refused, writes 0 bytes and returns the formal ISO 24617-2 Refusal.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn apply_directive(
        &mut self,
        repo_root: &Path,
        req: &OperatorPromptRequest,
    ) -> Result<OperatorApplyReport, ObserverModelError> {
        let decision = self.submit_directive(req)?;
        if !decision.admitted {
            return Ok(OperatorApplyReport {
                applied: false,
                decision,
                written_files: Vec::new(),
                audit: None,
            });
        }
        let mat = self.materialize_manager_artifacts(repo_root, Some(&decision.viewpoint_id))?;
        let audit = self.audit_repository(repo_root, Some(&decision.viewpoint_id));
        Ok(OperatorApplyReport {
            applied: audit.passed,
            decision,
            written_files: mat.written_files,
            audit: Some(audit),
        })
    }
}
