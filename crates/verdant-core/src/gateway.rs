//! **Public-Benefit Multi-Gateway Stitch Hosting, Portable Bundle Emitter & Native HTTP Gateway (`gateway.rs`, `VE-77`, `VE-78`)**.
//!
//! Ensures Verdant Engine (its `<= 30 KiB` Blank Frame, verified `.holo` capsule, `4-of-8`
//! `STCH2` Reed-Solomon shards, all 6 stakeholder parts, and optional RFC 2789 Cargo Sparse
//! Registry) is hosted and served simultaneously on **GitHub** AND through **zero-registration
//! public-benefit Stitch carriers** (Stitch Layer-5 WebRTC peer swarm, volunteer `verdant gateway serve`
//! / OCI nodes, permissionless IPFS/IPNS HTTP gateways, Zenodo permanent DOI archives, and jsDelivr CDN)
//! with ZERO commercial cloud PaaS provider registrations outside GitHub and full primary-outage failover.

use std::collections::BTreeSet;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use verdant_tensor::scalars::{
    gateway_provider_families_independent, gateway_stakeholder_part_count_complete, two_u32,
};

use crate::digest::sha256_hex;
use crate::error::ObserverModelError;
use crate::observer::CommunicationChannelKind;
use crate::vcs::FramePinManifest;

/// Supported public-benefit Verdant Stitch hosting carrier classes (`AUTH-STITCH-SELF-HOSTING`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GatewayProviderKind {
    /// GitHub Pages static hosting (`*.github.io`).
    GitHubPages,
    /// GitHub raw `stitch-mirror` branch (`raw.githubusercontent.com`).
    GitHubRawMirror,
    /// jsDelivr public CDN mirror over `stitch-mirror` (`cdn.jsdelivr.net`).
    JsDelivrCdn,
    /// Permanent public-benefit Zenodo DOI archive (`zenodo.org`).
    ZenodoPublicArchive,
    /// Zero-account Stitch Layer-5 `stitch/1` signaling & WebRTC peer swarm (`ntfy.sh`, `stitch-swarm`).
    StitchPeerSwarm,
    /// Content-addressed IPFS / IPNS HTTP gateway (`ipfs.io`, `dweb.link`, `*.ipfs.w3s.link`).
    IpfsHttpGateway,
    /// Volunteer or LAN self-hosted Verdant Stitch HTTP gateway (`verdant gateway serve` or OCI container).
    SelfHostedStitchNode,
}

/// Canonical stakeholder part deep-link exposed identically across every Verdant gateway.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakeholderPartLink {
    /// Short route slug (`studio`, `catalogs`, `demo`, `docs`, `starter`, `card`).
    pub part_id: String,
    /// Human-readable surface label.
    pub label: String,
    /// Canonical named Stitch URI (`stitch://verdant/<part>`).
    pub stitch_uri: String,
    /// Immutable content-addressed Stitch URI (`stitch://<releaseKappaHex>/<part>`).
    pub content_addressed_uri: String,
    /// Verified ServiceWorker Virtual FS entry route inside the `.holo` capsule.
    pub virtual_route: String,
    /// Clean static/edge gateway path (`/part/<part>`).
    pub gateway_path: String,
    /// Query deep-link mounted directly by the Blank Frame (`/?pin=stitch://verdant/<part>`).
    pub query_path: String,
}

/// Concrete Stitch hosting gateway endpoint participating in multi-gateway distribution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StitchGatewayEndpoint {
    /// Unique gateway identifier (e.g. `github-pages`, `stitch-swarm`, `ipfs-dweb`, `zenodo-doi`).
    pub id: String,
    /// Hosting provider class.
    pub provider: GatewayProviderKind,
    /// Independent provider family (`github`, `jsdelivr`, `zenodo`, `stitch-swarm`, `ipfs`, `self-hosted`).
    pub provider_family: String,
    /// Base HTTPS (or localhost loopback) gateway URL.
    pub base_url: String,
    /// Blank Frame entry URL hosted by this gateway.
    pub frame_url: String,
    /// CORS-enabled `4-of-8` `STCH2` shard relay base URL.
    pub shard_relay_url: String,
    /// Optional direct portable `.holo` capsule URL.
    pub holo_url: Option<String>,
    /// Optional RFC 2789 Cargo Sparse Registry index URL.
    pub crates_index_url: Option<String>,
    /// Lower priority values are preferred first during parallel racing (`1..=100`).
    pub priority: u32,
    /// Whether `Access-Control-Allow-Origin: *` is required on cross-origin shard fetches.
    pub cors_wildcard_required: bool,
}

/// Cryptographically committed multi-gateway hosting manifest (`verdant/multi-gateway-manifest/1`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiGatewayManifest {
    /// Fixed schema identifier (`verdant/multi-gateway-manifest/1`).
    pub spec: String,
    /// Semantic version of the hosted Verdant release.
    pub release_version: String,
    /// Strictly monotonic Stitch release epoch.
    pub release_epoch: u64,
    /// Verified SHA-256 `kappa` of `verdant-v<version>.holo`.
    pub release_kappa_hex: String,
    /// Reed-Solomon recovery threshold `k` (`4`).
    pub data_shards_k: u8,
    /// Reed-Solomon total shards `n` (`8`).
    pub total_shards_n: u8,
    /// Canonical shard file names (`shard-0.stch..shard-7.stch`).
    pub shard_files: Vec<String>,
    /// All participating GitHub and independent non-GitHub hosting gateways.
    pub gateways: Vec<StitchGatewayEndpoint>,
    /// Distinct independent provider families (`>= 2` required; GitHub-only plans are refused).
    pub distinct_provider_families: Vec<String>,
    /// All 6 canonical stakeholder parts reachable from every gateway.
    pub stakeholder_parts: Vec<StakeholderPartLink>,
    /// SHA-256 commitment over the canonical manifest.
    pub manifest_kappa_hex: String,
}

/// Telemetry returned by a bounded [`GatewayOp::Serve`] Stitch HTTP gateway run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayServeReport {
    /// Requested bind address.
    pub bind_addr: String,
    /// Actual bound TCP port.
    pub bound_port: u16,
    /// Total HTTP requests handled.
    pub requests_served: usize,
    /// Binary `STCH2` shard responses served (`200 OK`).
    pub shard_requests: usize,
    /// Stakeholder `/part/<name>` redirects served (`302 Found`).
    pub part_redirects: usize,
    /// `/healthz` readiness probes served.
    pub healthz_checks: usize,
    /// `/gateway-manifest.json` requests served.
    pub manifest_requests: usize,
    /// `/swarm` Stitch Layer-5 volunteer bridge requests served.
    pub swarm_requests: usize,
    /// `404 Not Found` responses served.
    pub not_found_count: usize,
}

/// Live dual-gateway failover & adapter audit report (`VE-77`, `VE-79`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayFailoverReport {
    /// Fixed schema identifier (`verdant/gateway-failover-report/1`).
    pub spec: String,
    /// Verified release version.
    pub release_version: String,
    /// Verified release epoch.
    pub release_epoch: u64,
    /// Verified `.holo` SHA-256 `kappa`.
    pub release_kappa_hex: String,
    /// Simulated GitHub primary outage status (`503`).
    pub primary_outage_status: u16,
    /// Independent secondary gateway `/healthz` status (`200`).
    pub secondary_healthz_status: u16,
    /// Stakeholder `/part/studio` redirect status (`302`).
    pub part_redirect_status: u16,
    /// CORS preflight `OPTIONS` status (`204`).
    pub cors_preflight_status: u16,
    /// Secondary volunteer gateway `/swarm` Stitch Layer-5 bridge status (`200`).
    pub swarm_endpoint_status: u16,
    /// Whether `verdant-portable-v<ver>.html` verified its embedded `.holo` capsule.
    pub portable_html_verified: bool,
    /// Surviving shards fetched from the independent secondary gateway (`4`).
    pub surviving_shards_fetched: u32,
    /// Withheld shards tolerated during failover (`4`).
    pub withheld_shards_tolerated: u32,
    /// Lifted files verified inside the recovered `.holo` capsule.
    pub recovered_files_count: usize,
    /// Verified hosting adapter files (`_headers`, `_redirects`, `stitch-swarm-config.json`, `.ipfs-gateway-root.json`, `Dockerfile`).
    pub adapters_verified: Vec<String>,
    /// Independent provider families committed in `gateway-manifest.json`.
    pub distinct_provider_families: Vec<String>,
    /// True iff all failover, CORS, stakeholder part, swarm, portable HTML, and 4-of-8 recovery assertions passed.
    pub all_passed: bool,
}

/// Native multi-gateway Stitch hosting operations dispatched via [`execute_gateway_op`].
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GatewayOp {
    /// Emits a self-contained drop-in public-benefit Stitch bundle deployable identically to GitHub Pages,
    /// Stitch Layer-5 peer swarms, IPFS/IPNS, Zenodo archives, volunteer OCI/Docker, and `verdant gateway serve`.
    PackBundle {
        /// Repository root containing `site/frame/`.
        repo_root: PathBuf,
        /// Packed Stitch release directory (`frame-pin.json`, `shards/*.stch`, `verdant-v*.holo`).
        release_dir: PathBuf,
        /// Output directory for the portable multi-gateway bundle.
        out_dir: PathBuf,
        /// Additional public-benefit gateway endpoints (Stitch swarm, IPFS, Zenodo, volunteer self-hosted, etc.).
        extra_gateways: Vec<StitchGatewayEndpoint>,
        /// Optional RFC 2789 Cargo Sparse Registry directory (`dist/crates`) to include.
        crates_dir: Option<PathBuf>,
    },
    /// Runs Verdant's native zero-dependency HTTP/1.1 + CORS `*` Stitch Gateway server + `/swarm` bridge.
    Serve {
        /// Multi-gateway bundle directory emitted by `PackBundle`.
        bundle_dir: PathBuf,
        /// TCP socket bind address (e.g. `127.0.0.1:8088` or `0.0.0.0:8080`).
        bind_addr: String,
        /// Optional bounded request count before graceful exit (`None` runs indefinitely).
        max_requests: Option<usize>,
        /// Optional Stitch Layer-5 text relay base URL (e.g. `https://ntfy.sh`) for volunteer announcements.
        swarm_relay: Option<String>,
    },
    /// Executes live dual-gateway failover verification: primary outage (`503`) + secondary
    /// independent gateway with `4` of `8` shards withheld -> bit-exact `.holo` recovery.
    VerifyFailover {
        /// Multi-gateway bundle directory to verify.
        bundle_dir: PathBuf,
    },
}

/// Structured result returned by [`execute_gateway_op`].
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum GatewayOpOutcome {
    /// Portable multi-gateway bundle emitted.
    Packed(MultiGatewayManifest),
    /// Native Stitch HTTP gateway finished serving.
    Served(GatewayServeReport),
    /// Live dual-gateway failover verification passed.
    Verified(GatewayFailoverReport),
}

/// Builds and cryptographically commits a [`MultiGatewayManifest`] from `pin` and `extra_gateways`,
/// enforcing `>= 2` independent provider families (refusing GitHub-only plans), safe HTTPS/localhost
/// origins, and complete exposure of all 6 canonical Verdant stakeholder parts (`VE-77`).
pub fn build_multi_gateway_manifest(
    pin: &FramePinManifest,
    extra_gateways: &[StitchGatewayEndpoint],
) -> Result<MultiGatewayManifest, ObserverModelError> {
    let make_err = |detail: String| ObserverModelError::CommunicationEnvelopeRejected {
        channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
        detail,
    };

    let classify_url =
        |raw: &str| -> Result<(GatewayProviderKind, String, String), ObserverModelError> {
            let clean = raw.trim().trim_end_matches('/').to_string();
            let is_https = clean.starts_with("https://");
            let is_local =
                clean.starts_with("http://127.0.0.1") || clean.starts_with("http://localhost");
            if !is_https && !is_local {
                return Err(make_err(format!(
                    "refused non-HTTPS/non-localhost Stitch gateway URL: `{clean}`"
                )));
            }
            let host = clean
                .split("://")
                .nth(1)
                .and_then(|rest| rest.split('/').next())
                .unwrap_or("")
                .to_ascii_lowercase();
            if host.is_empty() {
                return Err(make_err(format!(
                    "Stitch gateway URL missing host: `{clean}`"
                )));
            }
            if host.ends_with(".pages.dev")
                || host.ends_with(".workers.dev")
                || host.contains("cloudflare")
                || host.ends_with(".netlify.app")
                || host.contains("netlify")
                || host.ends_with(".vercel.app")
                || host.contains("vercel")
                || host.contains("pinata")
            {
                return Err(make_err(format!(
                    "refused commercial cloud PaaS gateway URL (Verdant is a free public-benefit service with zero non-GitHub commercial provider registrations): `{clean}`"
                )));
            }
            let (kind, family) = if host.ends_with(".github.io") || host == "github.io" {
                (GatewayProviderKind::GitHubPages, "github")
            } else if host.ends_with("githubusercontent.com") || host == "github.com" {
                (GatewayProviderKind::GitHubRawMirror, "github")
            } else if host.ends_with("jsdelivr.net") {
                (GatewayProviderKind::JsDelivrCdn, "jsdelivr")
            } else if host == "zenodo.org" || host.ends_with(".zenodo.org") {
                (GatewayProviderKind::ZenodoPublicArchive, "zenodo")
            } else if host == "ntfy.sh"
                || host.ends_with(".ntfy.sh")
                || host.contains("stitch-swarm")
            {
                (GatewayProviderKind::StitchPeerSwarm, "stitch-swarm")
            } else if host == "ipfs.io"
                || host == "dweb.link"
                || host.ends_with(".ipfs.w3s.link")
                || host.contains("ipfs")
            {
                (GatewayProviderKind::IpfsHttpGateway, "ipfs")
            } else {
                (GatewayProviderKind::SelfHostedStitchNode, "self-hosted")
            };
            Ok((kind, family.to_string(), clean))
        };

    let mut gateways: Vec<StitchGatewayEndpoint> = Vec::new();
    let mut seen_relays: BTreeSet<String> = BTreeSet::new();
    let mut next_priority: u32 = 1;

    for relay in &pin.stitch_relays {
        if relay.starts_with("./") {
            continue;
        }
        let (provider, provider_family, clean_relay) = classify_url(relay)?;
        if !seen_relays.insert(clean_relay.clone()) {
            continue;
        }
        let base_url = clean_relay
            .split("/stitch/")
            .next()
            .unwrap_or(&clean_relay)
            .trim_end_matches("/shards")
            .to_string();
        let frame_url = if provider == GatewayProviderKind::GitHubPages {
            format!("{base_url}/frame/")
        } else {
            format!("{base_url}/")
        };
        let crates_index_url = if provider == GatewayProviderKind::GitHubPages {
            Some(format!("{base_url}/crates/index/"))
        } else {
            None
        };
        gateways.push(StitchGatewayEndpoint {
            id: format!("{provider_family}-{next_priority}"),
            provider,
            provider_family,
            base_url,
            frame_url,
            shard_relay_url: clean_relay,
            holo_url: None,
            crates_index_url,
            priority: next_priority,
            cors_wildcard_required: true,
        });
        next_priority = next_priority.saturating_add(1);
    }

    for extra in extra_gateways {
        let (detected_kind, detected_family, clean_relay) = classify_url(&extra.shard_relay_url)?;
        let (_, _, clean_base) = classify_url(&extra.base_url)?;
        let (_, _, clean_frame) = classify_url(&extra.frame_url)?;
        if !seen_relays.insert(clean_relay.clone()) {
            continue;
        }
        let family = if extra.provider_family.trim().is_empty() {
            detected_family
        } else {
            extra.provider_family.trim().to_ascii_lowercase()
        };
        let mut ep = extra.clone();
        if ep.id.trim().is_empty() {
            ep.id = format!("{family}-{next_priority}");
        }
        if ep.provider == GatewayProviderKind::SelfHostedStitchNode
            && detected_kind != GatewayProviderKind::SelfHostedStitchNode
        {
            ep.provider = detected_kind;
        }
        ep.provider_family = family;
        ep.base_url = clean_base;
        ep.frame_url = clean_frame;
        ep.shard_relay_url = clean_relay;
        if ep.priority == 0 {
            ep.priority = next_priority;
        }
        gateways.push(ep);
        next_priority = next_priority.saturating_add(1);
    }

    gateways.sort_by_key(|g| (g.priority, g.id.clone()));

    let mut families_set: BTreeSet<String> = BTreeSet::new();
    for g in &gateways {
        families_set.insert(g.provider_family.clone());
    }
    let distinct_provider_families: Vec<String> = families_set.into_iter().collect();
    if !gateway_provider_families_independent(distinct_provider_families.len() as u32) {
        return Err(make_err(format!(
            "multi-gateway resilience requires >= {} independent provider families (found {:?}); add an independent public-benefit non-GitHub carrier (Stitch Layer-5 swarm, IPFS, Zenodo, jsDelivr, or volunteer self-hosted)",
            two_u32(),
            distinct_provider_families
        )));
    }

    let part_specs = [
        ("studio", "Verdant Engine Studio", "studio/index.html"),
        (
            "catalogs",
            "Standards & World Catalogs",
            "catalogs/index.html",
        ),
        ("demo", "Interactive Showcase Demo", "demo/index.html"),
        ("docs", "Executable Diataxis Docs", "docs/index.html"),
        ("starter", "Browser 3D Starter", "starter/index.html"),
        ("card", "Verdant Identity Card", "card.html"),
    ];
    let stakeholder_parts: Vec<StakeholderPartLink> = part_specs
        .iter()
        .map(|&(part_id, label, virtual_route)| StakeholderPartLink {
            part_id: part_id.to_string(),
            label: label.to_string(),
            stitch_uri: format!("stitch://verdant/{part_id}"),
            content_addressed_uri: format!("stitch://{}/{part_id}", pin.release_kappa_hex),
            virtual_route: virtual_route.to_string(),
            gateway_path: format!("/part/{part_id}"),
            query_path: format!("/?pin=stitch://verdant/{part_id}"),
        })
        .collect();

    if !gateway_stakeholder_part_count_complete(stakeholder_parts.len() as u32) {
        return Err(make_err(
            "multi-gateway manifest missing canonical stakeholder parts".to_string(),
        ));
    }

    let mut manifest = MultiGatewayManifest {
        spec: "verdant/multi-gateway-manifest/1".to_string(),
        release_version: pin.release_version.clone(),
        release_epoch: pin.release_epoch,
        release_kappa_hex: pin.release_kappa_hex.clone(),
        data_shards_k: pin.data_shards_k,
        total_shards_n: pin.total_shards_n,
        shard_files: pin.shard_files.clone(),
        gateways,
        distinct_provider_families,
        stakeholder_parts,
        manifest_kappa_hex: String::new(),
    };
    let canonical = serde_json::to_vec(&manifest)
        .map_err(|e| make_err(format!("serialize multi-gateway manifest: {e}")))?;
    manifest.manifest_kappa_hex = sha256_hex(&canonical);
    Ok(manifest)
}

#[cfg(not(target_arch = "wasm32"))]
pub use crate::gateway_swarm::execute_gateway_op;
