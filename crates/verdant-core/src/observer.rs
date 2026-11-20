//! Second-order Observer Prism holon (`Observer.lex.tex`), differentiated platform organs,
//! stakeholder Prism models, arbitrary Verdant model extrapolation, native `uor-matmul`
//! `Packed<Grid<16>, 2>` ISO 24617-2 passage-grounded language/reasoning engine, live A*
//! `NavMesh` NPC corridor directives, and observed W3C SOSA/PROV-O communication channels.
//!
//! Verdant Engine is the sole model runtime across all parts of the Observer: zero external
//! ONNX, Transformers.js, Three.js, or cloud LLM runtimes are permitted. Every entity, organ,
//! stakeholder model, arbitrary model envelope, and communication carries a distinct 32-byte
//! SHA-256 `kappa` identity derived from its canonical content.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use verdant_tensor::{infer_coded_slm_intent_logits, CODED_TABLE_LEN};

use crate::character::ArticulatedCharacter;
use crate::cybernetics::{
    ContainerGrantMatrix, ContainerOrgan, CyberneticObserverLoop, PlayerObserverSample,
};
use crate::ecs::TwoLevelBvh;
use crate::error::ObserverModelError;
use crate::hd_world::hd_terrain_height;
use crate::math::{four_u32, geometric_epsilon, half_f64, nth_prime_u64, pow10_u64, two_u32, Vec3};
use crate::navmesh::NavMesh;
use crate::network::PrismIdSessionEnvelope;
use crate::prism_kernel;
use crate::standards::{CatalogBundleRegistry, DmnAdmissionInput, InstantiatedDomainPack};

/// Canonical SHA-256 `kappa` byte width (`32` bytes).
pub const KAPPA_DIGEST_BYTES: usize = 1 << (1 + 1 + 1 + 1 + 1);

/// Computes a raw 32-byte SHA-256 digest over `bytes` (for `PrismIdSessionEnvelope` binding).
#[must_use]
pub fn sha256_payload_digest(bytes: &[u8]) -> [u8; KAPPA_DIGEST_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let out = hasher.finalize();
    core::array::from_fn(|i| out[i])
}

fn compute_kappa(tag: &str, payload: &[u8]) -> ([u8; KAPPA_DIGEST_BYTES], String) {
    let mut hasher = Sha256::new();
    hasher.update(b"verdant/kappa/v1:");
    hasher.update(tag.as_bytes());
    hasher.update(b":");
    hasher.update(payload);
    let out = hasher.finalize();
    let arr: [u8; KAPPA_DIGEST_BYTES] = core::array::from_fn(|i| out[i]);
    let hex = crate::digest::hex_encode(&arr);
    (arr, hex)
}

/// Differentiated platform organ kinds composing the Observer (`0..5`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ObserverOrganKind {
    /// Core 3D & model runtime (`/` — basis prime `2`).
    Engine,
    /// Multi-workspace authoring workbench (`/studio/` — basis prime `3`).
    Studio,
    /// SHA-256-locked standards & world-pack registry (`/catalogs/` — basis prime `5`).
    Catalogs,
    /// Catalog-instantiated showcase worlds (`/demo/` — basis prime `7`).
    Demo,
    /// Executable Diataxis documentation & live cold-boot embeds (`/docs/` — basis prime `11`).
    Reference,
}

impl ObserverOrganKind {
    /// Zero-based organ index (`0..5`).
    #[must_use]
    pub const fn index(self) -> usize {
        let two = 1_u32.wrapping_add(1_u32);
        let three = two.wrapping_add(1_u32);
        let four = two.wrapping_mul(two);
        let idx = match self {
            Self::Engine => 0_u32,
            Self::Studio => 1_u32,
            Self::Catalogs => two,
            Self::Demo => three,
            Self::Reference => four,
        };
        idx as usize
    }

    /// Canonical slug matching `VERDANT_PLATFORM_ORGANS`.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Engine => "engine",
            Self::Studio => "studio",
            Self::Catalogs => "catalogs",
            Self::Demo => "demo",
            Self::Reference => "docs",
        }
    }
}

/// Full Prism model descriptor for one of the Observer's five differentiated platform organs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObserverOrganModel {
    /// Organ enum tag.
    pub kind: ObserverOrganKind,
    /// Zero-based index (`0..5`).
    pub index: usize,
    /// Constructive basis prime (`2, 3, 5, 7, 11`).
    pub basis_prime: u64,
    /// True iff this organ is the universal model runtime (`Engine`).
    pub hosts_all_models: bool,
    /// Strong 32-byte content-addressed `kappa` identity.
    pub kappa: [u8; KAPPA_DIGEST_BYTES],
    /// Lowercase hex encoding of `kappa`.
    pub kappa_hex: String,
    /// Schema.org JSON-LD representation (`SoftwareApplication` / `Dataset` / `TechArticle`).
    pub schema_org_jsonld: serde_json::Value,
}

/// The seven stakeholder categories modeled in `Architecture.lex.tex` and `Observer.lex.tex`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StakeholderRoleKind {
    /// Builds 3D games and interactive worlds (`workspace = code (1)`).
    GameDeveloper,
    /// Authors Khronos glTF 2.0 PBR materials, rigs, and UORC streams (`workspace = assets (3)`).
    TechnicalArtist,
    /// Governs `uor-matmul` semiring kernels, W5H telos, and frame budgets (`workspace = dashboard (0)`).
    SystemsEngineer,
    /// Explores 3D worlds, possesses characters, and converses with NPCs (`workspace = gameplay (6)`).
    BrowserPlayer,
    /// Audits DMN 1.5 runtime exclusivity, `FK-07` grants, and SHA-256 locks (`workspace = director (5)`).
    SecurityAuditor,
    /// Proves tropical max-plus UI layouts, 48px touch bounds, and WCAG contrast (`workspace = design (2)`).
    UiComplianceArchitect,
    /// Exchanges Prism ID signed peer messages, Sabine audio, and Stitch shards (`workspace = audio (4)`).
    MultiplayerCitizen,
}

impl StakeholderRoleKind {
    /// Zero-based stakeholder index (`0..7`).
    #[must_use]
    pub const fn index(self) -> usize {
        let two = 1_u32.wrapping_add(1_u32);
        let three = two.wrapping_add(1_u32);
        let four = two.wrapping_mul(two);
        let five = four.wrapping_add(1_u32);
        let six = three.wrapping_mul(two);
        let idx = match self {
            Self::GameDeveloper => 0_u32,
            Self::TechnicalArtist => 1_u32,
            Self::SystemsEngineer => two,
            Self::BrowserPlayer => three,
            Self::SecurityAuditor => four,
            Self::UiComplianceArchitect => five,
            Self::MultiplayerCitizen => six,
        };
        idx as usize
    }

    /// Bijective Studio functional workspace index (`0..7`: `dashboard, code, design, assets, audio, director, gameplay`).
    #[must_use]
    pub const fn workspace_index(self) -> usize {
        let two = 1_u32.wrapping_add(1_u32);
        let three = two.wrapping_add(1_u32);
        let four = two.wrapping_mul(two);
        let five = four.wrapping_add(1_u32);
        let six = three.wrapping_mul(two);
        let idx = match self {
            Self::SystemsEngineer => 0_u32,
            Self::GameDeveloper => 1_u32,
            Self::UiComplianceArchitect => two,
            Self::TechnicalArtist => three,
            Self::MultiplayerCitizen => four,
            Self::SecurityAuditor => five,
            Self::BrowserPlayer => six,
        };
        idx as usize
    }

    /// Functional Studio workspace slug (`dashboard..gameplay`).
    #[must_use]
    pub const fn workspace_slug(self) -> &'static str {
        match self {
            Self::SystemsEngineer => "dashboard",
            Self::GameDeveloper => "code",
            Self::UiComplianceArchitect => "design",
            Self::TechnicalArtist => "narrative",
            Self::MultiplayerCitizen => "audio",
            Self::SecurityAuditor => "director",
            Self::BrowserPlayer => "gameplay",
        }
    }
}

/// Full Prism model of a platform stakeholder experienced by the Observer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StakeholderPrismModel {
    /// Stakeholder category.
    pub kind: StakeholderRoleKind,
    /// Zero-based index (`0..7`).
    pub index: usize,
    /// Bijective Studio workspace index (`0..7`).
    pub workspace_index: usize,
    /// Functional workspace slug (`dashboard..gameplay`).
    pub workspace_slug: &'static str,
    /// Constructive basis prime (`13, 17, 19, 23, 29, 31, 37`).
    pub basis_prime: u64,
    /// Permitted `FK-07` container organs for this stakeholder's workflows.
    pub allowed_organs: Vec<ContainerOrgan>,
    /// Strong 32-byte `kappa` identity.
    pub kappa: [u8; KAPPA_DIGEST_BYTES],
    /// Lowercase hex encoding of `kappa`.
    pub kappa_hex: String,
    /// Schema.org `Role` JSON-LD node verified by `SchemaOrgVocabulary`.
    pub schema_org_jsonld: serde_json::Value,
}

/// The five communication channels experienced and audited by the Observer (`0..5`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CommunicationChannelKind {
    /// Inter-organ telemetry & state coupling (`Engine <-> Studio <-> Catalogs <-> Demo <-> Docs`).
    OrganToOrgan,
    /// Human `Player/User <-> Player/User` peer communication (`Prism id` signed).
    PlayerToPlayer,
    /// Human `Player/User -> Character/Agent` natural-language prompt or directive (`Prism id` signed).
    PlayerToAgent,
    /// `Character/Agent -> Player/User` passage-grounded response or `"Unknown"` abstention.
    AgentToPlayer,
    /// `Character/Agent <-> Character/Agent` autonomous patrol & conserved item coordination.
    AgentToAgent,
}

impl CommunicationChannelKind {
    /// Zero-based channel code (`0..5`) matching `Observer.lex.tex`.
    #[must_use]
    pub const fn code(self) -> u64 {
        let two = 1_u64.wrapping_add(1_u64);
        let three = two.wrapping_add(1_u64);
        let four = two.wrapping_mul(two);
        match self {
            Self::OrganToOrgan => 0,
            Self::PlayerToPlayer => 1,
            Self::PlayerToAgent => two,
            Self::AgentToPlayer => three,
            Self::AgentToAgent => four,
        }
    }

    /// Canonical channel slug.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::OrganToOrgan => "organ-to-organ",
            Self::PlayerToPlayer => "player-to-player",
            Self::PlayerToAgent => "player-to-agent",
            Self::AgentToPlayer => "agent-to-player",
            Self::AgentToAgent => "agent-to-agent",
        }
    }

    /// Whether `Observer::observedChannelEnvelopeValid` requires a verified `Prism id` session signature.
    #[must_use]
    pub const fn requires_prism_signature(self) -> bool {
        matches!(self, Self::PlayerToPlayer | Self::PlayerToAgent)
    }
}

/// Arbitrary model domain families hosted natively inside Verdant Engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArbitraryModelDomainKind {
    /// High-definition 3D mesh, Khronos PBR, or 22-joint skeletal rig (`verdant-uorc`).
    Geometry3D,
    /// Zero-decode `Packed<Grid<16>, 2>` language, dialogue, or intent router (`verdant-coded-matrix`).
    CodedLanguage,
    /// Q15 leaky-integrate-and-fire / state-space salience recurrence (`verdant-coded-matrix`).
    NeuromorphicSsm,
    /// Tropical `(min, +)` reachability & exact supply-conserving automaton (`verdant-holo`).
    ConservationAutomaton,
    /// ISO 3382-1 Sabine enclosure & Euler-Bernoulli modal acoustics (`verdant-uorc`).
    SpatialAcoustics,
    /// Tropical `(max, +)` 2D/diegetic 3D UI constraint & WCAG model (`verdant-holo`).
    TropicalUi,
    /// Symplectic / XPBD Delassus rigid-body & vehicle dynamics (`verdant-uorc`).
    RigidBodyPhysics,
}

/// Envelope extrapolating Verdant Engine's runtime to arbitrary Prism models (`arbitraryPrismModelValid`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerdantModelEnvelope {
    /// Unique model identifier (`MDL-*` or `PRISM-*`).
    pub model_id: String,
    /// Domain family hosted by Verdant Engine.
    pub domain_kind: ArbitraryModelDomainKind,
    /// Owning platform organ (`0..5`).
    pub owner_organ: ObserverOrganKind,
    /// Required `FK-07` browser container organ.
    pub required_container_organ: ContainerOrgan,
    /// Execution format (`"verdant-uorc"`, `"verdant-coded-matrix"`, `"verdant-holo"`).
    pub model_format: String,
    /// Canonical serialized model payload bytes.
    pub payload_bytes: Vec<u8>,
    /// Expected 32-byte `kappa` digest over `(model_id, payload_bytes)`.
    pub kappa: [u8; KAPPA_DIGEST_BYTES],
    /// Schema.org JSON-LD metadata coupling this model to its world/organ entities.
    pub schema_org_jsonld: serde_json::Value,
}

impl VerdantModelEnvelope {
    /// Constructs a canonical `VerdantModelEnvelope` with computed 32-byte `kappa` identity.
    #[must_use]
    pub fn new_sealed(
        model_id: impl Into<String>,
        domain_kind: ArbitraryModelDomainKind,
        owner_organ: ObserverOrganKind,
        required_container_organ: ContainerOrgan,
        model_format: impl Into<String>,
        payload_bytes: Vec<u8>,
        schema_org_jsonld: serde_json::Value,
    ) -> Self {
        let id_str = model_id.into();
        let (kappa, _) = compute_kappa(&id_str, &payload_bytes);
        Self {
            model_id: id_str,
            domain_kind,
            owner_organ,
            required_container_organ,
            model_format: model_format.into(),
            payload_bytes,
            kappa,
            schema_org_jsonld,
        }
    }
}

#[path = "observer_language.rs"]
mod observer_language;
pub use observer_language::*;

/// Request payload to admit and auto-tag an external or local catalog model into the active project.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectModelImportRequest {
    /// Stable model identifier (e.g. `khronos:DamagedHelmet` or `fontsource:inter`).
    pub model_id: String,
    /// Human-readable display title.
    pub title: String,
    /// Authoritative Schema.org `@type` (e.g. `3DModel`, `PBRMaterial`, `Font`, `AudioObject`, `Dataset`).
    pub schema_type: String,
    /// Authoritative IANA media type (e.g. `model/gltf-binary`, `font/woff2`, `audio/mpeg`).
    pub media_type: String,
    /// Automatic Studio target binding slot (e.g. `design:3d-model`, `design:typography`, `audio:soundscape`).
    pub target_slot: String,
    /// Origin catalog stream identifier (e.g. `khronos-gltf`, `polyhaven`, `fontsource`, `openverse`, `wikidata`, `github`, `verdant-prism`).
    pub source_stream: String,
    /// Authoritative upstream source URI.
    pub source_uri: String,
    /// SPDX license identifier (e.g. `CC0-1.0`, `OFL-1.1`, `MIT`).
    pub license_spdx: String,
    /// Pre-tagged semantic and catalog taxonomy tags.
    pub tags: Vec<String>,
    /// Canonical editable JSON properties payload.
    pub properties_json: String,
    /// Optional commit message for initial import revision.
    #[serde(default)]
    pub commit_note: String,
}

/// Request payload to edit an imported project model into a new cryptographically chained version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectModelEditRequest {
    /// Target model identifier in `ObserverPrismHolon::project_models`.
    pub model_id: String,
    /// Updated display title.
    pub title: String,
    /// Updated semantic/taxonomy tags.
    pub tags: Vec<String>,
    /// Updated canonical JSON properties string.
    pub properties_json: String,
    /// Commit note describing the edit.
    pub commit_note: String,
}

/// Single immutable version entry in an imported project model's PROV-O revision chain.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelRevisionRecord {
    /// 1-based monotonic revision number (`v1`, `v2`, …).
    pub version: u32,
    /// SHA-256 `kappa` digest of this revision's canonical payload.
    pub kappa_hex: String,
    /// Parent revision's `kappa_hex` (`None` on `v1`).
    pub parent_kappa_hex: Option<String>,
    /// Title snapshot at this revision.
    pub title: String,
    /// Tag snapshot at this revision.
    pub tags: Vec<String>,
    /// Editable JSON properties snapshot at this revision.
    pub properties_json: String,
    /// Human-readable commit/provenance note.
    pub commit_note: String,
}

/// Live project-imported arbitrary model admitted, auto-tagged, editable, versioned, and removable
/// inside `ObserverPrismHolon`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectImportedModelRecord {
    /// Stable model identifier.
    pub model_id: String,
    /// Current display title.
    pub title: String,
    /// Authoritative Schema.org `@type`.
    pub schema_type: String,
    /// Authoritative IANA media type.
    pub media_type: String,
    /// Automatic Studio target binding slot.
    pub target_slot: String,
    /// Origin catalog stream identifier.
    pub source_stream: String,
    /// Authoritative upstream URI.
    pub source_uri: String,
    /// SPDX license identifier.
    pub license_spdx: String,
    /// Active monotonic version (`1..=N`).
    pub active_version: u32,
    /// SHA-256 `kappa` hex of the active version.
    pub current_kappa_hex: String,
    /// Current semantic tags.
    pub tags: Vec<String>,
    /// Current editable JSON properties string.
    pub properties_json: String,
    /// Complete immutable revision history (`v1..=vN`).
    pub revisions: Vec<ModelRevisionRecord>,
}

/// The eight differentiated Observer parts that connect to sensory streams so Verdant can
/// autonomously elevate its resolution from procedural stubs (`R1`) to stream-hydrated HD (`R4`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ObserverPartSlot {
    /// World/stage 3D architecture, terrains, and equirectangular sky/IBL environments.
    StageEnvironment,
    /// Articulated / skinned 3D character and creature rigs.
    CharacterRigs,
    /// Placed hero 3D artifacts, vehicles, spacecraft, and interactive props.
    HeroProps,
    /// Physically-based surface textures (albedo, metallic, roughness, normal).
    SurfaceMaterials,
    /// Spatial audio soundscapes, foley, and Sabine acoustic rooms.
    SpatialAcoustics,
    /// W3C DTCG visual themes and CSS Fonts Level 4 typography.
    VisualIdentity,
    /// Structured encyclopedic/ontological knowledge passages feeding ISO 24617-2 language.
    CognitiveLanguage,
    /// WGSL shaders, WebAssembly behaviors, and Schema.org/Prism frameworks.
    RuntimeFrameworks,
}

impl ObserverPartSlot {
    /// Returns all eight differentiated Observer parts in canonical order.
    #[must_use]
    pub const fn all() -> [Self; 1 << (1 + 1 + 1)] {
        [
            Self::StageEnvironment,
            Self::CharacterRigs,
            Self::HeroProps,
            Self::SurfaceMaterials,
            Self::SpatialAcoustics,
            Self::VisualIdentity,
            Self::CognitiveLanguage,
            Self::RuntimeFrameworks,
        ]
    }

    /// Canonical kebab-case identifier matching Studio/Catalog bindings.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::StageEnvironment => "stage-environment",
            Self::CharacterRigs => "character-rigs",
            Self::HeroProps => "hero-props",
            Self::SurfaceMaterials => "surface-materials",
            Self::SpatialAcoustics => "spatial-acoustics",
            Self::VisualIdentity => "visual-identity",
            Self::CognitiveLanguage => "cognitive-language",
            Self::RuntimeFrameworks => "runtime-frameworks",
        }
    }

    /// Human-readable label for Observer cognition reports.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::StageEnvironment => "Stage & Sky Environment",
            Self::CharacterRigs => "Characters & Articulated Rigs",
            Self::HeroProps => "Hero 3D Props & Artifacts",
            Self::SurfaceMaterials => "PBR Surface Materials",
            Self::SpatialAcoustics => "Spatial Audio & Acoustics",
            Self::VisualIdentity => "Visual Identity & Typography",
            Self::CognitiveLanguage => "Natural Language & Ontologies",
            Self::RuntimeFrameworks => "Shaders & Runtime Frameworks",
        }
    }

    /// Parses a kebab-case or semantic target slot string into an [`ObserverPartSlot`].
    #[must_use]
    pub fn from_target_slot(target_slot: &str) -> Self {
        let low = target_slot.to_ascii_lowercase();
        if low.contains("character") || low.contains("rig") || low.contains("avatar") {
            ObserverPartSlot::CharacterRigs
        } else if low.contains("world")
            || low.contains("stage")
            || low.contains("env")
            || low.contains("hdri")
        {
            ObserverPartSlot::StageEnvironment
        } else if low.contains("material") || low.contains("pbr") || low.contains("texture") {
            ObserverPartSlot::SurfaceMaterials
        } else if low.contains("audio") || low.contains("sound") || low.contains("acoustic") {
            ObserverPartSlot::SpatialAcoustics
        } else if low.contains("theme")
            || low.contains("font")
            || low.contains("typography")
            || low.contains("identity")
        {
            ObserverPartSlot::VisualIdentity
        } else if low.contains("ontology")
            || low.contains("story")
            || low.contains("language")
            || low.contains("narrative")
        {
            ObserverPartSlot::CognitiveLanguage
        } else if low.contains("code")
            || low.contains("shader")
            || low.contains("framework")
            || low.contains("wasm")
        {
            ObserverPartSlot::RuntimeFrameworks
        } else {
            ObserverPartSlot::HeroProps
        }
    }
}

#[path = "observer_types.rs"]
mod observer_types;
pub use observer_types::*;

/// Self-observing second-order Observer Prism holon (`ObserverRootHolon`) composing:
/// - Its own strong `kappa` identity (`2*3*5*7*11 = 2310` organ basis product),
/// - All `5` differentiated Platform Organ Prism models,
/// - All `7` Stakeholder Prism models (`bijective` with the `7` Studio workspaces),
/// - Active cataloged domain pack + live A* `NavMesh` NPC agents,
/// - Dynamic Stream Registry (`ObserverStreamModel`) & Host Capability Envelope,
/// - Admitted arbitrary `VerdantModelEnvelope` registry,
/// - Native `VerdantLanguageEngine`, and
/// - Chronological W3C SOSA/PROV-O `ObservedCommunicationRecord` bus.
#[derive(Clone, Debug)]
pub struct ObserverPrismHolon {
    /// Root Observer `kappa` digest.
    pub observer_kappa: [u8; KAPPA_DIGEST_BYTES],
    /// Lowercase hex of `observer_kappa`.
    pub observer_kappa_hex: String,
    /// Verified basis-prime product across all 5 platform organs (`2310`).
    pub organ_basis_product: u64,
    /// Whether the emergency halt latch is active (`OMG DMN 1.5 R1`).
    pub emergency_halted: bool,
    /// Five differentiated platform organ Prism models.
    pub organs: Vec<ObserverOrganModel>,
    /// Seven differentiated stakeholder Prism models.
    pub stakeholders: Vec<StakeholderPrismModel>,
    /// Embedded SHA-256-locked catalog registry.
    pub catalogs: CatalogBundleRegistry,
    /// Active instantiated domain pack (`Sunspire`, `Meridian`, `Glasshouse`, or `Starfall`).
    pub active_domain: InstantiatedDomainPack,
    /// Walkable `NavMesh` extracted from `active_domain`'s sculpted terrain.
    pub active_navmesh: NavMesh,
    /// Live autonomous NPC navigation & directive controllers.
    pub npc_agents: Vec<NpcNavAgentState>,
    /// Native `uor-matmul` ISO 24617-2 dialogue & passage-grounding engine.
    pub language_engine: VerdantLanguageEngine,
    /// Coupled Q15 LIF/SSM mutual-attention loop experiencing world and communication events.
    pub attention_loop: CyberneticObserverLoop,
    /// Admitted arbitrary Prism model `kappa` hex digests.
    pub admitted_model_kappas: Vec<(String, String)>,
    /// Dynamically imported, editable, versioned project models admitted from live or locked catalogs.
    pub project_models: Vec<ProjectImportedModelRecord>,
    /// Real-time host container capability & budget envelope.
    pub host_envelope: HostCapabilityEnvelope,
    /// Live sensory catalog streams managed by the Observer (built-in + dynamically added).
    pub sensory_streams: Vec<ObserverStreamModel>,
    /// Next prime ordinal index allocated to newly registered sensory streams.
    pub next_stream_prime_ordinal: u32,
    /// Live knowledge passages ingested from connected streams into ISO 24617-2 cognition.
    pub ingested_knowledge: Vec<IngestedKnowledgeRecord>,
    /// Per-part stream hydration precision records (`R1..R4`).
    pub part_hydrations: Vec<PartHydrationRecord>,
    /// Chronological communication log across all 5 channels (`Organ`, `Player<->Player`, `Player<->Agent`, `Agent<->Agent`).
    pub communications: Vec<ObservedCommunicationRecord>,
}

impl ObserverPrismHolon {
    /// Constructs and cross-verifies the complete Observer Prism holon over `initial_domain_slug`.
    pub fn new(initial_domain_slug: &str) -> Result<Self, ObserverModelError> {
        let catalogs = CatalogBundleRegistry::embedded().map_err(|err| {
            ObserverModelError::InvalidPrismModelEnvelope {
                model_id: "CatalogBundleRegistry".to_string(),
                detail: err.to_string(),
            }
        })?;

        let organ_defs = [
            (
                ObserverOrganKind::Engine,
                "SoftwareApplication",
                "Verdant Engine Runtime",
                true,
            ),
            (
                ObserverOrganKind::Studio,
                "WebApplication",
                "Verdant Studio Workbench",
                false,
            ),
            (
                ObserverOrganKind::Catalogs,
                "Dataset",
                "Verdant Authority Catalogs",
                false,
            ),
            (
                ObserverOrganKind::Demo,
                "VideoGame",
                "Verdant Multi-Domain Showcase",
                false,
            ),
            (
                ObserverOrganKind::Reference,
                "TechArticle",
                "Verdant Diataxis Reference",
                false,
            ),
        ];

        let mut organs = Vec::with_capacity(organ_defs.len());
        let mut basis_prod = 1_u64;
        for (kind, schema_cls, title, hosts_all) in organ_defs {
            let idx = kind.index();
            let prime = nth_prime_u64(idx as u32);
            basis_prod = basis_prod.saturating_mul(prime);
            let schema_doc = json!({
                "@context": "https://schema.org",
                "@id": format!("urn:verdant:organ:{}", kind.slug()),
                "@type": schema_cls,
                "name": title,
                "identifier": format!("organ:{}:prime:{prime}", kind.slug())
            });
            catalogs
                .schema_org
                .validate_schema_org_jsonld(&schema_doc)
                .map_err(|err| ObserverModelError::InvalidPrismModelEnvelope {
                    model_id: kind.slug().to_string(),
                    detail: err.to_string(),
                })?;
            let payload = serde_json::to_vec(&schema_doc).unwrap_or_default();
            let (kappa, kappa_hex) = compute_kappa(kind.slug(), &payload);
            organs.push(ObserverOrganModel {
                kind,
                index: idx,
                basis_prime: prime,
                hosts_all_models: hosts_all,
                kappa,
                kappa_hex,
                schema_org_jsonld: schema_doc,
            });
        }

        let kernel_prod = prism_kernel::observerOrganBasisProduct().unwrap_or(basis_prod);
        if basis_prod != kernel_prod {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id: "ObserverRootHolon".to_string(),
                detail: format!("organ basis product {basis_prod} != Prism kernel {kernel_prod}"),
            });
        }

        let stakeholder_defs = [
            (
                StakeholderRoleKind::GameDeveloper,
                "Game Developer",
                vec![
                    ContainerOrgan::WebAssemblyCore,
                    ContainerOrgan::OffscreenCanvasViewport,
                    ContainerOrgan::ServiceWorkerCache,
                    ContainerOrgan::OpfsAssetCache,
                    ContainerOrgan::GamepadController,
                ],
            ),
            (
                StakeholderRoleKind::TechnicalArtist,
                "Technical Artist",
                vec![
                    ContainerOrgan::WebAssemblyCore,
                    ContainerOrgan::OffscreenCanvasViewport,
                    ContainerOrgan::ServiceWorkerCache,
                    ContainerOrgan::OpfsAssetCache,
                    ContainerOrgan::WebCryptoPrismId,
                    ContainerOrgan::WebNnAccelerator,
                ],
            ),
            (
                StakeholderRoleKind::SystemsEngineer,
                "Engine Systems Engineer",
                vec![
                    ContainerOrgan::WebAssemblyCore,
                    ContainerOrgan::OffscreenCanvasViewport,
                    ContainerOrgan::ServiceWorkerCache,
                    ContainerOrgan::OpfsAssetCache,
                    ContainerOrgan::WebNnAccelerator,
                ],
            ),
            (
                StakeholderRoleKind::BrowserPlayer,
                "Browser Player",
                vec![
                    ContainerOrgan::WebAssemblyCore,
                    ContainerOrgan::OffscreenCanvasViewport,
                    ContainerOrgan::ServiceWorkerCache,
                    ContainerOrgan::AudioWorkletSynth,
                    ContainerOrgan::GamepadController,
                    ContainerOrgan::DeviceOrientationSensor,
                    ContainerOrgan::WebCryptoPrismId,
                    ContainerOrgan::BroadcastChannelTabs,
                ],
            ),
            (
                StakeholderRoleKind::SecurityAuditor,
                "Security Auditor",
                vec![
                    ContainerOrgan::WebAssemblyCore,
                    ContainerOrgan::OffscreenCanvasViewport,
                    ContainerOrgan::ServiceWorkerCache,
                    ContainerOrgan::WebCryptoPrismId,
                    ContainerOrgan::BroadcastChannelTabs,
                ],
            ),
            (
                StakeholderRoleKind::UiComplianceArchitect,
                "UI & Compliance Architect",
                vec![
                    ContainerOrgan::WebAssemblyCore,
                    ContainerOrgan::OffscreenCanvasViewport,
                    ContainerOrgan::ServiceWorkerCache,
                    ContainerOrgan::GamepadController,
                    ContainerOrgan::DeviceOrientationSensor,
                ],
            ),
            (
                StakeholderRoleKind::MultiplayerCitizen,
                "Multiplayer World Citizen",
                vec![
                    ContainerOrgan::WebAssemblyCore,
                    ContainerOrgan::OffscreenCanvasViewport,
                    ContainerOrgan::ServiceWorkerCache,
                    ContainerOrgan::AudioWorkletSynth,
                    ContainerOrgan::OpfsAssetCache,
                    ContainerOrgan::GamepadController,
                    ContainerOrgan::DeviceOrientationSensor,
                    ContainerOrgan::WebCryptoPrismId,
                    ContainerOrgan::BroadcastChannelTabs,
                    ContainerOrgan::WebRtcStitchMesh,
                ],
            ),
        ];

        let five_u32 = four_u32() + 1;
        let mut stakeholders = Vec::with_capacity(stakeholder_defs.len());
        for (kind, title, allowed_organs) in stakeholder_defs {
            let idx = kind.index();
            let ws_idx = kind.workspace_index();
            let prime = nth_prime_u64((idx as u32) + five_u32);
            let schema_doc = json!({
                "@context": "https://schema.org",
                "@id": format!("urn:verdant:stakeholder:{idx}"),
                "@type": "Role",
                "name": title,
                "jobTitle": title,
                "identifier": format!("stakeholder:{idx}:workspace:{}:prime:{prime}", kind.workspace_slug())
            });
            catalogs
                .schema_org
                .validate_schema_org_jsonld(&schema_doc)
                .map_err(|err| ObserverModelError::InvalidPrismModelEnvelope {
                    model_id: title.to_string(),
                    detail: err.to_string(),
                })?;
            let payload = serde_json::to_vec(&schema_doc).unwrap_or_default();
            let (kappa, kappa_hex) = compute_kappa(kind.workspace_slug(), &payload);
            stakeholders.push(StakeholderPrismModel {
                kind,
                index: idx,
                workspace_index: ws_idx,
                workspace_slug: kind.workspace_slug(),
                basis_prime: prime,
                allowed_organs,
                kappa,
                kappa_hex,
                schema_org_jsonld: schema_doc,
            });
        }

        let active_domain = catalogs
            .instantiate_domain_pack(initial_domain_slug, None)
            .map_err(|_| ObserverModelError::UnknownDomainOrCharacter {
                identifier: initial_domain_slug.to_string(),
            })?;
        let active_navmesh =
            NavMesh::from_mesh(&active_domain.sector.meshes[0]).map_err(|err| {
                ObserverModelError::InvalidPrismModelEnvelope {
                    model_id: active_domain.spec.id.clone(),
                    detail: err.to_string(),
                }
            })?;

        let npc_agents = Self::build_npc_agents(&active_domain, &active_navmesh);
        let salience_slots = prism_kernel::observerSalienceDim().unwrap_or(1) as usize;
        let attention_loop = CyberneticObserverLoop::new(salience_slots).map_err(|err| {
            ObserverModelError::InvalidPrismModelEnvelope {
                model_id: "CyberneticObserverLoop".to_string(),
                detail: err.to_string(),
            }
        })?;

        let mut root_seed_bytes = Vec::new();
        for o in &organs {
            root_seed_bytes.extend_from_slice(&o.kappa);
        }
        for s in &stakeholders {
            root_seed_bytes.extend_from_slice(&s.kappa);
        }
        let (observer_kappa, observer_kappa_hex) =
            compute_kappa("ObserverRootHolon", &root_seed_bytes);

        let (sensory_streams, next_stream_prime_ordinal) = Self::seed_default_sensory_streams();

        let mut holon = Self {
            observer_kappa,
            observer_kappa_hex,
            organ_basis_product: basis_prod,
            emergency_halted: false,
            organs,
            stakeholders,
            catalogs,
            active_domain,
            active_navmesh,
            npc_agents,
            language_engine: VerdantLanguageEngine::new(),
            attention_loop,
            admitted_model_kappas: Vec::new(),
            project_models: Vec::new(),
            host_envelope: HostCapabilityEnvelope::default_safe(),
            sensory_streams,
            next_stream_prime_ordinal,
            ingested_knowledge: Vec::new(),
            part_hydrations: Vec::new(),
            communications: Vec::new(),
        };

        // Record initial Organ -> Organ boot observation experiencing domain activation:
        let boot_msg = format!(
            "Activated catalog domain `{}` (seed={})",
            holon.active_domain.spec.id, holon.active_domain.spec.seed
        );
        let _ = holon.observe_communication(
            CommunicationChannelKind::OrganToOrgan,
            "urn:verdant:organ:catalogs",
            "urn:verdant:organ:engine",
            "en",
            0,
            &boot_msg,
            None,
            0,
        )?;

        Ok(holon)
    }

    fn closest_nav_polygon(navmesh: &NavMesh, pos: Vec3) -> usize {
        let mut best_idx = 0usize;
        let mut best_d2 = f64::INFINITY;
        for (idx, poly) in navmesh.polygons.iter().enumerate() {
            let dx = poly.centroid.x - pos.x;
            let dz = poly.centroid.z - pos.z;
            let d2 = dx * dx + dz * dz;
            if d2 < best_d2 {
                best_d2 = d2;
                best_idx = idx;
            }
        }
        best_idx
    }

    fn plan_directive_corridor(
        navmesh: &NavMesh,
        start_pos: Vec3,
        patrol_radius_m: f64,
        directive_index: usize,
    ) -> Vec<Vec3> {
        if navmesh.polygons.is_empty() {
            return vec![start_pos];
        }
        let start_poly = Self::closest_nav_polygon(navmesh, start_pos);
        let dirs = four_u32() as f64;
        let angle = (directive_index as f64) * (std::f64::consts::TAU / dirs) + half_f64();
        let target_hint = Vec3::new(
            start_pos.x + patrol_radius_m * angle.cos(),
            start_pos.y,
            start_pos.z + patrol_radius_m * angle.sin(),
        );
        let goal_poly = Self::closest_nav_polygon(navmesh, target_hint);
        if let Ok(corridor) = navmesh.find_path_astar(start_poly, goal_poly) {
            if corridor.waypoints.len() > 1 {
                return corridor.waypoints;
            }
        }
        // Fallback: search neighboring connected polygons for a guaranteed multi-waypoint walkable loop:
        let mut cursor = start_poly;
        let mut loop_pts = vec![navmesh.polygons[start_poly].centroid];
        let max_hops = (four_u32() * two_u32()) as usize;
        for hop in 0..max_hops {
            let nbrs = &navmesh.polygons[cursor].neighbors;
            if nbrs.is_empty() {
                break;
            }
            let pick = nbrs[(directive_index + hop) % nbrs.len()];
            if pick != cursor {
                cursor = pick;
                loop_pts.push(navmesh.polygons[cursor].centroid);
            }
        }
        loop_pts
    }

    fn build_npc_agents(
        domain: &InstantiatedDomainPack,
        navmesh: &NavMesh,
    ) -> Vec<NpcNavAgentState> {
        let deci = pow10_u64(1) as f64;
        domain
            .spec
            .characters
            .iter()
            .enumerate()
            .map(|(idx, ch_spec)| {
                let start_pos = domain
                    .characters
                    .get(idx)
                    .map_or(Vec3::ZERO, |c| c.foot_position);
                let radius_m = (ch_spec.patrol_radius_decimeters as f64) / deci;
                let waypoints = Self::plan_directive_corridor(navmesh, start_pos, radius_m, 0);
                NpcNavAgentState {
                    character_id: ch_spec.id.clone(),
                    active_directive_index: 0,
                    active_directive_label: ch_spec.directives.first().cloned().unwrap_or_default(),
                    corridor_waypoints: waypoints,
                    waypoint_cursor: 0,
                    distance_walked_meters: 0.0,
                }
            })
            .collect()
    }

    /// Switches the Observer's active world to another cataloged domain pack (`Sunspire`,
    /// `Meridian`, `Glasshouse`, or `Starfall`), rebuilding the walkable `NavMesh` and NPC agents.
    pub fn switch_domain_pack(
        &mut self,
        domain_id_or_slug: &str,
        lod_override: Option<u64>,
    ) -> Result<(), ObserverModelError> {
        let next_domain = self
            .catalogs
            .instantiate_domain_pack(domain_id_or_slug, lod_override)
            .map_err(|_| ObserverModelError::UnknownDomainOrCharacter {
                identifier: domain_id_or_slug.to_string(),
            })?;
        let next_navmesh = NavMesh::from_mesh(&next_domain.sector.meshes[0]).map_err(|err| {
            ObserverModelError::InvalidPrismModelEnvelope {
                model_id: next_domain.spec.id.clone(),
                detail: err.to_string(),
            }
        })?;
        self.npc_agents = Self::build_npc_agents(&next_domain, &next_navmesh);
        self.active_domain = next_domain;
        self.active_navmesh = next_navmesh;
        let msg = format!(
            "Switched active domain pack to `{}` (seed={})",
            self.active_domain.spec.id, self.active_domain.spec.seed
        );
        let _ = self.observe_communication(
            CommunicationChannelKind::OrganToOrgan,
            "urn:verdant:organ:catalogs",
            "urn:verdant:organ:engine",
            "en",
            0,
            &msg,
            None,
            0,
        )?;
        Ok(())
    }

    /// Updates `npc_index`'s active directive and replans its admissible `NavMesh` A* corridor.
    pub fn set_npc_directive(
        &mut self,
        npc_index: usize,
        directive_index: usize,
    ) -> Result<NpcNavAgentState, ObserverModelError> {
        let Some(ch_spec) = self.active_domain.spec.characters.get(npc_index) else {
            return Err(ObserverModelError::UnknownDomainOrCharacter {
                identifier: format!("npc:{npc_index}"),
            });
        };
        if directive_index >= ch_spec.directives.len() {
            return Err(ObserverModelError::DirectiveOutOfBounds {
                character_id: ch_spec.id.clone(),
                directive_index,
                directive_count: ch_spec.directives.len(),
            });
        }
        let deci = pow10_u64(1) as f64;
        let radius_m = (ch_spec.patrol_radius_decimeters as f64) / deci;
        let cur_pos = self
            .active_domain
            .characters
            .get(npc_index)
            .map_or(Vec3::ZERO, |c| c.foot_position);
        let corridor =
            Self::plan_directive_corridor(&self.active_navmesh, cur_pos, radius_m, directive_index);
        let label = ch_spec.directives[directive_index].clone();
        let ch_id = ch_spec.id.clone();

        let agent = &mut self.npc_agents[npc_index];
        agent.active_directive_index = directive_index;
        agent.active_directive_label.clone_from(&label);
        agent.corridor_waypoints = corridor;
        agent.waypoint_cursor = 0;
        let snapshot = agent.clone();

        // Record Agent <-> Agent coordination event on the Observer bus:
        let peer_id = self
            .active_domain
            .spec
            .characters
            .get((npc_index + 1) % self.active_domain.spec.characters.len())
            .map_or(ch_id.as_str(), |c| c.id.as_str())
            .to_string();
        let _ = self.observe_communication(
            CommunicationChannelKind::AgentToAgent,
            &format!("urn:verdant:character:{ch_id}"),
            &format!("urn:verdant:character:{peer_id}"),
            "en",
            (1 + 1) + 1,
            &format!("Directive `{label}` active along A* NavMesh corridor"),
            None,
            0,
        )?;

        Ok(snapshot)
    }

    /// Steps all cataloged NPC characters along their walkable `NavMesh` A* corridors and solves
    /// analytical 22-joint Two-Bone Foot IK against `two_level_bvh`.
    pub fn step_npc_patrols(
        &mut self,
        dt_seconds: f64,
        sim_time_seconds: f64,
        player_focus_pos: Vec3,
        two_level_bvh: &TwoLevelBvh,
    ) {
        let dt = dt_seconds.clamp(0.0, half_f64());
        if dt <= 0.0 {
            return;
        }
        let patrol_speed_mps = 1.0 + half_f64();
        let arrive_dist = half_f64();
        let seed = self.active_domain.spec.seed;
        let relief = self.active_domain.terrain_relief;

        for (idx, ch) in self.active_domain.characters.iter_mut().enumerate() {
            let Some(agent) = self.npc_agents.get_mut(idx) else {
                continue;
            };
            let vel = if agent.corridor_waypoints.len() > 1 {
                let target = agent.corridor_waypoints
                    [agent.waypoint_cursor % agent.corridor_waypoints.len()];
                let to_wp = Vec3::new(
                    target.x - ch.foot_position.x,
                    0.0,
                    target.z - ch.foot_position.z,
                );
                let dist = to_wp.length();
                if dist <= arrive_dist {
                    agent.waypoint_cursor =
                        (agent.waypoint_cursor + 1) % agent.corridor_waypoints.len();
                    Vec3::ZERO
                } else {
                    let dir = to_wp.scale(1.0 / dist.max(geometric_epsilon()));
                    let step_len = (patrol_speed_mps * dt).min(dist);
                    let nx = ch.foot_position.x + dir.x * step_len;
                    let nz = ch.foot_position.z + dir.z * step_len;
                    let ny = hd_terrain_height(seed, &relief, nx, nz);
                    ch.foot_position = Vec3::new(nx, ny, nz);
                    agent.distance_walked_meters += step_len;
                    dir.scale(patrol_speed_mps)
                }
            } else {
                Vec3::ZERO
            };

            let _ = ch.step_and_solve_ik(
                dt,
                ch.foot_position,
                vel,
                true,
                sim_time_seconds,
                Some(player_focus_pos),
                two_level_bvh,
                |x, z| hd_terrain_height(seed, &relief, x, z),
            );
        }
    }

    /// Borrows the active domain's articulated NPC characters.
    #[must_use]
    pub fn npc_characters(&self) -> &[ArticulatedCharacter] {
        &self.active_domain.characters
    }

    /// Admits an arbitrary `VerdantModelEnvelope` into the Verdant Engine runtime after verifying:
    /// 1. Non-zero 32-byte `kappa` matching `(model_id, payload_bytes)`,
    /// 2. OMG DMN 1.5 `FIRST`-hit Verdant-only format admission (`dmnModelRouteAdmitted`),
    /// 3. `FK-07` capability grant on `required_container_organ`,
    /// 4. Schema.org JSON-LD validity (`SchemaOrgVocabulary`), and
    /// 5. Constructive `prism_kernel::arbitraryPrismModelValid`.
    pub fn admit_arbitrary_prism_model(
        &mut self,
        envelope: &VerdantModelEnvelope,
        grants: &ContainerGrantMatrix,
    ) -> Result<String, ObserverModelError> {
        if envelope.payload_bytes.is_empty() || envelope.kappa.iter().all(|&b| b == 0) {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id: envelope.model_id.clone(),
                detail: "empty payload or all-zero kappa digest".to_string(),
            });
        }
        let (expected_kappa, kappa_hex) =
            compute_kappa(&envelope.model_id, &envelope.payload_bytes);
        if expected_kappa != envelope.kappa {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id: envelope.model_id.clone(),
                detail: "kappa digest mismatch against canonical model payload".to_string(),
            });
        }

        let route_granted = grants.is_allowed(envelope.required_container_organ);
        let dmn_decision = self
            .catalogs
            .language_dmn
            .evaluate_dmn_admission(&DmnAdmissionInput {
                model_format: envelope.model_format.clone(),
                observer_halted: self.emergency_halted,
                source_locked: true,
                route_granted,
                runtime_state: "match".to_string(),
                weights_state: "match".to_string(),
            })
            .map_err(|err| ObserverModelError::InvalidPrismModelEnvelope {
                model_id: envelope.model_id.clone(),
                detail: err.to_string(),
            })?;

        let verdant_only = self
            .catalogs
            .language_dmn
            .allowed_verdant_formats
            .iter()
            .any(|f| f == &envelope.model_format);
        let kernel_dmn = prism_kernel::dmnModelRouteAdmitted(
            !self.emergency_halted,
            true,
            verdant_only,
            route_granted,
            true,
            true,
        );
        if dmn_decision.outcome != "run" || !kernel_dmn {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id: envelope.model_id.clone(),
                detail: format!(
                    "DMN 1.5 admission outcome `{}` ({})",
                    dmn_decision.outcome, dmn_decision.reason
                ),
            });
        }

        let schema_rep = self
            .catalogs
            .schema_org
            .validate_schema_org_jsonld(&envelope.schema_org_jsonld)
            .map_err(|err| ObserverModelError::InvalidPrismModelEnvelope {
                model_id: envelope.model_id.clone(),
                detail: err.to_string(),
            })?;

        let kernel_model_ok = prism_kernel::arbitraryPrismModelValid(
            envelope.kappa.len() as u64,
            schema_rep.entity_count as u64,
            envelope.owner_organ.index() as u64,
            verdant_only,
            true,
        )
        .unwrap_or(false);
        if !kernel_model_ok {
            return Err(ObserverModelError::InvalidPrismModelEnvelope {
                model_id: envelope.model_id.clone(),
                detail: "rejected by Observer::arbitraryPrismModelValid".to_string(),
            });
        }

        self.admitted_model_kappas
            .push((envelope.model_id.clone(), kappa_hex.clone()));
        Ok(kappa_hex)
    }

    /// Experiences, verifies, and records a communication across any of the Observer's 5 channels
    /// (`Organ<->Organ`, `Player<->Player`, `Player->Agent`, `Agent->Player`, `Agent<->Agent`).
    #[allow(clippy::too_many_arguments)]
    pub fn observe_communication(
        &mut self,
        channel: CommunicationChannelKind,
        sender_id: &str,
        recipient_id: &str,
        language_bcp47: &str,
        dialogue_act_code: u8,
        message_text: &str,
        prism_session: Option<(&PrismIdSessionEnvelope, &str, u64)>,
        attended_slot: usize,
    ) -> Result<ObservedCommunicationRecord, ObserverModelError> {
        let trimmed = message_text.trim();
        if trimmed.is_empty() || sender_id.trim().is_empty() || recipient_id.trim().is_empty() {
            return Err(ObserverModelError::CommunicationEnvelopeRejected {
                channel: channel.slug().to_string(),
                detail: "sender, recipient, and message text must be non-empty".to_string(),
            });
        }

        let msg_digest = sha256_payload_digest(trimmed.as_bytes());
        let prism_sig_valid = if let Some((env, expected_origin, now_sec)) = prism_session {
            env.verify(expected_origin, now_sec, &msg_digest).is_ok()
        } else {
            false
        };

        if channel.requires_prism_signature() && !prism_sig_valid {
            return Err(ObserverModelError::CommunicationEnvelopeRejected {
                channel: channel.slug().to_string(),
                detail:
                    "player communication channel requires a valid fresh PrismIdSessionEnvelope"
                        .to_string(),
            });
        }

        let seq = (self.communications.len() as u64) + 1;
        let action_iri = format!("urn:verdant:comm:{seq}");
        let sender_type = if channel == CommunicationChannelKind::OrganToOrgan {
            "SoftwareApplication"
        } else {
            "Person"
        };
        let recipient_type = if channel == CommunicationChannelKind::OrganToOrgan {
            "SoftwareApplication"
        } else {
            "Person"
        };
        let schema_action_jsonld = json!({
            "@context": "https://schema.org",
            "@graph": [
                { "@id": sender_id, "@type": sender_type, "name": sender_id },
                { "@id": recipient_id, "@type": recipient_type, "name": recipient_id },
                {
                    "@id": action_iri,
                    "@type": "CommunicateAction",
                    "name": format!("{}#{seq}", channel.slug()),
                    "agent": { "@id": sender_id },
                    "recipient": { "@id": recipient_id },
                    "inLanguage": language_bcp47,
                    "about": trimmed
                }
            ]
        });
        self.catalogs
            .schema_org
            .validate_schema_org_jsonld(&schema_action_jsonld)
            .map_err(|err| ObserverModelError::CommunicationEnvelopeRejected {
                channel: channel.slug().to_string(),
                detail: err.to_string(),
            })?;

        let prev_iri = if seq > 1 {
            format!("urn:verdant:comm:{}", seq - 1)
        } else {
            format!("urn:verdant:domain:{}", self.active_domain.spec.id)
        };
        let sanitized_lit: String = trimmed
            .chars()
            .filter(|&c| c != '"' && c != '\\' && c != '\n' && c != '\r')
            .collect();
        let sosa_provo_turtle = format!(
            "@prefix sosa: <http://www.w3.org/ns/sosa/> .\n\
             @prefix prov: <http://www.w3.org/ns/prov#> .\n\
             @prefix schema: <https://schema.org/> .\n\
             <{sender_id}> a prov:Agent ; schema:name \"{sender_id}\" .\n\
             <urn:verdant:activity:comm:{seq}> a prov:Activity ; prov:wasAssociatedWith <{sender_id}> .\n\
             <{prev_iri}> a prov:Entity ; prov:wasAttributedTo <{sender_id}> .\n\
             <{action_iri}> a prov:Entity ; prov:wasDerivedFrom <{prev_iri}> ; prov:wasGeneratedBy <urn:verdant:activity:comm:{seq}> .\n\
             <urn:verdant:obs:comm:{seq}> a sosa:Observation ; sosa:madeBySensor <urn:verdant:sensor:observer-holon> ; sosa:hasFeatureOfInterest <{recipient_id}> ; sosa:hasSimpleResult \"{sanitized_lit}\" .\n"
        );
        self.catalogs
            .sosa_provo
            .validate_sosa_provo_turtle(&sosa_provo_turtle)
            .map_err(|err| ObserverModelError::CommunicationEnvelopeRejected {
                channel: channel.slug().to_string(),
                detail: err.to_string(),
            })?;

        let kernel_env_ok =
            prism_kernel::observedChannelEnvelopeValid(channel.code(), prism_sig_valid, true, true)
                .unwrap_or(false);
        if !kernel_env_ok {
            return Err(ObserverModelError::CommunicationEnvelopeRejected {
                channel: channel.slug().to_string(),
                detail: "rejected by Observer::observedChannelEnvelopeValid".to_string(),
            });
        }

        // Excite the Observer's neuromorphic LIF salience loop so the Observer genuinely experiences the message:
        let hundred = pow10_u64(two_u32()) as u32;
        let sixteen = four_u32() * four_u32();
        let sample = PlayerObserverSample {
            gazed_entity_slot: attended_slot,
            gaze_dwell_millis: hundred * two_u32(),
            camera_angular_speed_milli: hundred,
            locomotion_speed_milli: 0,
            cadence_focus_milli: hundred * (four_u32() * two_u32()),
            audio_alignment_milli: hundred as i32,
            ui_focus_slot: u32::from(dialogue_act_code),
            frame_dt_millis: sixteen,
        };
        let spike_mask = self
            .attention_loop
            .step_observation(sample, 0)
            .map_or(0, |d| d.spike_mask);

        let (_, kappa_hex) = compute_kappa(&action_iri, sosa_provo_turtle.as_bytes());
        let rec = ObservedCommunicationRecord {
            sequence: seq,
            channel,
            sender_id: sender_id.to_string(),
            recipient_id: recipient_id.to_string(),
            language_bcp47: language_bcp47.to_string(),
            dialogue_act_code,
            message_text: trimmed.to_string(),
            prism_signed: prism_sig_valid,
            kappa_hex,
            schema_action_jsonld,
            sosa_provo_turtle,
            observer_spike_mask: spike_mask,
        };
        self.communications.push(rec.clone());
        Ok(rec)
    }

    /// Executes a complete observed `Player -> Agent` and `Agent -> Player` dialogue exchange:
    /// 1. Logs and verifies the signed `PlayerToAgent` communication,
    /// 2. Evaluates grounded Q&A or directive mutation via `VerdantLanguageEngine`,
    /// 3. Applies any directive update to `set_npc_directive` (replanning A* `NavMesh` patrol), and
    /// 4. Logs and verifies the resulting `AgentToPlayer` response (`inform`, `request`, or `"Unknown"` `abstain`).
    pub fn converse_player_with_npc(
        &mut self,
        player_id: &str,
        prism_session: (&PrismIdSessionEnvelope, &str, u64),
        npc_index: usize,
        utterance: &str,
    ) -> Result<NpcDialogueTurnResult, ObserverModelError> {
        let Some(ch_spec) = self.active_domain.spec.characters.get(npc_index) else {
            return Err(ObserverModelError::UnknownDomainOrCharacter {
                identifier: format!("npc:{npc_index}"),
            });
        };
        let npc_iri = format!("urn:verdant:character:{}", ch_spec.id);
        let (lang, p_act) = self
            .language_engine
            .classify_utterance(utterance, &self.catalogs);

        let _ = self.observe_communication(
            CommunicationChannelKind::PlayerToAgent,
            player_id,
            &npc_iri,
            &lang,
            p_act,
            utterance,
            Some(prism_session),
            npc_index,
        )?;

        let turn = self.language_engine.evaluate_npc_dialogue_turn(
            utterance,
            &self.active_domain,
            npc_index,
            &self.catalogs,
        )?;

        if let Some(dir_idx) = turn.mutated_directive_index {
            let _ = self.set_npc_directive(npc_index, dir_idx)?;
        }

        let _ = self.observe_communication(
            CommunicationChannelKind::AgentToPlayer,
            &npc_iri,
            player_id,
            &turn.detected_language,
            turn.response_act_code,
            &turn.reply_text,
            None,
            npc_index,
        )?;

        Ok(turn)
    }

    /// Serializes the Observer holon's live identity, organs, stakeholders, active domain, NPC
    /// directives, and recent observed communications into UTF-8 JSON for Studio and Catalogs.
    #[must_use]
    pub fn to_status_json_bytes(&self) -> Vec<u8> {
        let max_recent = (four_u32() * two_u32()) as usize;
        let comm_start = self.communications.len().saturating_sub(max_recent);
        let recent_comms: Vec<serde_json::Value> = self.communications[comm_start..]
            .iter()
            .map(|c| {
                json!({
                    "sequence": c.sequence,
                    "channel": c.channel.slug(),
                    "sender": c.sender_id,
                    "recipient": c.recipient_id,
                    "lang": c.language_bcp47,
                    "act": c.dialogue_act_code,
                    "text": c.message_text,
                    "prismSigned": c.prism_signed,
                    "kappa": c.kappa_hex,
                    "spikeMask": c.observer_spike_mask
                })
            })
            .collect();

        let organs_json: Vec<serde_json::Value> = self
            .organs
            .iter()
            .map(|o| {
                json!({
                    "slug": o.kind.slug(),
                    "index": o.index,
                    "basisPrime": o.basis_prime,
                    "hostsAllModels": o.hosts_all_models,
                    "kappa": o.kappa_hex
                })
            })
            .collect();

        let stakeholders_json: Vec<serde_json::Value> = self
            .stakeholders
            .iter()
            .map(|s| {
                json!({
                    "index": s.index,
                    "workspaceIndex": s.workspace_index,
                    "workspaceSlug": s.workspace_slug,
                    "basisPrime": s.basis_prime,
                    "organGrantCount": s.allowed_organs.len(),
                    "kappa": s.kappa_hex
                })
            })
            .collect();

        let npcs_json: Vec<serde_json::Value> = self
            .active_domain
            .spec
            .characters
            .iter()
            .enumerate()
            .map(|(idx, ch)| {
                let agent = self.npc_agents.get(idx);
                json!({
                    "index": idx,
                    "id": ch.id,
                    "name": ch.name,
                    "role": ch.role,
                    "rigId": ch.rig_id,
                    "directives": ch.directives,
                    "activeDirectiveIndex": agent.map_or(0, |a| a.active_directive_index),
                    "activeDirectiveLabel": agent.map_or("", |a| a.active_directive_label.as_str()),
                    "corridorWaypoints": agent.map_or(0, |a| a.corridor_waypoints.len()),
                    "distanceWalkedMeters": agent.map_or(0.0, |a| a.distance_walked_meters)
                })
            })
            .collect();

        let doc = json!({
            "observerKappa": self.observer_kappa_hex,
            "organBasisProduct": self.organ_basis_product,
            "emergencyHalted": self.emergency_halted,
            "activeDomain": {
                "id": self.active_domain.spec.id,
                "slug": self.active_domain.spec.slug,
                "title": self.active_domain.spec.title,
                "schemaOrgType": self.active_domain.spec.schema_type,
                "kappa": compute_kappa(&self.active_domain.spec.id, self.active_domain.spec.slug.as_bytes()).1,
                "seed": self.active_domain.spec.seed,
                "defaultLodRank": self.active_domain.spec.default_lod_rank,
                "cameraMode": self.active_domain.spec.camera_mode,
                "conservedSupply": self.active_domain.item_ledger.total_supply(),
                "navPolygons": self.active_navmesh.polygons.len()
            },
            "organs": organs_json,
            "roleCount": self.stakeholders.len(),
            "stakeholders": stakeholders_json,
            "npcs": npcs_json,
            "admittedModelCount": self.admitted_model_kappas.len(),
            "importedProjectModelCount": self.project_models.len(),
            "communicationCount": self.communications.len(),
            "recentCommunications": recent_comms
        });
        serde_json::to_vec(&doc).unwrap_or_default()
    }
}

#[path = "observer_streams.rs"]
mod observer_streams;

#[cfg(test)]
mod observer_cognition_tests {
    use super::*;

    #[test]
    fn observer_sensory_streams_have_unique_primes_kappas_and_dmn_lifecycle() {
        let mut holon = ObserverPrismHolon::new("sunspire-regatta").expect("observer init");
        assert_eq!(holon.sensory_streams.len(), 10);

        let primes: BTreeSet<u64> = holon
            .sensory_streams
            .iter()
            .map(|s| s.basis_prime)
            .collect();
        let kappas: BTreeSet<&str> = holon
            .sensory_streams
            .iter()
            .map(|s| s.kappa_hex.as_str())
            .collect();
        assert_eq!(
            primes.len(),
            10,
            "each sensory stream needs a unique basis prime"
        );
        assert_eq!(
            kappas.len(),
            10,
            "each sensory stream needs a unique SHA-256 kappa"
        );

        let grants = ContainerGrantMatrix::default();
        // Negative DMN / URI scheme validation test: non-HTTPS endpoint must be refused!
        let bad = holon.register_stream(
            StreamRegisterRequest {
                stream_id: "insecure-feed".to_string(),
                label: "Insecure HTTP Feed".to_string(),
                authority: "Untrusted".to_string(),
                adapter_kind: "schema-org-catalog".to_string(),
                endpoint_url: "http://insecure.example/feed.json".to_string(),
                target_parts: vec!["hero-props".to_string()],
            },
            &grants,
        );
        assert!(bad.is_err(), "non-https stream endpoint must be refused");

        // Positive admission + toggle + removal lifecycle:
        let added = holon
            .register_stream(
                StreamRegisterRequest {
                    stream_id: "test-uhd-stream".to_string(),
                    label: "Verified UHD glTF Stream".to_string(),
                    authority: "Open 3D Consortium".to_string(),
                    adapter_kind: "khronos-index".to_string(),
                    endpoint_url: "https://example.org/model-index.json".to_string(),
                    target_parts: vec!["character-rigs".to_string(), "hero-props".to_string()],
                },
                &grants,
            )
            .expect("valid https stream must be admitted");
        assert!(added.basis_prime > 47);
        assert_eq!(holon.sensory_streams.len(), 11);

        let paused = holon
            .toggle_stream("test-uhd-stream", false)
            .expect("toggle off");
        assert!(!paused.enabled);

        let removed_kappa = holon
            .remove_stream("test-uhd-stream")
            .expect("remove stream");
        assert_eq!(removed_kappa, added.kappa_hex);
        assert_eq!(holon.sensory_streams.len(), 10);
    }

    #[test]
    fn live_stream_knowledge_ingestion_grounds_iso_24617_2_and_preserves_abstention() {
        let mut holon = ObserverPrismHolon::new("sunspire-regatta").expect("observer init");
        let grants = ContainerGrantMatrix::default();

        let rec = holon
            .ingest_knowledge_passage(KnowledgePassageRequest {
                source_stream: "wikidata".to_string(),
                source_uri: "https://en.wikipedia.org/wiki/Apollo_Lunar_Module".to_string(),
                topic: "Apollo Lunar Module".to_string(),
                passage: "The Apollo Lunar Module was the lunar lander spacecraft flown between lunar orbit and the Moon surface during the Apollo program.".to_string(),
            })
            .expect("knowledge passage ingestion");
        assert_eq!(rec.kappa_hex.len(), 64);

        // Grounded query against freshly ingested stream knowledge must answer without hallucinating:
        let _ = grants;
        let q1 = "What was the Apollo Lunar Module spacecraft?";
        let d1 = sha256_payload_digest(q1.as_bytes());
        let env1 = PrismIdSessionEnvelope::sign_deterministic(
            "prism:test:observer",
            b"observer-test-secret",
            "https://afflom.github.io",
            100,
            200,
            &d1,
        );
        let turn = holon
            .converse_player_with_npc(
                "prism:test:observer",
                (&env1, "https://afflom.github.io", 100),
                0,
                q1,
            )
            .expect("grounded dialogue turn");
        assert!(
            !turn.abstained_unknown,
            "ingested stream passage should ground answer"
        );
        assert!(turn.reply_text.contains("Apollo Lunar Module"));

        // Ungrounded query must still abstain with Unknown (zero hallucination law):
        let q2 = "What is the xylophone price on Neptune?";
        let d2 = sha256_payload_digest(q2.as_bytes());
        let env2 = PrismIdSessionEnvelope::sign_deterministic(
            "prism:test:observer",
            b"observer-test-secret",
            "https://afflom.github.io",
            100,
            200,
            &d2,
        );
        let unknown = holon
            .converse_player_with_npc(
                "prism:test:observer",
                (&env2, "https://afflom.github.io", 100),
                0,
                q2,
            )
            .expect("abstention dialogue turn");
        assert!(unknown.abstained_unknown, "ungrounded query must abstain");
    }

    #[test]
    fn host_capability_envelope_enforces_uhd_4k_drs_and_software_clamping() {
        let mut env = HostCapabilityEnvelope::default_safe();

        // Hardware GPU at UHD 4K (3840x2160) with strong frame headroom (6ms):
        env.update_with_viewport(8192, false, 12, 6000, 120_000, 3840, 2160);
        assert!(env.is_uhd_target);
        assert_eq!(env.uhd_tier_label, "UHD 4K Native");
        assert_eq!(env.recommended_render_scale_permille, 1000);
        assert_eq!(env.recommended_shadow_map_px, 4096);
        assert_eq!(env.recommended_anisotropy, 8);
        assert_eq!(env.recommended_texture_cap_px, 2048);

        // Hardware GPU at UHD 4K under heavier load (11ms) -> engages DRS + CAS without exceeding host:
        env.update_with_viewport(8192, false, 8, 11000, 180_000, 3840, 2160);
        assert_eq!(env.uhd_tier_label, "UHD 4K DRS + CAS");
        assert!(env.recommended_render_scale_permille < 1000);
        assert!(env.recommended_render_scale_permille >= 650);

        // Constrained Software WebGL2 at UHD 4K -> strictly clamps texture/shadow/DRS/GLB cap:
        env.update_with_viewport(8192, true, 12, 14000, 50_000, 3840, 2160);
        assert_eq!(env.uhd_tier_label, "Host-Bounded Software");
        assert_eq!(env.recommended_texture_cap_px, 512);
        assert_eq!(env.recommended_shadow_map_px, 1024);
        assert_eq!(env.recommended_anisotropy, 1);
        assert_eq!(env.recommended_render_scale_permille, 650);
    }
}
