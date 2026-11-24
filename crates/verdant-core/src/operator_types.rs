//! Stafford Beer VSM, ISO/IEC/IEEE 42010:2022 Viewpoint, and Operator report/request types.

use super::*;

/// One recursive VSM system descriptor (`S1`, `S2`, `S3`, `S3*`, `S4`, `S5`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VsmSystemSpec {
    /// System identifier (`VSM-S1`, `VSM-S2`, `VSM-S3`, `VSM-S3-STAR`, `VSM-S4`, `VSM-S5`).
    pub id: String,
    /// Short code (`S1`, `S2`, `S3`, `S3*`, `S4`, `S5`).
    pub code: String,
    /// Descriptive name.
    pub name: String,
    /// Cybernetic role.
    pub role: String,
    /// Enforced viability invariants.
    pub invariants: Vec<String>,
}

/// Directed cybernetic channel linking two VSM systems.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VsmChannelSpec {
    /// Channel identifier.
    pub id: String,
    /// Origin VSM system ID.
    pub from: String,
    /// Destination VSM system ID.
    pub to: String,
    /// Governance purpose of the channel.
    pub purpose: String,
}

/// Declarative anti-contrivance policy enforced by all Managers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AntiContrivanceRule {
    /// Every feature must bind to an imported authoritative standard.
    pub require_authoritative_standard: bool,
    /// Every Verdant part must have an accountable Manager.
    pub require_manager_ownership: bool,
    /// Disallow contrived hand-coded ungrounded features.
    pub disallow_hand_coded_ungrounded_features: bool,
}

/// Deserialized Stafford Beer VSM governance authority (`AUTH-VSM-CYBERNETICS`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VsmGovernanceCatalog {
    /// Specification version string.
    pub spec: String,
    /// Authority identifier (`AUTH-VSM-CYBERNETICS`).
    pub authority_id: String,
    /// Cited standard title.
    pub standard: String,
    /// Authoritative citation.
    pub citation: String,
    /// License identifier.
    pub license: String,
    /// All six VSM systems (`S1`, `S2`, `S3`, `S3*`, `S4`, `S5`).
    pub systems: Vec<VsmSystemSpec>,
    /// Required cybernetic channels.
    pub channels: Vec<VsmChannelSpec>,
    /// Anti-contrivance & total-ownership rule.
    pub anti_contrivance_rule: AntiContrivanceRule,
}

/// Stitch-network Verdant App Catalog publishing protocol descriptor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StitchAppCatalogProtocol {
    /// Exclusive archive extension (`.uorc`).
    pub archive_format: String,
    /// Erasure coding scheme (`2-of-3-xor-sha256`).
    pub erasure_scheme: String,
    /// Identity signature scheme (`prism/identity/4`).
    pub identity_scheme: String,
    /// Allowed Schema.org application types.
    pub schema_org_types: Vec<String>,
    /// Supported arbitrary application archetypes (FPS, MMO, indie scroller, etc.).
    pub supported_app_archetypes: Vec<String>,
}

/// Formal ISO/IEC/IEEE 42010:2022 Viewpoint + VSM Manager ownership specification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iso42010ManagerSpec {
    /// Unique Manager identifier (`mgr-*`).
    pub manager_id: String,
    /// Governing VSM system (`VSM-S1`, `VSM-S2`, `VSM-S3-STAR`, `VSM-S5`).
    pub vsm_system: String,
    /// ISO 42010 Viewpoint slug.
    pub viewpoint_id: String,
    /// Human-readable Manager title.
    pub title: String,
    /// Owned web route if this Manager owns a stakeholder view (`/` .. `/docs/`).
    pub route: Option<String>,
    /// Domain stakeholder served by this Viewpoint.
    pub stakeholder_domain: String,
    /// ISO/IEC/IEEE 12207/15288 lifecycle phases owned.
    pub alm_phases: Vec<String>,
    /// Explicit stakeholder concerns addressed by this Viewpoint.
    pub concerns: Vec<String>,
    /// Authoritative `AUTH-*` standards governing this Manager.
    pub governing_authorities: Vec<String>,
    /// Repository files owned by this Manager.
    pub owned_files: Vec<String>,
    /// Shipped Rust crates owned by this Manager.
    pub owned_crates: Vec<String>,
    /// `ObserverPartSlot` slugs owned by this Manager.
    pub owned_observer_parts: Vec<String>,
    /// Built-in sensory stream slugs owned by this Manager.
    pub owned_streams: Vec<String>,
    /// Locked authority IDs stewarded by this Manager.
    pub owned_authorities: Vec<String>,
    /// Prism LexLean strata & governance facets stewarded by this Manager (`VE-45`).
    #[serde(default)]
    pub owned_prism_facets: Vec<String>,
    /// Required DOM/surface elements on this Viewpoint's primary page.
    pub required_elements: Vec<String>,
    /// Forbidden self-referential / out-of-viewpoint terms refused on this Viewpoint.
    pub forbidden_cross_contamination: Vec<String>,
    /// Visual & functional differentiation invariants for this Viewpoint.
    pub differentiation_requirements: Vec<String>,
}

/// Deserialized ISO/IEC/IEEE 42010:2022 Viewpoints & Manager ownership catalog.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iso42010ViewpointsCatalog {
    /// Specification version string.
    pub spec: String,
    /// Authority identifier (`AUTH-ISO-42010-VIEWPOINTS`).
    pub authority_id: String,
    /// Standard title.
    pub standard: String,
    /// Authoritative citation.
    pub citation: String,
    /// License identifier.
    pub license: String,
    /// Stitch-network Verdant App Catalog publishing descriptor.
    pub stitch_app_catalog_protocol: StitchAppCatalogProtocol,
    /// All accountable VSM / ISO 42010 Managers partitioning Verdant with zero orphans.
    pub managers: Vec<Iso42010ManagerSpec>,
}

/// Parses and validates the embedded `AUTH-VSM-CYBERNETICS` governance catalog.
pub fn load_locked_vsm_catalog() -> Result<VsmGovernanceCatalog, ObserverModelError> {
    let cat: VsmGovernanceCatalog = serde_json::from_str(LOCKED_VSM_CATALOG_JSON).map_err(|e| {
        ObserverModelError::CommunicationEnvelopeRejected {
            channel: "vsm-iso42010".to_string(),
            detail: format!("invalid AUTH-VSM-CYBERNETICS catalog: {e}"),
        }
    })?;
    let system_ids: BTreeSet<&str> = cat.systems.iter().map(|s| s.id.as_str()).collect();
    for req in [
        "VSM-S1",
        "VSM-S2",
        "VSM-S3",
        "VSM-S3-STAR",
        "VSM-S4",
        "VSM-S5",
    ] {
        if !system_ids.contains(req) {
            return Err(ObserverModelError::CommunicationEnvelopeRejected {
                channel: "vsm-iso42010".to_string(),
                detail: format!("VSM catalog missing required system `{req}`"),
            });
        }
    }
    if !cat.anti_contrivance_rule.require_authoritative_standard
        || !cat.anti_contrivance_rule.require_manager_ownership
        || !cat
            .anti_contrivance_rule
            .disallow_hand_coded_ungrounded_features
    {
        return Err(ObserverModelError::CommunicationEnvelopeRejected {
            channel: "vsm-iso42010".to_string(),
            detail: "VSM catalog must enable total ownership and standards-only anti-contrivance"
                .to_string(),
        });
    }
    Ok(cat)
}

/// Parses and validates the embedded `AUTH-ISO-42010-VIEWPOINTS` Manager catalog.
pub fn load_locked_iso42010_catalog() -> Result<Iso42010ViewpointsCatalog, ObserverModelError> {
    let cat: Iso42010ViewpointsCatalog = serde_json::from_str(LOCKED_ISO42010_VIEWPOINTS_JSON)
        .map_err(|e| ObserverModelError::CommunicationEnvelopeRejected {
            channel: "vsm-iso42010".to_string(),
            detail: format!("invalid AUTH-ISO-42010-VIEWPOINTS catalog: {e}"),
        })?;
    if cat.managers.is_empty() {
        return Err(ObserverModelError::CommunicationEnvelopeRejected {
            channel: "vsm-iso42010".to_string(),
            detail: "ISO 42010 catalog defines zero Managers".to_string(),
        });
    }
    if cat.stitch_app_catalog_protocol.archive_format != ".uorc" {
        return Err(ObserverModelError::CommunicationEnvelopeRejected {
            channel: "vsm-iso42010".to_string(),
            detail: "Stitch App Catalog protocol must enforce exclusive .uorc archives".to_string(),
        });
    }
    Ok(cat)
}

/// Mechanical report proving the **Total Ownership Invariant ("No part of Verdant can exist
/// without a manager")** across routes, shipped crates, Observer parts, streams, and authorities.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalOwnershipReport {
    /// True iff zero orphan parts and zero duplicate owners exist.
    pub complete: bool,
    /// Total accountable Managers audited.
    pub managers_checked: usize,
    /// Any unmanaged/orphan parts lacking an accountable Manager.
    pub orphans: Vec<String>,
    /// Any parts claimed by more than one Manager.
    pub duplicate_owners: Vec<String>,
}

/// Verifies Total Ownership across all required Verdant routes, crates, `ObserverPartSlot`s,
/// sensory streams, and locked authority corpora.
#[must_use]
pub fn audit_total_ownership(
    catalog: &Iso42010ViewpointsCatalog,
    required_routes: &[&str],
    required_crates: &[&str],
    required_observer_parts: &[&str],
    required_streams: &[&str],
    required_authorities: &[&str],
) -> TotalOwnershipReport {
    let mut route_owners: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut crate_owners: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut part_owners: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut stream_owners: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut auth_owners: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

    for m in &catalog.managers {
        if let Some(ref r) = m.route {
            route_owners
                .entry(r.as_str())
                .or_default()
                .push(&m.manager_id);
        }
        for c in &m.owned_crates {
            crate_owners
                .entry(c.as_str())
                .or_default()
                .push(&m.manager_id);
        }
        for p in &m.owned_observer_parts {
            part_owners
                .entry(p.as_str())
                .or_default()
                .push(&m.manager_id);
        }
        for s in &m.owned_streams {
            stream_owners
                .entry(s.as_str())
                .or_default()
                .push(&m.manager_id);
        }
        for a in &m.owned_authorities {
            auth_owners
                .entry(a.as_str())
                .or_default()
                .push(&m.manager_id);
        }
    }

    let mut orphans = Vec::new();
    let mut duplicate_owners = Vec::new();

    let mut check = |kind: &str, items: &[&str], map: &BTreeMap<&str, Vec<&str>>| {
        for &item in items {
            match map.get(item) {
                None => orphans.push(format!("{kind}:{item}")),
                Some(owners) if owners.is_empty() => orphans.push(format!("{kind}:{item}")),
                Some(owners) if owners.len() > 1 => duplicate_owners
                    .push(format!("{kind}:{item} claimed by [{}]", owners.join(", "))),
                Some(_) => {}
            }
        }
    };

    check("route", required_routes, &route_owners);
    check("crate", required_crates, &crate_owners);
    check("observer-part", required_observer_parts, &part_owners);
    check("stream", required_streams, &stream_owners);
    check("authority", required_authorities, &auth_owners);

    TotalOwnershipReport {
        complete: orphans.is_empty() && duplicate_owners.is_empty(),
        managers_checked: catalog.managers.len(),
        orphans,
        duplicate_owners,
    }
}

/// Verifies Total Ownership across all Verdant surfaces AND every Prism stratum / governance
/// facet (`REQUIRED_PRISM_FACETS`, `VE-45`).
#[must_use]
pub fn audit_total_ownership_with_prism_facets(
    catalog: &Iso42010ViewpointsCatalog,
    required_routes: &[&str],
    required_crates: &[&str],
    required_observer_parts: &[&str],
    required_streams: &[&str],
    required_authorities: &[&str],
    required_prism_facets: &[&str],
) -> TotalOwnershipReport {
    let mut rep = audit_total_ownership(
        catalog,
        required_routes,
        required_crates,
        required_observer_parts,
        required_streams,
        required_authorities,
    );
    let mut facet_owners: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for m in &catalog.managers {
        for f in &m.owned_prism_facets {
            facet_owners
                .entry(f.as_str())
                .or_default()
                .push(&m.manager_id);
        }
    }
    for &facet in required_prism_facets {
        match facet_owners.get(facet) {
            None => rep.orphans.push(format!("prism-facet:{facet}")),
            Some(owners) if owners.is_empty() => rep.orphans.push(format!("prism-facet:{facet}")),
            Some(owners) if owners.len() > 1 => rep.duplicate_owners.push(format!(
                "prism-facet:{facet} claimed by [{}]",
                owners.join(", ")
            )),
            Some(_) => {}
        }
    }
    rep.complete = rep.orphans.is_empty() && rep.duplicate_owners.is_empty();
    rep
}

/// Content-addressed live instance of one accountable VSM / ISO 42010 Manager.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedDomainManager {
    /// Underlying ISO 42010 / VSM Manager specification.
    pub spec: Iso42010ManagerSpec,
    /// Orthogonal Prism basis prime assigned to this Manager.
    pub basis_prime: u64,
    /// Content-addressed 32-byte SHA-256 `kappa` hex identity.
    pub kappa_hex: String,
}

/// Structured Operator prompt sent over Verdant's Communications Channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorPromptRequest {
    /// Target Viewpoint slug (`creator-studio`, `asset-catalogs`, `playable-world`, etc.) or `"auto"`.
    pub viewpoint: String,
    /// Natural-language or structured feature/refinement directive.
    pub directive: String,
    /// Optional explicit governing standard citation (e.g., `"AUTH-KHRONOS-GLTF2"`, `"ISO 42010"`).
    #[serde(default)]
    pub cited_authority: Option<String>,
    /// Sovereign `Prism id` of the operator submitting this directive.
    #[serde(default = "default_operator_prism_id")]
    pub operator_prism_id: String,
}

pub(super) fn default_operator_prism_id() -> String {
    "prism:verdant-operator".to_string()
}

/// Structured ISO 24617-2 response emitted by Verdant's Observer & owning Manager.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorPromptResponse {
    /// True iff the directive passed OMG DMN 1.5, anti-contrivance, and ISO 42010 Viewpoint gates.
    pub admitted: bool,
    /// ISO 24617-2 communicative act (`"Inform"` when admitted, `"Refusal"` when rejected).
    pub dialogue_act: String,
    /// Accountable Manager ID handling or refusing the directive.
    pub manager_id: String,
    /// VSM system (`VSM-S1`, `VSM-S2`, `VSM-S3-STAR`, `VSM-S5`).
    pub vsm_system: String,
    /// Target ISO 42010 Viewpoint slug.
    pub viewpoint_id: String,
    /// Owned web route if applicable.
    pub route: Option<String>,
    /// If redirected from a wrong Viewpoint, the true owning Viewpoint slug.
    pub recommended_viewpoint: Option<String>,
    /// Human- and machine-readable rationale citing exact authoritative rules.
    pub rationale: String,
    /// Authoritative `AUTH-*` standards bound to this directive.
    pub bound_authorities: Vec<String>,
    /// Repository files governed by the owning Manager.
    pub target_files: Vec<String>,
    /// Required elements the Manager enforces on its Viewpoint.
    pub required_elements: Vec<String>,
    /// Forbidden cross-contamination terms refused by this Manager.
    pub forbidden_cross_contamination: Vec<String>,
    /// Differentiation requirements enforced on this Manager's part.
    pub differentiation_requirements: Vec<String>,
    /// Verification commands to close the VSM S3* audit loop.
    pub verification_commands: Vec<String>,
    /// Signed W3C SOSA/PROV-O communication `kappa` recorded in `ObserverPrismHolon`.
    pub comm_kappa_hex: String,
}

/// Discoverable Verdant App Catalog manifest published onto Prism's Stitch 2-of-3 network.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StitchAppCatalogEntry {
    /// Schema.org `@type` (`VideoGame` or `SoftwareApplication`).
    pub schema_org_type: String,
    /// Content-addressed app identifier (`urn:verdant:stitch-app:<kappa-prefix>`).
    pub app_iri: String,
    /// Human-readable application title.
    pub title: String,
    /// Archetype slug (e.g. `first-person-shooter`, `massively-multiplayer-universe`, `indie-side-scroller`).
    pub archetype: String,
    /// Application summary.
    pub description: String,
    /// Sealed `.uorc` payload byte length.
    pub uorc_byte_len: usize,
    /// Content-addressed `kappa` of the sealed `.uorc` archive.
    pub uorc_kappa_hex: String,
    /// Signed `Prism id` (`prism/identity/4`) publishing envelope.
    pub publisher_envelope: PrismIdSessionEnvelope,
    /// Three `afflom/stitch` 2-of-3 erasure-coded shard digests `(A, B, A xor B)`.
    pub stitch_fragments: Vec<StitchFragment>,
}

/// Per-Manager VSM S3* file & surface audit result.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagerAuditResult {
    /// Accountable Manager ID.
    pub manager_id: String,
    /// ISO 42010 Viewpoint slug.
    pub viewpoint_id: String,
    /// VSM system code.
    pub vsm_system: String,
    /// Manager `kappa` hex identity.
    pub kappa_hex: String,
    /// True iff all owned files exist and every Viewpoint invariant passed.
    pub passed: bool,
    /// Violations detected (empty when `passed == true`).
    pub violations: Vec<String>,
}

/// Manager-synthesized atmospheric, lighting, acoustic, and archetype profile for one cataloged
/// world (`AUTH-DOMAIN-PACKS` + `AUTH-KHRONOS-GLTF2`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldDifferentiationProfile {
    /// Domain identifier (`DOM-SUNSPIRE-HARBOR`, etc.).
    pub domain_id: String,
    /// Stable world slug (`sunspire-regatta`, etc.).
    pub slug: String,
    /// Display world title.
    pub title: String,
    /// Deterministic world seed.
    pub seed: u64,
    /// Default LOD rank.
    pub default_lod_rank: u64,
    /// Normalized RGB sky zenith color (`[0..1]`).
    pub sky_rgb: [f32; 1 + 1 + 1],
    /// Normalized RGB horizon fog color (`[0..1]`).
    pub fog_rgb: [f32; 1 + 1 + 1],
    /// Environment title from `AUTH-DOMAIN-PACKS`.
    pub environment_name: String,
    /// Sabine room acoustic volume in cubic meters.
    pub acoustic_volume_m3: u64,
    /// Sabine surface absorption in milli-m².
    pub acoustic_absorption_milli_m2: u64,
    /// Default Verdant App archetype suited for this world.
    pub default_app_genre: String,
    /// Human-readable genre label.
    pub default_app_genre_label: String,
    /// Primary Schema.org application type.
    pub schema_org_type: String,
    /// Hero model names bound to this world.
    pub hero_models: Vec<String>,
    /// Character names & roles bound to this world.
    pub characters: Vec<String>,
    /// Content-addressed Manager profile `kappa`.
    pub profile_kappa_hex: String,
}

/// One materialized Stitch 2-of-3 parity fragment summary in the generated Verdant App Catalog.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StitchCatalogShardSummary {
    /// Shard slot (`0` = Shard A, `1` = Shard B, `2` = Parity `A ^ B`).
    pub index: u8,
    /// Human-readable role label.
    pub role: String,
    /// Encoded `STCH` wire byte size.
    pub bytes: usize,
    /// Hex SHA-256 prefix of shard payload root.
    pub sha256: String,
}

/// One browser-ready Verdant App entry materialized into `site/generated/stitch-app-catalog.json`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StitchCatalogManifestApp {
    /// Stable app identifier.
    pub app_id: String,
    /// App title.
    pub title: String,
    /// Genre slug (`first-person-shooter`, `mmo-universe`, etc.).
    pub genre: String,
    /// Display genre label.
    pub genre_label: String,
    /// Schema.org type (`VideoGame` or `SoftwareApplication`).
    pub schema_type: String,
    /// Target world domain slug from `AUTH-DOMAIN-PACKS`.
    pub domain_slug: String,
    /// World display title.
    pub world_title: String,
    /// Concise app summary.
    pub summary: String,
    /// Sovereign `Prism id` publisher.
    pub publisher_id: String,
    /// Archive format (always `.uorc`).
    pub archive_format: String,
    /// Exact sealed `.uorc` payload byte length.
    pub uorc_bytes: usize,
    /// Full SHA-256 hex digest of the sealed `.uorc` payload.
    pub payload_sha256: String,
    /// All 3 `afflom/stitch` 2-of-3 parity fragment commitments.
    pub stitch_shards: Vec<StitchCatalogShardSummary>,
    /// Deterministic RFC-3339 publication timestamp.
    pub published_at: String,
    /// Whether this entry is one of the Manager-curated flagship apps.
    pub seeded: bool,
}

/// File written by an accountable Manager during `synthesize` or `apply`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterializedFileRecord {
    /// Owning Manager ID.
    pub manager_id: String,
    /// Repository-relative path written.
    pub path: String,
    /// Written byte length.
    pub bytes: usize,
    /// SHA-256 hex digest of written contents.
    pub sha256_hex: String,
}

/// Structured result of `verdant operator synthesize` / `materialize_manager_artifacts`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterializationReport {
    /// Whether materialization and closing S3* audit succeeded.
    pub ok: bool,
    /// Files materialized strictly under Manager ownership.
    pub written_files: Vec<MaterializedFileRecord>,
    /// Signed PROV-O communication `kappa` for this materialization run.
    pub comm_kappa_hex: String,
    /// Closing VSM S3* audit status.
    pub audit_passed: bool,
}

/// Structured result of `verdant operator apply`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorApplyReport {
    /// Whether the directive was admitted AND materialized cleanly.
    pub applied: bool,
    /// Signed Manager prompt admission / refusal decision.
    pub decision: OperatorPromptResponse,
    /// Materialized files (guaranteed empty when refused).
    pub written_files: Vec<MaterializedFileRecord>,
    /// Post-apply VSM S3* audit result (present when applied).
    pub audit: Option<OperatorAuditReport>,
}

/// One cryptographically sealed view or generated artifact bound to its accountable Manager.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SealedFileEntry {
    /// Repository-relative file path.
    pub path: String,
    /// Accountable Manager ID from `AUTH-ISO-42010-VIEWPOINTS`.
    pub owning_manager_id: String,
    /// Content-addressed 32-byte Manager `kappa`.
    pub manager_kappa_hex: String,
    /// Authoritative standards governing this file.
    pub governing_authorities: Vec<String>,
    /// Exact byte length on disk.
    pub bytes: usize,
    /// Raw standard SHA-256 hex digest of file contents.
    pub sha256_hex: String,
    /// Signed W3C PROV-O communication `kappa` of the Verdant API/MCP operation that sealed it.
    pub last_directive_kappa_hex: String,
}

/// Cryptographic View Immutability Seal (`site/generated/immutability-seal.json`) proving every
/// shipped Verdant view and Manager-synthesized artifact was produced through Verdant's API/MCP.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewImmutabilitySeal {
    /// Seal schema identifier (`verdant/view-immutability-seal/1`).
    pub spec: String,
    /// Owning executive Manager (`mgr-observer-executive`).
    pub owning_manager: String,
    /// Executive Manager cryptographic `kappa`.
    pub manager_kappa_hex: String,
    /// Root SHA-256 commitment over all sorted `(path, managerKappaHex, sha256Hex)` entries.
    pub seal_root_sha256_hex: String,
    /// Sealed view and generated artifact entries.
    pub files: Vec<SealedFileEntry>,
}

/// Result of verifying `site/generated/immutability-seal.json` against live repository files.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImmutabilityVerificationReport {
    /// True iff `seal_root_sha256_hex` and every sealed view's raw SHA-256 match bit-for-bit.
    pub valid: bool,
    /// Number of sealed view files checked.
    pub checked_files: usize,
    /// Root SHA-256 commitment of the verified seal.
    pub seal_root_sha256_hex: String,
    /// Exact violations (unsealed files, seal root tamper, or out-of-band manual file edits).
    pub violations: Vec<String>,
}

/// Report proving 100% of Verdant's workspace (all shipped Rust crates, all 26 Prism LexLean
/// strata, all 10 governance/authority catalogs, and all distilled site Viewpoints) was lifted
/// into a `4-of-8` Reed-Solomon sharded [`VerdantVcs`] Stitch capsule (`VE-46`, M5).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfHostedLiftReport {
    /// Active SemVer string lifted.
    pub version: String,
    /// Monotonic Stitch release epoch.
    pub release_epoch: u64,
    /// Signed W3C PROV-O commit `kappa` in [`VerdantVcs`].
    pub source_commit_kappa: String,
    /// Deterministic Merkle tree root SHA-256 over every lifted workspace file.
    pub source_tree_root_sha256: String,
    /// Number of shipped Rust `.rs` + `Cargo.toml` files lifted.
    pub lifted_rust_files: usize,
    /// Number of Prism LexLean `.lex.tex` + lock files lifted.
    pub lifted_lex_files: usize,
    /// Number of governance/model/feature files lifted.
    pub lifted_governance_files: usize,
    /// Number of distilled `site/` Viewpoint & Wasm files lifted.
    pub lifted_site_files: usize,
    /// Count of unlifted shipped workspace modules (`0` required by M5 ratchet).
    pub unlifted_workspace_modules: usize,
    /// Byte length of the sealed `verdant-source-vX.Y.Z.uorc` capsule.
    pub source_uorc_bytes: usize,
    /// Count of `4-of-8` `STCH2` source shards written.
    pub source_shard_count: usize,
}

/// Result of recovering Verdant's full Source DAG from any 4 of 8 Stitch shards and autonomously
/// bootstrapping a chained `v(N+1)` release via `mgr-change-release` (`VE-46`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfHostedNextReleaseReport {
    /// Parent `VerdantVcs` commit `kappa` recovered from Stitch.
    pub parent_commit_kappa: String,
    /// Newly minted child `VerdantVcs` commit `kappa` proving acyclic `vN -> v(N+1)` lineage.
    pub child_commit_kappa: String,
    /// Previous SemVer string (`vN`).
    pub parent_version: String,
    /// Newly advanced SemVer string (`v(N+1)`).
    pub next_version: String,
    /// Previous monotonic epoch.
    pub parent_epoch: u64,
    /// Newly advanced monotonic epoch.
    pub next_epoch: u64,
    /// Parent release `.holo` `kappa` bound into `prev_kappa_hex`.
    pub prev_release_kappa: String,
    /// Newly packed `v(N+1)` `.holo` release `kappa`.
    pub next_release_kappa: String,
    /// Number of surviving `STCH2` source shards used (`k = 4` of `n = 8`).
    pub survived_source_shards: usize,
    /// Total lifted files cryptographically verified during bootstrap.
    pub verified_files_count: usize,
}

/// Options governing Verdant's autonomous self-lift automation loop (`verdant operator ratchet-lift`
/// and MCP `verdant_ratchet_lift`, `VE-47`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutonomousRatchetLiftOptions {
    /// When `true`, synthesizes Manager manifests, updates `site/generated/immutability-seal.json`,
    /// and writes `site/generated/self-host-ratchet-manifest.toml` directly into the repository.
    pub write_repo_artifacts: bool,
    /// When `true`, atomically updates `site/frame/frame-pin.json` with the newly verified
    /// quorum-signed `FramePinManifest`.
    pub update_frame_pin: bool,
    /// When `true` (default), one-way auto-tightens `[frontier_ratchet]` & `[surface_ratchet]`
    /// watermarks and binds `frontier_rust_tree_sha256` in `model/parametric-ratchet.toml` (`VE-55`).
    #[serde(default = "default_true_flag")]
    pub tighten_ratchet: bool,
    /// Optional Stitch relay URLs pinned into `FramePinManifest` (defaults to the versioned
    /// GitHub Pages Stitch shard path when `update_frame_pin` is true, or `./shards` otherwise).
    pub stitch_relays: Vec<String>,
    /// Optional Observer progress signals to advance SemVer via `mgr-change-release` when the
    /// Ratchet and Release Gates pass.
    pub advance_signals: Option<ObserverProgressSignals>,
}

fn default_true_flag() -> bool {
    true
}

impl Default for AutonomousRatchetLiftOptions {
    fn default() -> Self {
        Self {
            write_repo_artifacts: true,
            update_frame_pin: true,
            tighten_ratchet: true,
            stitch_relays: Vec::new(),
            advance_signals: None,
        }
    }
}

/// Complete cryptographic outcome of Verdant's autonomous self-lift loop (`VE-47`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutonomousRatchetLiftReport {
    /// True iff Total Ownership, Blank Frame audit, 4-of-8 `.holo` verification, and 100% module
    /// lifting succeeded with zero violations.
    pub passed: bool,
    /// Active SemVer release version (post-advancement if `advance_signals` was provided).
    pub version: String,
    /// Active monotonic Stitch release epoch.
    pub release_epoch: u64,
    /// True iff `mgr-change-release` incremented SemVer during this self-lift.
    pub version_incremented: bool,
    /// Content-addressed `VerdantVcs` Source DAG commit `kappa`.
    pub source_commit_kappa: String,
    /// Merkle tree root SHA-256 of the lifted `VerdantVcs` Source DAG.
    pub source_tree_root_sha256: String,
    /// Verified cryptographic View Immutability Seal root SHA-256.
    pub immutability_seal_root_sha256: String,
    /// Verified `verdant-v<version>.holo` capsule SHA-256.
    pub release_capsule_sha256: String,
    /// Total lifted Rust + LexLean + Viewpoint UI modules SHA-256-recorded in the ratchet manifest.
    pub lifted_module_count: usize,
    /// Count of unlifted shipped workspace modules (`0` required).
    pub unlifted_workspace_modules: usize,
    /// Count of unlifted UI Viewpoint files (`0` required).
    pub unlifted_ui_files: usize,
    /// Count of unverified/loose assets inside `site/frame/` (`0` required).
    pub unverified_frame_assets: usize,
    /// Count of user-required external account / OAuth walls (`0` required).
    pub user_required_external_accounts: usize,
    /// Total byte weight of the immutable Blank Frame (`<= 30,720 B`).
    pub blank_frame_bytes: usize,
    /// Summary string returned by 4-of-8 erasure recovery verification (`verify_release_dir`).
    pub release_verification_summary: String,
    /// True iff `site/frame/frame-pin.json` was updated.
    pub frame_pin_updated: bool,
    /// Ratchet watermarks automatically tightened (clicked UP/DOWN) during this self-lift (`VE-55`).
    pub tightened_ratchet_fields: Vec<String>,
    /// Path where the canonical `self-host-ratchet-manifest.toml` was emitted.
    pub manifest_path: String,
}

/// Declarative mutation request routed exclusively through Verdant's API / MCP (`verdant_mutate_view`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedFileMutationRequest {
    /// Target ISO 42010 viewpoint (`creator-studio`, `asset-catalogs`, `design-system`, etc.).
    pub viewpoint: String,
    /// Human or agent directive justifying the standards-backed change.
    pub directive: String,
    /// Mandatory governing `AUTH-*` standard ID.
    pub cited_authority: String,
    /// Repository-relative path inside the owning Manager's `ownedFiles`.
    pub path: String,
    /// Mutation mode (`"replace_exact"` or `"write_full"`).
    pub mode: String,
    /// Exact target substring for `"replace_exact"`.
    #[serde(default)]
    pub target: Option<String>,
    /// Replacement substring for `"replace_exact"`.
    #[serde(default)]
    pub replacement: Option<String>,
    /// Full file contents for `"write_full"`.
    #[serde(default)]
    pub content: Option<String>,
    /// Sovereign `Prism id` actor (`prism:...`).
    #[serde(default = "default_operator_prism_id")]
    pub operator_prism_id: String,
}

/// Structured result of `verdant_mutate_view` / `mutate_owned_file`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedFileMutationReport {
    /// True iff all pre-write Manager gates, re-sealing, and closing S3* audit passed.
    pub mutated: bool,
    /// Owning Manager ID that admitted or refused the mutation.
    pub manager_id: String,
    /// Target path.
    pub path: String,
    /// Signed W3C PROV-O `kappa` for this mutation event.
    pub comm_kappa_hex: String,
    /// Updated raw SHA-256 of the file (empty on refusal).
    pub sha256_hex: String,
    /// Updated Immutability Seal root digest (empty on refusal).
    pub seal_root_sha256_hex: String,
    /// Admission rationale or exact refusal diagnostic.
    pub diagnostic: String,
}

/// Complete VSM S3* + Total Ownership audit report over the live repository.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorAuditReport {
    /// True iff Total Ownership AND every Manager's S3* audit passed.
    pub passed: bool,
    /// Total Ownership coverage report (zero orphans / zero duplicates).
    pub ownership: TotalOwnershipReport,
    /// Per-Manager verification results.
    pub managers: Vec<ManagerAuditResult>,
    /// Number of differentiated world packs verified in `AUTH-DOMAIN-PACKS`.
    pub differentiated_worlds: usize,
}

impl VerdantOperatorChannel {
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

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn exec_mcp_authority_or_gateway_tool(
        repo_root: &Path,
        tool_name: &str,
        args: &serde_json::Value,
    ) -> (bool, serde_json::Value) {
        if tool_name == "verdant_authority_exec" {
            let rel_ledger = args
                .get("ledgerPath")
                .and_then(|v| v.as_str())
                .unwrap_or(".verdant/authority-ledger.json");
            let ledger_path = if Path::new(rel_ledger).is_absolute() {
                std::path::PathBuf::from(rel_ledger)
            } else {
                repo_root.join(rel_ledger)
            };
            if let Some(owner) = args.get("initOwnerPrismId").and_then(|v| v.as_str()) {
                let rname = args
                    .get("initRepoName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("verdant-engine");
                let sec = args
                    .get("initOwnerSecret")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                match crate::authority::SovereignAuthorityLedger::init_genesis(rname, owner, sec) {
                    Ok(ledger) => {
                        if let Some(parent) = ledger_path.parent() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                        let _ = std::fs::write(
                            &ledger_path,
                            serde_json::to_string_pretty(&ledger).unwrap_or_default(),
                        );
                        (
                            false,
                            serde_json::to_value(&ledger).unwrap_or(serde_json::Value::Null),
                        )
                    }
                    Err(e) => (true, serde_json::json!({ "error": e.to_string() })),
                }
            } else {
                let cmd_val = args
                    .get("command")
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!({ "cmd": "verifyChain" }));
                let res = std::fs::read_to_string(&ledger_path)
                    .map_err(|e| format!("read ledger `{}`: {e}", ledger_path.display()))
                    .and_then(|raw| {
                        serde_json::from_str::<crate::authority::SovereignAuthorityLedger>(&raw)
                            .map_err(|e| format!("parse ledger: {e}"))
                    })
                    .and_then(|mut ledger| {
                        let cmd: crate::authority::AuthorityCommand =
                            serde_json::from_value(cmd_val)
                                .map_err(|e| format!("invalid command: {e}"))?;
                        let out = ledger.execute(cmd).map_err(|e| e.to_string())?;
                        std::fs::write(
                            &ledger_path,
                            serde_json::to_string_pretty(&ledger).unwrap_or_default(),
                        )
                        .map_err(|e| format!("write ledger: {e}"))?;
                        Ok(out)
                    });
                match res {
                    Ok(out) => (
                        false,
                        serde_json::to_value(&out).unwrap_or(serde_json::Value::Null),
                    ),
                    Err(e) => (true, serde_json::json!({ "error": e })),
                }
            }
        } else {
            let act = args
                .get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("verify");
            let dir = repo_root.join(
                args.get("dir")
                    .and_then(|v| v.as_str())
                    .unwrap_or("dist/gateway-bundle"),
            );
            let op = if act == "pack" {
                crate::gateway::GatewayOp::PackBundle {
                    repo_root: repo_root.to_path_buf(),
                    release_dir: repo_root.join(
                        args.get("releaseDir")
                            .and_then(|v| v.as_str())
                            .unwrap_or("dist/stitch-release"),
                    ),
                    out_dir: dir,
                    extra_gateways: Vec::new(),
                    crates_dir: None,
                }
            } else {
                crate::gateway::GatewayOp::VerifyFailover { bundle_dir: dir }
            };
            match crate::gateway::execute_gateway_op(&op) {
                Ok((o, _)) => (
                    false,
                    serde_json::to_value(&o).unwrap_or(serde_json::Value::Null),
                ),
                Err(e) => (true, serde_json::json!({ "error": e.to_string() })),
            }
        }
    }
}
