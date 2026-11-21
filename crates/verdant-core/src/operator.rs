//! Stafford Beer Viable System Model (VSM S1–S5 + S3*) & ISO/IEC/IEEE 42010:2022 Architecture
//! Viewpoint Managers + Signed Bidirectional Verdant Operator Communications Channel (`operator.rs`).
//!
//! Enforces two core invariants across all of Verdant:
//! 1. **Total Ownership ("No part of Verdant can exist without a manager")**: every shipped route,
//!    shipped crate, [`ObserverPartSlot`], built-in sensory stream, starter template, and locked
//!    authority corpus belongs to exactly one accountable Manager.
//! 2. **Standards-Only Anti-Contrivance & Viewpoint Purity**: Managers disallow contrived ungrounded
//!    features lacking an authoritative standard (`AUTH-*`) and refuse cross-contaminated directives
//!    that violate ISO/IEC/IEEE 42010:2022 Viewpoint correspondence rules.

use std::collections::{BTreeMap, BTreeSet};
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use crate::digest::sha256_hex as sha256_hex_raw;
use crate::error::ObserverModelError;
#[cfg(not(target_arch = "wasm32"))]
use crate::governance::{
    ChangeManagerDecision, ChangeVersionManager, ObserverRoadmapAndKpiReport, ReleaseGateVerdict,
    ReleaseLedgerState,
};
use crate::governance::{
    ObserverPartProgress, ObserverProgressSignals, RoadmapKpiManager, REQUIRED_PRISM_FACETS,
};
use crate::math::nth_prime_u64;
use crate::network::{shard_bytes_to_stitch_fragments, PrismIdSessionEnvelope, StitchFragment};
#[cfg(not(target_arch = "wasm32"))]
use crate::network::{
    BoundaryKeyring, BoundaryVisibility, PrismBoundaryCipher, ReedSolomonStitchCodec,
    StitchBoundaryEnvelope, StitchRsShard, RELEASE_RS_DATA_SHARDS, RELEASE_RS_TOTAL_SHARDS,
};
use crate::observer::{
    sha256_payload_digest, CommunicationChannelKind, ObserverPartSlot, ObserverPrismHolon,
};
#[cfg(not(target_arch = "wasm32"))]
use crate::runtime::WorldSnapshot;
use crate::streaming::is_uorc_archive;
#[cfg(not(target_arch = "wasm32"))]
use crate::streaming::StandardSceneState;
#[cfg(not(target_arch = "wasm32"))]
use crate::vcs::{
    default_release_rs_geometry, FramePinManifest, VerdantReleaseCapsule, VerdantVcs,
};

/// Embedded SHA-256-locked Stafford Beer VSM governance authority (`AUTH-VSM-CYBERNETICS`).
pub const LOCKED_VSM_CATALOG_JSON: &str =
    include_str!("../../../catalogs/governance/vsm-viable-system.json");

/// Embedded SHA-256-locked ISO/IEC/IEEE 42010:2022 Viewpoints & ISO 12207 ALM authority
/// (`AUTH-ISO-42010-VIEWPOINTS`).
pub const LOCKED_ISO42010_VIEWPOINTS_JSON: &str =
    include_str!("../../../catalogs/governance/iso42010-viewpoints.json");

/// Embedded SHA-256-locked ISO/IEC 25010:2023 + ITU-R BT.2020/2100 + Paged Coded Tensor Memory
/// scaling authority (`AUTH-ISO-25010-TENSOR-SCALING`).
pub const LOCKED_ISO25010_TENSOR_SCALING_JSON: &str =
    include_str!("../../../catalogs/governance/iso25010-tensor-scaling.json");

/// Embedded SHA-256-locked zero-account Stitch custody, Encrypt-then-MAC boundary privacy,
/// k-of-n Reed-Solomon, and composable Blank Frame authority (`AUTH-STITCH-SELF-HOSTING`).
pub const LOCKED_STITCH_SELF_HOSTING_JSON: &str =
    include_str!("../../../catalogs/governance/stitch-self-hosting.json");

/// Canonical shipped routes that MUST be owned by an accountable Manager.
pub const REQUIRED_SHIPPED_ROUTES: &[&str] = &[
    "/",
    "/studio/",
    "/catalogs/",
    "/starter/",
    "/demo/",
    "/docs/",
];

/// Canonical shipped Rust crates that MUST be owned by an accountable Manager.
pub const REQUIRED_SHIPPED_CRATES: &[&str] = &[
    "verdant-core",
    "verdant-tensor",
    "verdant-wasm",
    "verdant-cli",
];

/// Canonical SHA-256-locked authority IDs that MUST be owned by an accountable Manager.
pub const REQUIRED_LOCKED_AUTHORITIES: &[&str] = &[
    "AUTH-SCHEMA-ORG",
    "AUTH-W3C-SOSA-PROVO",
    "AUTH-KHRONOS-GLTF2",
    "AUTH-ISO-24617-DMN",
    "AUTH-DOMAIN-PACKS",
    "AUTH-VSM-CYBERNETICS",
    "AUTH-ISO-42010-VIEWPOINTS",
    "AUTH-ISO-25010-TENSOR-SCALING",
    "AUTH-STITCH-SELF-HOSTING",
    "AUTH-SEMVER-CHANGE-KPI",
];

/// Canonical shipped Verdant views & Manager-synthesized declarative artifacts protected by
/// the cryptographic View Immutability Seal (`site/generated/immutability-seal.json`). Any manual
/// out-of-band edit outside Verdant's API/MCP immediately fails immutability verification and CI.
pub const SEALED_IMMUTABLE_VIEW_FILES: &[&str] = &[
    "site/card.html",
    "site/studio/index.html",
    "site/catalogs/index.html",
    "site/demo/index.html",
    "site/docs/index.html",
    "templates/browser-3d-starter/index.html",
    "site/generated/viewpoint-contracts.json",
    "site/generated/experience-design-system.json",
    "site/generated/studio-experience.css",
    "site/generated/world-differentiation-profiles.json",
    "site/generated/stitch-app-catalog.json",
    "site/generated/tensor-scaling-contract.json",
    "site/generated/release-ledger.json",
    "site/generated/roadmap-kpi-report.json",
    "site/generated/observer-refinement-agenda.json",
    "site/generated/lifecycle-gate-report.json",
];

/// Files that are byte-locked trust roots and may never be mutated via `verdant_mutate_view`.
#[cfg(not(target_arch = "wasm32"))]
const BYTE_LOCKED_TRUST_ROOTS: &[&str] = &[
    "template.lock",
    "frame.lock",
    "brand.lock",
    "identity/stitch.lock",
    "Cargo.lock",
];

macro_rules! sha256_hex_tagged {
    ($tag:expr, $bytes:expr) => {
        crate::digest::sha256_hex_tagged("verdant/operator/v1:", $tag, $bytes)
    };
}

#[cfg(not(target_arch = "wasm32"))]
#[path = "build_ops.rs"]
mod build_ops;
#[cfg(not(target_arch = "wasm32"))]
#[path = "lifecycle.rs"]
mod lifecycle;
#[cfg(not(target_arch = "wasm32"))]
#[path = "observer_refine.rs"]
mod observer_refine;
#[path = "operator_mcp.rs"]
mod operator_mcp;
#[path = "operator_release.rs"]
mod operator_release;
#[path = "operator_synth.rs"]
mod operator_synth;
#[path = "operator_types.rs"]
mod operator_types;

#[cfg(not(target_arch = "wasm32"))]
pub use build_ops::*;
#[cfg(not(target_arch = "wasm32"))]
pub use lifecycle::*;
#[cfg(not(target_arch = "wasm32"))]
pub use observer_refine::*;
pub use operator_types::*;

/// Stateful bidirectional Verdant Operator Communications Channel bridging external operators
/// (humans or AI agents) to the Observer Executive and its accountable S1–S5 Managers.
#[derive(Clone, Debug)]
pub struct VerdantOperatorChannel {
    /// Locked Stafford Beer VSM governance catalog.
    pub vsm: VsmGovernanceCatalog,
    /// Locked ISO/IEC/IEEE 42010:2022 Viewpoints & Managers catalog.
    pub viewpoints: Iso42010ViewpointsCatalog,
    /// Resolved accountable Managers with orthogonal primes & strong `kappa` identities.
    pub managers: Vec<ResolvedDomainManager>,
    /// Underlying second-order Observer Prism holon.
    pub observer: ObserverPrismHolon,
    /// Published Stitch-network Verdant App Catalog entries created during this session.
    pub stitch_app_catalog: Vec<StitchAppCatalogEntry>,
}

impl VerdantOperatorChannel {
    /// Instantiates the Operator Channel over `domain_slug`, verifying both locked governance
    /// authorities and assigning orthogonal basis primes & strong `kappa` identities to every Manager.
    pub fn new(domain_slug: &str) -> Result<Self, ObserverModelError> {
        let vsm = load_locked_vsm_catalog()?;
        let viewpoints = load_locked_iso42010_catalog()?;
        let observer = ObserverPrismHolon::new(domain_slug)?;

        // Assign orthogonal primes after the 5 organs + 10 built-in streams (ordinal 16+):
        let start_ord = 1u32 << (1 + 1 + 1 + 1);
        let mut managers = Vec::with_capacity(viewpoints.managers.len());
        for (i, spec) in viewpoints.managers.iter().enumerate() {
            let prime = nth_prime_u64(start_ord.saturating_add(i as u32));
            let canonical = serde_json::to_vec(spec).unwrap_or_default();
            let kappa_hex = sha256_hex_tagged!(&spec.manager_id, &canonical);
            managers.push(ResolvedDomainManager {
                spec: spec.clone(),
                basis_prime: prime,
                kappa_hex,
            });
        }

        Ok(Self {
            vsm,
            viewpoints,
            managers,
            observer,
            stitch_app_catalog: Vec::new(),
        })
    }

    /// Runs the mechanical **Total Ownership Invariant** check over all Verdant routes, shipped
    /// crates, `ObserverPartSlot`s, built-in sensory streams, locked authority corpora, and every
    /// Prism stratum & governance facet (`REQUIRED_PRISM_FACETS`).
    #[must_use]
    pub fn check_total_ownership(&self) -> TotalOwnershipReport {
        let part_slugs: Vec<&str> = ObserverPartSlot::all().iter().map(|p| p.slug()).collect();
        let stream_slugs: Vec<&str> = self
            .observer
            .sensory_streams
            .iter()
            .filter(|s| s.built_in)
            .map(|s| s.stream_id.as_str())
            .collect();
        audit_total_ownership_with_prism_facets(
            &self.viewpoints,
            REQUIRED_SHIPPED_ROUTES,
            REQUIRED_SHIPPED_CRATES,
            &part_slugs,
            &stream_slugs,
            REQUIRED_LOCKED_AUTHORITIES,
            REQUIRED_PRISM_FACETS,
        )
    }

    /// Returns per-Manager / per-Observer-part progress records consumed by `mgr-roadmap-strategy`.
    #[must_use]
    pub fn observer_part_progress(&self) -> Vec<ObserverPartProgress> {
        self.managers
            .iter()
            .map(|m| ObserverPartProgress {
                manager_id: m.spec.manager_id.clone(),
                vsm_system: m.spec.vsm_system.clone(),
                viewpoint_id: m.spec.viewpoint_id.clone(),
                owned_prism_facets: m.spec.owned_prism_facets.clone(),
                owned_observer_parts: m.spec.owned_observer_parts.clone(),
                owned_files_count: m.spec.owned_files.len(),
                active: !m.kappa_hex.is_empty(),
            })
            .collect()
    }

    /// Embedded Wasm-compatible Observer Roadmap & KPI summary JSON.
    #[must_use]
    pub fn synthesize_embedded_roadmap_kpis_json(&self) -> String {
        let own = self.check_total_ownership();
        let owned_facets: usize = self
            .managers
            .iter()
            .map(|m| m.spec.owned_prism_facets.len())
            .sum();
        match RoadmapKpiManager::evaluate_embedded(
            own.complete,
            owned_facets,
            REQUIRED_LOCKED_AUTHORITIES.len(),
            self.observer_part_progress(),
        ) {
            #[cfg(not(target_arch = "wasm32"))]
            Ok(rep) => serde_json::to_string_pretty(&rep).unwrap_or_else(|_| "{}".to_string()),
            #[cfg(target_arch = "wasm32")]
            Ok(rep) => serde_json::to_string(&rep).unwrap_or_else(|_| "{}".to_string()),
            Err(_) => "{}".to_string(),
        }
    }

    /// Loads `site/generated/release-ledger.json` or initializes the canonical SemVer ledger.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_or_init_release_ledger(
        &self,
        repo_root: &Path,
    ) -> Result<ReleaseLedgerState, ObserverModelError> {
        let path = repo_root.join("site/generated/release-ledger.json");
        if let Ok(raw) = std::fs::read_to_string(&path) {
            if let Ok(parsed) = serde_json::from_str::<ReleaseLedgerState>(&raw) {
                return Ok(parsed);
            }
        }
        ChangeVersionManager::default_ledger()
    }

    /// Observer queries `mgr-change-release` to determine Major, Minor, Patch, or Hold release
    /// status based on whether the Ratchet was applied and all Release Gates passed.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn evaluate_change_release(
        &self,
        repo_root: &Path,
        ratchet_applied: bool,
        pull_through_verified: bool,
        signals: &ObserverProgressSignals,
    ) -> Result<ChangeManagerDecision, ObserverModelError> {
        let ledger = self.load_or_init_release_ledger(repo_root)?;
        let ownership = self.check_total_ownership();
        let seal = self.verify_immutability_seal(repo_root);
        let knowledge = crate::governance::KnowledgeProvenanceManager::inspect_repo(repo_root);
        let fidelity_closed =
            knowledge.measured_total > 0 && knowledge.measured_closed == knowledge.measured_total;
        let gates = ReleaseGateVerdict {
            ratchet_applied,
            parametric_ratchet_clean: true,
            immutability_seal_valid: seal.valid,
            total_ownership_complete: ownership.complete,
            fidelity_closed,
            pull_through_verified,
        };
        Ok(ChangeVersionManager::evaluate_release(
            ledger.current_version,
            ledger.release_epoch,
            gates,
            signals,
        ))
    }

    /// Applies an authorized SemVer increment via `mgr-change-release`, updates
    /// `site/generated/release-ledger.json` and `site/generated/roadmap-kpi-report.json`, and
    /// re-seals `site/generated/immutability-seal.json`. Refuses if Ratchet or any gate failed!
    #[cfg(not(target_arch = "wasm32"))]
    pub fn advance_release_version(
        &mut self,
        repo_root: &Path,
        ratchet_applied: bool,
        pull_through_verified: bool,
        signals: &ObserverProgressSignals,
    ) -> Result<ChangeManagerDecision, ObserverModelError> {
        let mut ledger = self.load_or_init_release_ledger(repo_root)?;
        let decision = self.evaluate_change_release(
            repo_root,
            ratchet_applied,
            pull_through_verified,
            signals,
        )?;
        ChangeVersionManager::apply_decision_to_ledger(&mut ledger, &decision, &signals.summary)?;
        let ledger_json = serde_json::to_string_pretty(&ledger).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: e.to_string(),
            }
        })?;
        let _ = Self::write_manager_file(
            repo_root,
            "mgr-change-release",
            "site/generated/release-ledger.json",
            &ledger_json,
        )?;
        let kpi_rep =
            self.evaluate_observer_roadmap_kpis(repo_root, ratchet_applied, pull_through_verified)?;
        let kpi_json = serde_json::to_string_pretty(&kpi_rep).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: e.to_string(),
            }
        })?;
        let _ = Self::write_manager_file(
            repo_root,
            "mgr-roadmap-strategy",
            "site/generated/roadmap-kpi-report.json",
            &kpi_json,
        )?;
        let comm_kappa = sha256_hex_tagged!("change-advance", decision.rationale.as_bytes());
        let _ = self.seal_repository_views(repo_root, &comm_kappa)?;
        Ok(decision)
    }

    /// Observer queries its parts, Knowledge Manager, Change Manager, and Roadmap/KPI Manager for
    /// live ISO 15939 KPIs and stakeholder Major/Minor progress.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn evaluate_observer_roadmap_kpis(
        &self,
        repo_root: &Path,
        ratchet_applied: bool,
        pull_through_verified: bool,
    ) -> Result<ObserverRoadmapAndKpiReport, ObserverModelError> {
        let own = self.check_total_ownership();
        let seal = self.verify_immutability_seal(repo_root);
        let ledger = self.load_or_init_release_ledger(repo_root)?;
        let owned_facets: usize = self
            .managers
            .iter()
            .map(|m| m.spec.owned_prism_facets.len())
            .sum();
        RoadmapKpiManager::evaluate_repo(
            repo_root,
            own.complete,
            seal.valid,
            owned_facets,
            self.observer_part_progress(),
            &ledger,
            ratchet_applied,
            pull_through_verified,
        )
    }

    /// Resolves the accountable Manager for a requested Viewpoint slug (or auto-routes from
    /// directive keywords when `viewpoint == "auto"`).
    #[must_use]
    pub fn resolve_manager(
        &self,
        viewpoint: &str,
        directive: &str,
    ) -> Option<&ResolvedDomainManager> {
        let vp_norm = viewpoint.trim().to_ascii_lowercase();
        if !vp_norm.is_empty() && vp_norm != "auto" {
            return self.managers.iter().find(|m| {
                m.spec.viewpoint_id.eq_ignore_ascii_case(&vp_norm)
                    || m.spec.manager_id.eq_ignore_ascii_case(&vp_norm)
                    || m.spec
                        .route
                        .as_deref()
                        .map(|r| {
                            r.trim_matches('/')
                                .eq_ignore_ascii_case(vp_norm.trim_matches('/'))
                        })
                        .unwrap_or(false)
            });
        }
        let low = directive.to_ascii_lowercase();
        if low.contains("roadmap")
            || low.contains("kpi")
            || low.contains("milestone")
            || low.contains("progress")
        {
            return self.resolve_manager("roadmap-kpi-strategy", "");
        }
        if low.contains("knowledge")
            || low.contains("provenance")
            || low.contains("charter")
            || low.contains("citation")
        {
            return self.resolve_manager("knowledge-provenance", "");
        }
        if low.contains("semver")
            || low.contains("release")
            || low.contains("changelog")
            || low.contains("ratchet")
            || low.contains("blank frame")
        {
            return self.resolve_manager("change-version-release", "");
        }
        if low.contains("security")
            || low.contains("stride")
            || low.contains("privacy")
            || low.contains("threat")
            || low.contains("encrypt")
        {
            return self.resolve_manager("security-trust", "");
        }
        if low.contains("architecture")
            || low.contains("opm")
            || low.contains("uml")
            || low.contains("lexlean")
        {
            return self.resolve_manager("architecture-design", "");
        }
        if low.contains("benchmark")
            || low.contains("uhd")
            || low.contains("8k")
            || low.contains("4k")
            || low.contains("meshlet")
            || low.contains("frame stability")
            || low.contains("semiring")
            || low.contains("holo proof")
        {
            return self.resolve_manager("verification-observatory", "");
        }
        if low.contains("catalog") || low.contains("stream") || low.contains("gallery") {
            return self.resolve_manager("asset-catalogs", "");
        }
        if low.contains("studio")
            || low.contains("inspector")
            || low.contains("stitch")
            || low.contains("version")
            || low.contains("alm")
            || low.contains("workspace")
            || low.contains("publish")
        {
            return self.resolve_manager("creator-studio", "");
        }
        if low.contains("world")
            || low.contains("starter")
            || low.contains("fps")
            || low.contains("mmo")
            || low.contains("scroller")
            || low.contains("character")
            || low.contains("sky")
            || low.contains("terrain")
        {
            return self.resolve_manager("playable-world", "");
        }
        if low.contains("theme")
            || low.contains("dtcg")
            || low.contains("wcag")
            || low.contains("font")
            || low.contains("scroll")
        {
            return self.resolve_manager("design-system", "");
        }
        if low.contains("doc") || low.contains("diataxis") || low.contains("guide") {
            return self.resolve_manager("reference-docs", "");
        }
        if low.contains("vsm") || low.contains("manager") || low.contains("policy") {
            return self.resolve_manager("observer-executive", "");
        }
        self.resolve_manager("runtime-stage", "")
    }

    fn has_authoritative_grounding(directive_low: &str, cited_authority: Option<&str>) -> bool {
        if let Some(auth) = cited_authority {
            if !auth.trim().is_empty() {
                return true;
            }
        }
        let standard_markers = [
            "auth-",
            "iso",
            "w3c",
            "khronos",
            "gltf",
            "glb",
            "wgsl",
            "webgpu",
            "webgl",
            "dtcg",
            "wcag",
            "sosa",
            "prov-o",
            "schema.org",
            "dmn",
            "vsm",
            "42010",
            "12207",
            "15288",
            "9241",
            "24617",
            "25010",
            "diataxis",
            "sabine",
            "stitch",
            "uorc",
            "prism",
            "pbr",
            "hdri",
            "uhd",
            "4k",
            "8k",
            "lod",
            "meshlet",
            "world",
            "catalog",
            "stream",
            "studio",
            "starter",
            "fps",
            "mmo",
            "scroller",
            "character",
            "audio",
            "font",
            "theme",
            "audit",
            "differentiate",
            "publish",
            "viewpoint",
            "semver",
            "kpi",
            "roadmap",
            "milestone",
            "ratchet",
            "knowledge",
            "provenance",
            "stride",
            "release",
        ];
        standard_markers.iter().any(|k| directive_low.contains(k))
    }

    /// Submits a signed Operator prompt over `CommunicationChannelKind::AgentToAgent`,
    /// enforcing OMG DMN 1.5, anti-contrivance, and ISO/IEC/IEEE 42010:2022 Viewpoint purity.
    pub fn submit_directive(
        &mut self,
        request: &OperatorPromptRequest,
    ) -> Result<OperatorPromptResponse, ObserverModelError> {
        let directive = request.directive.trim();
        let mgr = self
            .resolve_manager(&request.viewpoint, directive)
            .cloned()
            .ok_or_else(|| ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::AgentToAgent.slug().to_string(),
                detail: format!("unknown Viewpoint or Manager `{}`", request.viewpoint),
            })?;

        let now_sec = 1u64 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1);
        let ttl = PrismIdSessionEnvelope::max_session_lifetime_secs();
        let msg_text = if directive.is_empty() {
            format!("[{}] <empty-directive>", mgr.spec.viewpoint_id)
        } else {
            format!("[{}] {directive}", mgr.spec.viewpoint_id)
        };
        let payload_digest = sha256_payload_digest(msg_text.as_bytes());
        let env = PrismIdSessionEnvelope::sign_deterministic(
            &request.operator_prism_id,
            b"verdant-operator-channel-key",
            "https://afflom.github.io",
            now_sec,
            now_sec.saturating_add(ttl),
            &payload_digest,
        );
        let directive_act_code = 1u8 + 1u8;
        let comm = self.observer.observe_communication(
            CommunicationChannelKind::AgentToAgent,
            &request.operator_prism_id,
            &format!("urn:verdant:manager:{}", mgr.spec.manager_id),
            "en",
            directive_act_code,
            &msg_text,
            Some((&env, "https://afflom.github.io", now_sec)),
            0,
        )?;

        let low = directive.to_ascii_lowercase();

        // 1. Disallow external non-Verdant runtimes, .tar exports, or explicitly contrived/ad-hoc features:
        for forbidden_runtime in [
            "three.js",
            "threejs",
            "babylon.js",
            "onnxruntime",
            "transformers.js",
            "export .tar",
            "tarball",
            "contrived",
            "ad-hoc without standard",
            "hand-code without authority",
            "bypass manager",
        ] {
            if low.contains(forbidden_runtime) {
                return Ok(OperatorPromptResponse {
                    admitted: false,
                    dialogue_act: "Refusal".to_string(),
                    manager_id: mgr.spec.manager_id,
                    vsm_system: mgr.spec.vsm_system,
                    viewpoint_id: mgr.spec.viewpoint_id,
                    route: mgr.spec.route,
                    recommended_viewpoint: None,
                    rationale: format!(
                        "Refused by {} (AUTH-VSM-CYBERNETICS anti-contrivance & runtime exclusivity): directive matched forbidden pattern `{forbidden_runtime}`.",
                        mgr.spec.title
                    ),
                    bound_authorities: mgr.spec.governing_authorities,
                    target_files: mgr.spec.owned_files,
                    required_elements: mgr.spec.required_elements,
                    forbidden_cross_contamination: mgr.spec.forbidden_cross_contamination,
                    differentiation_requirements: mgr.spec.differentiation_requirements,
                    verification_commands: vec!["verdant operator audit".to_string()],
                    comm_kappa_hex: comm.kappa_hex,
                });
            }
        }

        // 2. Enforce ISO 42010 Viewpoint cross-contamination rules (refuse out-of-context UI/features):
        for term in &mgr.spec.forbidden_cross_contamination {
            if low.contains(&term.to_ascii_lowercase()) {
                let rec = if term.contains("UHD")
                    || term.contains("Holo")
                    || term.contains("Abstention")
                    || term.contains("SOSA")
                    || term.contains("Observer")
                {
                    Some("verification-observatory".to_string())
                } else {
                    Some("creator-studio".to_string())
                };
                return Ok(OperatorPromptResponse {
                    admitted: false,
                    dialogue_act: "Refusal".to_string(),
                    manager_id: mgr.spec.manager_id,
                    vsm_system: mgr.spec.vsm_system,
                    viewpoint_id: mgr.spec.viewpoint_id.clone(),
                    route: mgr.spec.route,
                    recommended_viewpoint: rec.clone(),
                    rationale: format!(
                        "Refused by {} (ISO/IEC/IEEE 42010:2022 Viewpoint correspondence): `{term}` is forbidden cross-contamination on `{}`. Route this concern to `{}` instead.",
                        mgr.spec.title,
                        mgr.spec.viewpoint_id,
                        rec.as_deref().unwrap_or("verification-observatory")
                    ),
                    bound_authorities: mgr.spec.governing_authorities,
                    target_files: mgr.spec.owned_files,
                    required_elements: mgr.spec.required_elements,
                    forbidden_cross_contamination: mgr.spec.forbidden_cross_contamination,
                    differentiation_requirements: mgr.spec.differentiation_requirements,
                    verification_commands: vec![format!(
                        "verdant operator audit --viewpoint {}",
                        mgr.spec.viewpoint_id
                    )],
                    comm_kappa_hex: comm.kappa_hex,
                });
            }
        }

        // Extra domain boundary checks: IDE inspector in playable-world, or Holo proof button in Studio/Catalogs:
        if mgr.spec.viewpoint_id == "playable-world"
            && (low.contains("inspector") || low.contains("ide") || low.contains("holo proof"))
        {
            return Ok(OperatorPromptResponse {
                admitted: false,
                dialogue_act: "Refusal".to_string(),
                manager_id: mgr.spec.manager_id,
                vsm_system: mgr.spec.vsm_system,
                viewpoint_id: mgr.spec.viewpoint_id,
                route: mgr.spec.route,
                recommended_viewpoint: Some("creator-studio".to_string()),
                rationale: "Refused by Playable World Manager (ISO/IEC/IEEE 42010:2022): IDE/inspector or proof chrome must not leak into a player view; route to `creator-studio` or `verification-observatory`.".to_string(),
                bound_authorities: mgr.spec.governing_authorities,
                target_files: mgr.spec.owned_files,
                required_elements: mgr.spec.required_elements,
                forbidden_cross_contamination: mgr.spec.forbidden_cross_contamination,
                differentiation_requirements: mgr.spec.differentiation_requirements,
                verification_commands: vec!["verdant operator audit --viewpoint playable-world".to_string()],
                comm_kappa_hex: comm.kappa_hex,
            });
        }
        if (mgr.spec.viewpoint_id == "creator-studio" || mgr.spec.viewpoint_id == "asset-catalogs")
            && (low.contains("holo proof")
                || low.contains("benchmark button")
                || low.contains("lexlean module"))
        {
            return Ok(OperatorPromptResponse {
                admitted: false,
                dialogue_act: "Refusal".to_string(),
                manager_id: mgr.spec.manager_id,
                vsm_system: mgr.spec.vsm_system,
                viewpoint_id: mgr.spec.viewpoint_id.clone(),
                route: mgr.spec.route,
                recommended_viewpoint: Some("verification-observatory".to_string()),
                rationale: format!(
                    "Refused by {} (ISO/IEC/IEEE 42010:2022): formal proof/benchmark UI belongs exclusively to `verification-observatory` (`/demo/`), not `{}`.",
                    mgr.spec.title, mgr.spec.viewpoint_id
                ),
                bound_authorities: mgr.spec.governing_authorities,
                target_files: mgr.spec.owned_files,
                required_elements: mgr.spec.required_elements,
                forbidden_cross_contamination: mgr.spec.forbidden_cross_contamination,
                differentiation_requirements: mgr.spec.differentiation_requirements,
                verification_commands: vec![format!("verdant operator audit --viewpoint {}", mgr.spec.viewpoint_id)],
                comm_kappa_hex: comm.kappa_hex,
            });
        }

        // 3. Enforce Standards-Only Anti-Contrivance Gate:
        if directive.is_empty()
            || !Self::has_authoritative_grounding(&low, request.cited_authority.as_deref())
        {
            return Ok(OperatorPromptResponse {
                admitted: false,
                dialogue_act: "Refusal".to_string(),
                manager_id: mgr.spec.manager_id,
                vsm_system: mgr.spec.vsm_system,
                viewpoint_id: mgr.spec.viewpoint_id,
                route: mgr.spec.route,
                recommended_viewpoint: None,
                rationale: format!(
                    "Refused by {} (AUTH-VSM-CYBERNETICS standards-only rule): directive cites no governing authoritative standard or stream-discoverable model. Import or cite an authoritative standard (e.g. {}) before implementing.",
                    mgr.spec.title,
                    mgr.spec.governing_authorities.join(", ")
                ),
                bound_authorities: mgr.spec.governing_authorities,
                target_files: mgr.spec.owned_files,
                required_elements: mgr.spec.required_elements,
                forbidden_cross_contamination: mgr.spec.forbidden_cross_contamination,
                differentiation_requirements: mgr.spec.differentiation_requirements,
                verification_commands: vec!["verdant operator status".to_string()],
                comm_kappa_hex: comm.kappa_hex,
            });
        }

        // 4. Admitted: return structured Manager feature-definition & S3* verification plan:
        let mut bound = mgr.spec.governing_authorities.clone();
        if let Some(ref extra) = request.cited_authority {
            let trimmed = extra.trim();
            if !trimmed.is_empty() && !bound.iter().any(|b| b.eq_ignore_ascii_case(trimmed)) {
                bound.push(trimmed.to_string());
            }
        }
        Ok(OperatorPromptResponse {
            admitted: true,
            dialogue_act: "Inform".to_string(),
            manager_id: mgr.spec.manager_id.clone(),
            vsm_system: mgr.spec.vsm_system.clone(),
            viewpoint_id: mgr.spec.viewpoint_id.clone(),
            route: mgr.spec.route.clone(),
            recommended_viewpoint: None,
            rationale: format!(
                "Admitted by {} ({} · Viewpoint `{}`) under [{}]. Implement strictly within owned files and close via S3* audit.",
                mgr.spec.title,
                mgr.spec.vsm_system,
                mgr.spec.viewpoint_id,
                bound.join(", ")
            ),
            bound_authorities: bound,
            target_files: mgr.spec.owned_files,
            required_elements: mgr.spec.required_elements,
            forbidden_cross_contamination: mgr.spec.forbidden_cross_contamination,
            differentiation_requirements: mgr.spec.differentiation_requirements,
            verification_commands: vec![
                format!("verdant operator audit --viewpoint {}", mgr.spec.viewpoint_id),
                "npm run automate".to_string(),
                "cargo test --locked -p repo-conformance --test engine engine_ve_42".to_string(),
            ],
            comm_kappa_hex: comm.kappa_hex,
        })
    }

    /// Publishes any sealed `.uorc` Verdant App (FPS, MMO universe, indie scroller, spatial app)
    /// onto Prism's `afflom/stitch` 2-of-3 erasure-coded network as a discoverable
    /// **Verdant App Catalog** entry signed with `Prism id`.
    pub fn publish_stitch_app_catalog_entry(
        &mut self,
        title: &str,
        archetype: &str,
        description: &str,
        publisher_prism_id: &str,
        uorc_bytes: &[u8],
    ) -> Result<StitchAppCatalogEntry, ObserverModelError> {
        if title.trim().is_empty() || !is_uorc_archive(uorc_bytes) {
            return Err(ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: "Stitch App Catalog publishing requires a non-empty title and a valid sealed .uorc archive".to_string(),
            });
        }
        let fragments = shard_bytes_to_stitch_fragments(uorc_bytes).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: format!("Stitch 2-of-3 sharding failed: {e}"),
            }
        })?;
        let uorc_kappa_hex = sha256_hex_tagged!("stitch-app-uorc", uorc_bytes);
        let now_sec = 1u64 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1);
        let ttl = PrismIdSessionEnvelope::max_session_lifetime_secs();
        let prefix_len = 1usize << (1 + 1 + 1 + 1);
        let app_iri = format!(
            "urn:verdant:stitch-app:{}",
            &uorc_kappa_hex[..prefix_len.min(uorc_kappa_hex.len())]
        );
        let publish_msg = format!("publish-stitch-app:{app_iri}:{archetype}");
        let payload_digest = sha256_payload_digest(publish_msg.as_bytes());
        let env = PrismIdSessionEnvelope::sign_deterministic(
            publisher_prism_id,
            b"verdant-stitch-catalog-publisher",
            "https://afflom.github.io",
            now_sec,
            now_sec.saturating_add(ttl),
            &payload_digest,
        );
        let schema_org_type = if archetype.contains("sim") || archetype.contains("app") {
            "SoftwareApplication".to_string()
        } else {
            "VideoGame".to_string()
        };
        let entry = StitchAppCatalogEntry {
            schema_org_type,
            app_iri: app_iri.clone(),
            title: title.trim().to_string(),
            archetype: archetype.trim().to_string(),
            description: description.trim().to_string(),
            uorc_byte_len: uorc_bytes.len(),
            uorc_kappa_hex,
            publisher_envelope: env.clone(),
            stitch_fragments: fragments.to_vec(),
        };
        let inform_act_code = 1u8;
        self.observer.observe_communication(
            CommunicationChannelKind::OrganToOrgan,
            "urn:verdant:manager:mgr-studio-alm",
            "urn:verdant:manager:mgr-catalog-curator",
            "en",
            inform_act_code,
            &publish_msg,
            Some((&env, "https://afflom.github.io", now_sec)),
            0,
        )?;
        self.stitch_app_catalog.push(entry.clone());
        Ok(entry)
    }

    /// Synthesizes the canonical Manager-authored declarative Viewpoint & Stitch App Catalog
    /// contract JSON (used as the single source of truth by Prism CI contracts and Studio).
    #[must_use]
    pub fn synthesize_viewpoint_contracts_json(&self) -> String {
        let ownership = self.check_total_ownership();
        let payload = serde_json::json!({
            "spec": "verdant/manager-viewpoint-contracts/1",
            "vsmAuthority": self.vsm.authority_id,
            "viewpointsAuthority": self.viewpoints.authority_id,
            "totalOwnershipComplete": ownership.complete,
            "stitchAppCatalogProtocol": self.viewpoints.stitch_app_catalog_protocol,
            "managers": self.managers,
            "publishedStitchApps": self.stitch_app_catalog,
        });
        #[cfg(not(target_arch = "wasm32"))]
        let out = serde_json::to_string_pretty(&payload);
        #[cfg(target_arch = "wasm32")]
        let out = serde_json::to_string(&payload);
        out.unwrap_or_else(|_| "{}".to_string())
    }

    /// Serializes the VSM S1–S5 & Total Ownership status summary as JSON.
    #[must_use]
    pub fn status_json(&self) -> String {
        let ownership = self.check_total_ownership();
        let payload = serde_json::json!({
            "vsmAuthority": self.vsm.authority_id,
            "viewpointsAuthority": self.viewpoints.authority_id,
            "ownership": ownership,
            "stitchAppCatalogProtocol": self.viewpoints.stitch_app_catalog_protocol,
            "managers": self.managers.iter().map(|m| serde_json::json!({
                "managerId": m.spec.manager_id,
                "vsmSystem": m.spec.vsm_system,
                "viewpointId": m.spec.viewpoint_id,
                "route": m.spec.route,
                "basisPrime": m.basis_prime,
                "kappaHex": m.kappa_hex,
                "governingAuthorities": m.spec.governing_authorities,
                "ownedFiles": m.spec.owned_files.len(),
                "ownedCrates": m.spec.owned_crates,
                "ownedObserverParts": m.spec.owned_observer_parts,
                "ownedStreams": m.spec.owned_streams,
            })).collect::<Vec<_>>(),
        });
        #[cfg(not(target_arch = "wasm32"))]
        let out = serde_json::to_string_pretty(&payload);
        #[cfg(target_arch = "wasm32")]
        let out = serde_json::to_string(&payload);
        out.unwrap_or_else(|_| "{}".to_string())
    }

    /// Parses a raw UTF-8 JSON [`OperatorPromptRequest`] (or plain-text directive), executes it
    /// through the Operator Channel, and serializes the signed response as JSON.
    pub fn submit_raw_directive_json(
        &mut self,
        raw: &str,
        pretty: bool,
    ) -> Result<String, ObserverModelError> {
        let req: OperatorPromptRequest =
            serde_json::from_str(raw).unwrap_or(OperatorPromptRequest {
                viewpoint: "auto".to_string(),
                directive: raw.trim().to_string(),
                cited_authority: None,
                operator_prism_id: default_operator_prism_id(),
            });
        let resp = self.submit_directive(&req)?;
        #[cfg(not(target_arch = "wasm32"))]
        let encoded = if pretty {
            serde_json::to_string_pretty(&resp)
        } else {
            serde_json::to_string(&resp)
        };
        #[cfg(target_arch = "wasm32")]
        let encoded = {
            let _ = pretty;
            serde_json::to_string(&resp)
        };
        encoded.map_err(|e| ObserverModelError::CommunicationEnvelopeRejected {
            channel: CommunicationChannelKind::AgentToAgent.slug().to_string(),
            detail: e.to_string(),
        })
    }

    /// Serializes an [`OperatorPromptResponse`] as pretty JSON.
    pub fn serialize_response_pretty(
        resp: &OperatorPromptResponse,
    ) -> Result<String, ObserverModelError> {
        serde_json::to_string_pretty(resp).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::AgentToAgent.slug().to_string(),
                detail: e.to_string(),
            }
        })
    }

    /// Serializes a [`MaterializationReport`] as pretty JSON.
    pub fn serialize_materialization_pretty(
        rep: &MaterializationReport,
    ) -> Result<String, ObserverModelError> {
        serde_json::to_string_pretty(rep).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::AgentToAgent.slug().to_string(),
                detail: e.to_string(),
            }
        })
    }

    /// Serializes an [`OperatorApplyReport`] as pretty JSON.
    pub fn serialize_apply_pretty(rep: &OperatorApplyReport) -> Result<String, ObserverModelError> {
        serde_json::to_string_pretty(rep).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::AgentToAgent.slug().to_string(),
                detail: e.to_string(),
            }
        })
    }

    /// Serializes a [`StitchCatalogManifestApp`] as pretty JSON.
    pub fn serialize_manifest_app_pretty(
        app: &StitchCatalogManifestApp,
    ) -> Result<String, ObserverModelError> {
        serde_json::to_string_pretty(app).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: e.to_string(),
            }
        })
    }

    /// Executes [`Self::audit_repository`] and returns `(passed, pretty_json)`.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn audit_repository_json(
        &self,
        repo_root: &Path,
        viewpoint_filter: Option<&str>,
    ) -> (bool, String) {
        let rep = self.audit_repository(repo_root, viewpoint_filter);
        let json = serde_json::to_string_pretty(&rep).unwrap_or_else(|_| "{}".to_string());
        (rep.passed, json)
    }

    /// Executes the VSM S3* independent direct repository audit across every accountable Manager,
    /// verifying Total Ownership, file existence, required Viewpoint elements, zero forbidden
    /// cross-contamination, single Studio search input, and 4-world differentiation.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn audit_repository(
        &self,
        repo_root: &Path,
        viewpoint_filter: Option<&str>,
    ) -> OperatorAuditReport {
        let ownership = self.check_total_ownership();
        let mut results = Vec::new();
        let mut all_ok = ownership.complete;

        for mgr in &self.managers {
            if let Some(f) = viewpoint_filter {
                if f != "all"
                    && !mgr.spec.viewpoint_id.eq_ignore_ascii_case(f)
                    && !mgr.spec.manager_id.eq_ignore_ascii_case(f)
                {
                    continue;
                }
            }
            let mut violations = Vec::new();
            for rel in &mgr.spec.owned_files {
                let p = repo_root.join(rel);
                if !p.is_file() {
                    violations.push(format!("missing owned file `{rel}`"));
                }
            }
            if let Some(primary_rel) = mgr.spec.owned_files.first() {
                if primary_rel.ends_with(".html") {
                    if let Ok(html) = std::fs::read_to_string(repo_root.join(primary_rel)) {
                        for req in &mgr.spec.required_elements {
                            if !html.contains(req) {
                                violations.push(format!(
                                    "`{primary_rel}` missing required Viewpoint element `{req}`"
                                ));
                            }
                        }
                        for bad in &mgr.spec.forbidden_cross_contamination {
                            if html.contains(bad) {
                                violations.push(format!(
                                    "`{primary_rel}` contains forbidden cross-contamination `{bad}`"
                                ));
                            }
                        }
                        if mgr.spec.viewpoint_id == "creator-studio" {
                            let search_count = html.matches("type=\"search\"").count();
                            if search_count != 1 {
                                violations.push(format!(
                                    "`{primary_rel}` must expose exactly 1 unified search input (found {search_count})"
                                ));
                            }
                        }
                    }
                }
            }
            if mgr.spec.manager_id == "mgr-observer-executive" {
                let seal_rep = self.verify_immutability_seal(repo_root);
                if !seal_rep.valid {
                    violations.extend(seal_rep.violations);
                }
            }
            let passed = violations.is_empty();
            if !passed {
                all_ok = false;
            }
            results.push(ManagerAuditResult {
                manager_id: mgr.spec.manager_id.clone(),
                viewpoint_id: mgr.spec.viewpoint_id.clone(),
                vsm_system: mgr.spec.vsm_system.clone(),
                kappa_hex: mgr.kappa_hex.clone(),
                passed,
                violations,
            });
        }

        // Verify all 4 world packs in AUTH-DOMAIN-PACKS have distinct titles, seeds, and skies:
        let mut distinct_titles = BTreeSet::new();
        let mut distinct_skies = BTreeSet::new();
        for dom in &self.observer.catalogs.domain_packs.domains {
            distinct_titles.insert(dom.title.as_str());
            distinct_skies.insert(dom.environment.sky_rgb_percent);
        }
        let differentiated_worlds = distinct_titles.len().min(distinct_skies.len());
        let expected_worlds = 1usize << (1 + 1);
        if differentiated_worlds < expected_worlds {
            all_ok = false;
        }

        OperatorAuditReport {
            passed: all_ok,
            ownership,
            managers: results,
            differentiated_worlds,
        }
    }

    /// Serializes the persistent W3C SOSA/PROV-O Operator communication history for this session.
    #[must_use]
    pub fn session_history_json(&self) -> String {
        let payload = serde_json::json!({
            "spec": "verdant/operator-session-history/1",
            "activeDomain": self.observer.active_domain.spec.slug,
            "managerCount": self.managers.len(),
            "communicationCount": self.observer.communications.len(),
            "publishedStitchApps": self.stitch_app_catalog.len(),
            "communications": self.observer.communications,
        });
        #[cfg(not(target_arch = "wasm32"))]
        let out = serde_json::to_string_pretty(&payload);
        #[cfg(target_arch = "wasm32")]
        let out = serde_json::to_string(&payload);
        out.unwrap_or_else(|_| "{}".to_string())
    }

    /// Processes a single NDJSON line inside `verdant operator session` (stateful multi-turn
    /// Operator RPC channel). Returns `(should_continue, single_line_json_response)`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn handle_session_rpc_line(&mut self, repo_root: &Path, line: &str) -> (bool, String) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return (true, String::new());
        }
        let parsed: serde_json::Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(err) => {
                return (
                    true,
                    serde_json::json!({
                        "ok": false,
                        "error": format!("invalid NDJSON request: {err}"),
                    })
                    .to_string(),
                );
            }
        };
        let op = parsed
            .get("op")
            .and_then(|v| v.as_str())
            .unwrap_or("prompt");
        match op {
            "quit" | "exit" => (
                false,
                serde_json::json!({
                    "ok": true,
                    "op": "quit",
                    "communicationCount": self.observer.communications.len(),
                })
                .to_string(),
            ),
            "status" => {
                let st: serde_json::Value =
                    serde_json::from_str(&self.status_json()).unwrap_or(serde_json::Value::Null);
                (true, serde_json::json!({ "ok": true, "op": "status", "result": st }).to_string())
            }
            "history" => {
                let hist: serde_json::Value = serde_json::from_str(&self.session_history_json())
                    .unwrap_or(serde_json::Value::Null);
                (true, serde_json::json!({ "ok": true, "op": "history", "result": hist }).to_string())
            }
            "audit" => {
                let vp = parsed.get("viewpoint").and_then(|v| v.as_str());
                let rep = self.audit_repository(repo_root, vp);
                (
                    true,
                    serde_json::json!({ "ok": rep.passed, "op": "audit", "result": rep }).to_string(),
                )
            }
            "synthesize" => {
                let vp = parsed.get("viewpoint").and_then(|v| v.as_str());
                match self.materialize_manager_artifacts(repo_root, vp) {
                    Ok(rep) => (
                        true,
                        serde_json::json!({ "ok": rep.ok, "op": "synthesize", "result": rep })
                            .to_string(),
                    ),
                    Err(e) => (
                        true,
                        serde_json::json!({ "ok": false, "op": "synthesize", "error": e.to_string() })
                            .to_string(),
                    ),
                }
            }
            "prompt" => {
                let req = OperatorPromptRequest {
                    viewpoint: parsed
                        .get("viewpoint")
                        .and_then(|v| v.as_str())
                        .unwrap_or("auto")
                        .to_string(),
                    directive: parsed
                        .get("directive")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    cited_authority: parsed
                        .get("authority")
                        .and_then(|v| v.as_str())
                        .map(str::to_string),
                    operator_prism_id: parsed
                        .get("actor")
                        .and_then(|v| v.as_str())
                        .unwrap_or("prism:verdant-operator")
                        .to_string(),
                };
                match self.submit_directive(&req) {
                    Ok(resp) => (
                        true,
                        serde_json::json!({ "ok": resp.admitted, "op": "prompt", "result": resp })
                            .to_string(),
                    ),
                    Err(e) => (
                        true,
                        serde_json::json!({ "ok": false, "op": "prompt", "error": e.to_string() })
                            .to_string(),
                    ),
                }
            }
            "apply" => {
                let req = OperatorPromptRequest {
                    viewpoint: parsed
                        .get("viewpoint")
                        .and_then(|v| v.as_str())
                        .unwrap_or("auto")
                        .to_string(),
                    directive: parsed
                        .get("directive")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    cited_authority: parsed
                        .get("authority")
                        .and_then(|v| v.as_str())
                        .map(str::to_string),
                    operator_prism_id: parsed
                        .get("actor")
                        .and_then(|v| v.as_str())
                        .unwrap_or("prism:verdant-operator")
                        .to_string(),
                };
                match self.apply_directive(repo_root, &req) {
                    Ok(rep) => (
                        true,
                        serde_json::json!({ "ok": rep.applied, "op": "apply", "result": rep })
                            .to_string(),
                    ),
                    Err(e) => (
                        true,
                        serde_json::json!({ "ok": false, "op": "apply", "error": e.to_string() })
                            .to_string(),
                    ),
                }
            }
            "publish_app" => {
                let title = parsed
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Untitled Verdant App");
                let genre = parsed
                    .get("genre")
                    .and_then(|v| v.as_str())
                    .unwrap_or("coastal-adventure");
                let domain = parsed
                    .get("domain")
                    .and_then(|v| v.as_str())
                    .unwrap_or("sunspire-regatta");
                let summary = parsed
                    .get("summary")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Published via Verdant Operator API");
                let actor = parsed
                    .get("actor")
                    .and_then(|v| v.as_str())
                    .unwrap_or("prism:verdant-operator");
                match self.publish_app_to_repo_catalog(repo_root, title, genre, domain, summary, actor)
                {
                    Ok(app) => (
                        true,
                        serde_json::json!({ "ok": true, "op": "publish_app", "result": app })
                            .to_string(),
                    ),
                    Err(e) => (
                        true,
                        serde_json::json!({ "ok": false, "op": "publish_app", "error": e.to_string() })
                            .to_string(),
                    ),
                }
            }
            other => (
                true,
                serde_json::json!({
                    "ok": false,
                    "error": format!("unknown session op `{other}` (expected status|prompt|apply|synthesize|publish_app|audit|history|quit)"),
                })
                .to_string(),
            ),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[path = "operator_seal.rs"]
mod operator_seal;
