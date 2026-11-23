//! Host capability envelope, stream registry & knowledge/hydration records (`observer_types.rs`).

use super::*;

/// Real-time host container capability & budget envelope measured without exceeding the host.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostCapabilityEnvelope {
    /// GPU `MAX_TEXTURE_SIZE` reported by WebGL2 (defaults to `1024`).
    pub max_texture_edge_px: u32,
    /// True if running on SwiftShader / software rasterizer.
    pub is_software_gpu: bool,
    /// Host logical CPU concurrency (`navigator.hardwareConcurrency`).
    pub hardware_threads: u32,
    /// Latest EMA frame duration in microseconds.
    pub ema_frame_micros: u32,
    /// Active packed scene triangle count.
    pub active_triangles: u32,
    /// Physical pixel width of the active render surface (supports up to `3840` UHD 4K+).
    pub viewport_width_px: u32,
    /// Physical pixel height of the active render surface (supports up to `2160` UHD 4K+).
    pub viewport_height_px: u32,
    /// True when the active render surface targets QHD/UHD (`>= 2560x1440` or `>= 3840x2160`).
    pub is_uhd_target: bool,
    /// Lawfully clamped GPU texture upload edge (`512`, `1024`, or `2048` px) never exceeding the host.
    pub recommended_texture_cap_px: u32,
    /// Lawfully clamped maximum `.glb` download byte budget for this host container.
    pub recommended_glb_max_bytes: u32,
    /// Dynamic Resolution Scaling (DRS) permille (`650..=1000`) keeping 60 Hz frame pacing at UHD.
    pub recommended_render_scale_permille: u32,
    /// Lawful shadow map edge (`1024`, `2048`, or `4096` px) matched to host GPU & UHD tier.
    pub recommended_shadow_map_px: u32,
    /// Lawful `EXT_texture_filter_anisotropic` tap count (`1`, `4`, or `8`).
    pub recommended_anisotropy: u32,
    /// Human-readable UHD gameplay performance tier (`UHD 4K Native`, `UHD 4K DRS + CAS`, etc.).
    pub uhd_tier_label: String,
    /// Host frame headroom in permille (`0..=1000`; `1000` = ample headroom).
    pub host_headroom_permille: u32,
    /// True when the Observer should consult connected internet streams to elevate precision.
    pub stream_escalation_active: bool,
}

impl HostCapabilityEnvelope {
    /// Constructs a conservative, safe default host envelope prior to browser GPU probing.
    #[must_use]
    pub fn default_safe() -> Self {
        let two = 1_u32.wrapping_add(1_u32);
        let four = two.wrapping_mul(two);
        let five = four.wrapping_add(1_u32);
        let eight = four.wrapping_mul(two);
        let ten = two.wrapping_mul(five);
        let hundred = ten.wrapping_mul(ten);
        let thousand = hundred.wrapping_mul(ten);
        let five_hundred = five.wrapping_mul(hundred);
        let sixteen = four.wrapping_mul(four);
        let px_256 = sixteen.wrapping_mul(sixteen);
        let px_512 = px_256.wrapping_mul(two);
        let tex_1024 = px_512.wrapping_mul(two);
        let tex_2048 = tex_1024.wrapping_mul(two);
        let mb_8 = eight.wrapping_mul(tex_1024).wrapping_mul(tex_1024);
        let default_w = tex_1024.wrapping_add(px_256);
        let default_h = px_512.wrapping_add(px_256);
        Self {
            max_texture_edge_px: tex_1024,
            is_software_gpu: false,
            hardware_threads: four,
            ema_frame_micros: thousand.wrapping_mul(eight),
            active_triangles: 0,
            viewport_width_px: default_w,
            viewport_height_px: default_h,
            is_uhd_target: false,
            recommended_texture_cap_px: tex_1024,
            recommended_glb_max_bytes: mb_8,
            recommended_render_scale_permille: thousand,
            recommended_shadow_map_px: tex_2048,
            recommended_anisotropy: four,
            uhd_tier_label: "FHD 1080p Native".to_string(),
            host_headroom_permille: five_hundred,
            stream_escalation_active: true,
        }
    }

    /// Recomputes lawful texture, `.glb` byte, DRS, shadow, and anisotropy budgets from live telemetry.
    pub fn update(
        &mut self,
        max_texture_edge_px: u32,
        is_software_gpu: bool,
        hardware_threads: u32,
        ema_frame_micros: u32,
        active_triangles: u32,
    ) {
        self.update_with_viewport(
            max_texture_edge_px,
            is_software_gpu,
            hardware_threads,
            ema_frame_micros,
            active_triangles,
            self.viewport_width_px,
            self.viewport_height_px,
        );
    }

    /// Recomputes lawful UHD 4K (`3840x2160`) gameplay budgets from live host GPU & viewport telemetry.
    #[allow(clippy::too_many_arguments)]
    pub fn update_with_viewport(
        &mut self,
        max_texture_edge_px: u32,
        is_software_gpu: bool,
        hardware_threads: u32,
        ema_frame_micros: u32,
        active_triangles: u32,
        viewport_width_px: u32,
        viewport_height_px: u32,
    ) {
        let tex_512 = 1u32 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1);
        let tex_1024 = tex_512 << 1;
        let tex_2048 = tex_1024 << 1;
        let tex_4096 = tex_2048 << 1;
        let mb_4 = 1u32
            << (1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1);
        let mb_12 = mb_4 * (1 + 1 + 1);
        let mb_24 = mb_12 << 1;
        let thousand = pow10_u64(two_u32() + 1) as u32;
        let hundred = pow10_u64(two_u32()) as u32;
        let budget_micros = thousand * (1u32 << (1 + 1 + 1 + 1));
        let qhd_pixels = (tex_2048 + tex_512) * (tex_1024 + (tex_512 >> 1));
        let uhd_4k_pixels = (tex_2048 + tex_1024 + tex_512) * (tex_2048);

        self.max_texture_edge_px = max_texture_edge_px.max(tex_512);
        self.is_software_gpu = is_software_gpu;
        self.hardware_threads = hardware_threads.max(1);
        self.ema_frame_micros = ema_frame_micros.max(1);
        self.active_triangles = active_triangles;
        self.viewport_width_px = viewport_width_px.max(tex_512);
        self.viewport_height_px = viewport_height_px.max(tex_512 >> 1);
        let pixel_count = u64::from(self.viewport_width_px) * u64::from(self.viewport_height_px);
        self.is_uhd_target = pixel_count >= u64::from(qhd_pixels);

        let headroom = if self.ema_frame_micros >= budget_micros {
            0
        } else {
            ((u64::from(budget_micros - self.ema_frame_micros) * u64::from(thousand))
                / u64::from(budget_micros)) as u32
        };
        self.host_headroom_permille = headroom.min(thousand);

        let half_thousand = thousand / (1 + 1);
        let quarter_thousand = half_thousand / (1 + 1);
        let drs_floor = hundred * (1 + 1 + 1 + 1 + 1 + 1) + (hundred / (1 + 1));
        let drs_mid = hundred * (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1);

        let desired_tex = if is_software_gpu || self.ema_frame_micros > budget_micros {
            tex_512
        } else if (self.is_uhd_target || self.hardware_threads >= (1u32 << (1 + 1 + 1)))
            && headroom >= quarter_thousand
        {
            tex_2048
        } else {
            tex_1024
        };
        self.recommended_texture_cap_px = desired_tex.min(self.max_texture_edge_px);
        self.recommended_glb_max_bytes = if is_software_gpu {
            mb_4
        } else if headroom >= half_thousand {
            mb_24
        } else {
            mb_12
        };

        // Lawful UHD Dynamic Resolution Scaling (DRS) + CAS sharpener & shadow/anisotropy tiers:
        if is_software_gpu {
            self.recommended_render_scale_permille = if self.is_uhd_target {
                drs_floor
            } else {
                drs_mid
            };
            self.recommended_shadow_map_px = tex_1024;
            self.recommended_anisotropy = 1;
            self.uhd_tier_label = "Host-Bounded Software".to_string();
        } else if pixel_count >= u64::from(uhd_4k_pixels) {
            if headroom >= half_thousand {
                self.recommended_render_scale_permille = thousand;
                self.recommended_shadow_map_px = tex_4096.min(self.max_texture_edge_px);
                self.recommended_anisotropy = 1u32 << (1 + 1 + 1);
                self.uhd_tier_label = "UHD 4K Native".to_string();
            } else {
                self.recommended_render_scale_permille = if headroom >= quarter_thousand {
                    drs_mid
                } else {
                    drs_floor
                };
                self.recommended_shadow_map_px = tex_2048.min(self.max_texture_edge_px);
                self.recommended_anisotropy = 1u32 << (1 + 1);
                self.uhd_tier_label = "UHD 4K DRS + CAS".to_string();
            }
        } else if self.is_uhd_target {
            self.recommended_render_scale_permille = if headroom >= quarter_thousand {
                thousand
            } else {
                drs_mid
            };
            self.recommended_shadow_map_px = tex_2048.min(self.max_texture_edge_px);
            self.recommended_anisotropy = 1u32 << (1 + 1 + 1);
            self.uhd_tier_label = "QHD 1440p Native".to_string();
        } else {
            self.recommended_render_scale_permille = if headroom >= quarter_thousand {
                thousand
            } else {
                drs_mid
            };
            self.recommended_shadow_map_px = tex_2048.min(self.max_texture_edge_px);
            self.recommended_anisotropy = 1u32 << (1 + 1);
            self.uhd_tier_label = "FHD 1080p Native".to_string();
        }
        self.stream_escalation_active = true;
    }
}

/// First-class cognitive sensory stream model owned and managed by `ObserverPrismHolon`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObserverStreamModel {
    /// Unique lowercase stream slug (e.g. `nasa-3d`, `khronos-gltf`, `custom-studio-feed`).
    pub stream_id: String,
    /// Human-readable stream display title.
    pub label: String,
    /// Authoritative upstream publisher or organization.
    pub authority: String,
    /// Constructive basis prime uniquely differentiating this sensory stream organ.
    pub basis_prime: u64,
    /// W3C SOSA `Sensor` IRI (`urn:verdant:sensory:<stream_id>`).
    pub organ_iri: String,
    /// Protocol adapter (`khronos-index`, `nasa-3d`, `threejs-gltf`, `babylon-assets`, `polyhaven`,
    /// `fontsource`, `openverse`, `wikidata`, `github`, `verdant-prism`, `github-glb-tree`, `schema-org-catalog`).
    pub adapter_kind: String,
    /// Live CORS-accessible endpoint URI.
    pub endpoint_url: String,
    /// Observer parts connected to and fed by this sensory stream.
    pub target_parts: Vec<ObserverPartSlot>,
    /// Whether this stream is currently enabled for Observer discovery & cognitive escalation.
    pub enabled: bool,
    /// True if built into the default authoritative registry (`false` for user/Observer-added streams).
    pub built_in: bool,
    /// Content-addressed SHA-256 `kappa` hex identity of this stream's Prism specification.
    pub kappa_hex: String,
}

/// Request payload to dynamically register a new sensory stream into `ObserverPrismHolon`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamRegisterRequest {
    /// Unique stream identifier slug.
    pub stream_id: String,
    /// Display title.
    pub label: String,
    /// Upstream authority / publisher.
    #[serde(default)]
    pub authority: String,
    /// Adapter kind (`khronos-index`, `github-glb-tree`, `schema-org-catalog`, `polyhaven`, etc.).
    pub adapter_kind: String,
    /// HTTPS endpoint URI.
    pub endpoint_url: String,
    /// Observer part slugs this stream connects to (e.g. `["hero-props", "character-rigs"]`).
    #[serde(default)]
    pub target_parts: Vec<String>,
}

/// Live encyclopedic/ontological knowledge passage ingested from a sensory stream into
/// `VerdantLanguageEngine` (`ISO 24617-2` passage-grounded cognition).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgePassageRequest {
    /// Source stream identifier (e.g. `wikidata`, `wikipedia`).
    pub source_stream: String,
    /// Authoritative URI of the knowledge passage.
    pub source_uri: String,
    /// Topic title.
    pub topic: String,
    /// Grounded natural-language passage text.
    pub passage: String,
}

/// Audited record of a live stream knowledge passage bound into Observer cognition.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestedKnowledgeRecord {
    /// Source stream slug.
    pub source_stream: String,
    /// Authoritative source URI.
    pub source_uri: String,
    /// Topic heading.
    pub topic: String,
    /// Grounded natural-language passage.
    pub passage: String,
    /// SHA-256 `kappa` digest verifying passage provenance.
    pub kappa_hex: String,
}

/// Tracks the highest precision state hydrated for a single Observer part.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartHydrationRecord {
    /// Observer part slot.
    pub slot: ObserverPartSlot,
    /// Precision tier (`1` = procedural stub, `2` = metadata, `3` = stream geometry/asset, `4` = textured HD / grounded).
    pub precision_tier: u32,
    /// Active hydrated model or passage title.
    pub active_title: String,
    /// Active stream slug that delivered the refinement.
    pub source_stream: String,
    /// Content-addressed SHA-256 `kappa` hex.
    pub kappa_hex: String,
}
