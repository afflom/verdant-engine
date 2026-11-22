//! **Multi-Gateway Stitch Hosting, Portable Edge Bundle Emitter & Native HTTP Gateway (`gateway.rs`, `VE-77`)**.
//!
//! Ensures Verdant Engine (its `<= 30 KiB` Blank Frame, verified `.holo` capsule, `4-of-8`
//! `STCH2` Reed-Solomon shards, all 6 stakeholder parts, and optional RFC 2789 Cargo Sparse
//! Registry) is hosted and served simultaneously on **GitHub** AND through **independent
//! non-GitHub gateways** (Cloudflare Pages/Workers, Netlify Edge, IPFS/IPNS HTTP gateways,
//! jsDelivr CDN, and self-hosted `verdant gateway serve` nodes) with zero external account
//! requirements and full primary-outage failover.

use std::collections::{BTreeMap, BTreeSet};
#[cfg(not(target_arch = "wasm32"))]
use std::io::{Read, Write};
#[cfg(not(target_arch = "wasm32"))]
use std::net::{TcpListener, TcpStream};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use verdant_tensor::scalars::{
    eight_u32, four_u32, gateway_blank_frame_budget_ok, gateway_http_status_no_content,
    gateway_http_status_ok, gateway_http_status_redirect, gateway_http_status_unavailable,
    gateway_immutable_cache_max_age_secs,
};
use verdant_tensor::scalars::{
    gateway_provider_families_independent, gateway_stakeholder_part_count_complete, two_u32,
};

use crate::digest::sha256_hex;
use crate::error::ObserverModelError;
use crate::observer::CommunicationChannelKind;
use crate::vcs::FramePinManifest;

/// Supported Verdant Stitch hosting gateway provider classes (`AUTH-STITCH-SELF-HOSTING`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GatewayProviderKind {
    /// GitHub Pages static hosting (`*.github.io`).
    GitHubPages,
    /// GitHub raw `stitch-mirror` branch (`raw.githubusercontent.com`).
    GitHubRawMirror,
    /// jsDelivr global CDN mirror over `stitch-mirror` (`cdn.jsdelivr.net`).
    JsDelivrCdn,
    /// Cloudflare Pages / Workers edge gateway (`*.pages.dev`, `*.workers.dev`, custom domain).
    CloudflarePages,
    /// Netlify Edge static gateway (`*.netlify.app`, custom domain).
    NetlifyEdge,
    /// Content-addressed IPFS / IPNS HTTP gateway (`ipfs.io`, `dweb.link`, `*.ipfs.w3s.link`).
    IpfsHttpGateway,
    /// Self-hosted Verdant Stitch HTTP gateway (`verdant gateway serve` or OCI container).
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
    /// Unique gateway identifier (e.g. `github-pages`, `cloudflare-pages`, `ipfs-dweb`).
    pub id: String,
    /// Hosting provider class.
    pub provider: GatewayProviderKind,
    /// Independent provider family (`github`, `jsdelivr`, `cloudflare`, `netlify`, `ipfs`, `self-hosted`).
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
    /// `404 Not Found` responses served.
    pub not_found_count: usize,
}

/// Live dual-gateway failover & adapter audit report (`VE-77`).
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
    /// Surviving shards fetched from the independent secondary gateway (`4`).
    pub surviving_shards_fetched: u32,
    /// Withheld shards tolerated during failover (`4`).
    pub withheld_shards_tolerated: u32,
    /// Lifted files verified inside the recovered `.holo` capsule.
    pub recovered_files_count: usize,
    /// Verified hosting adapter files (`_headers`, `_redirects`, `wrangler.toml`, `netlify.toml`, `.ipfs-gateway-root.json`, `Dockerfile`).
    pub adapters_verified: Vec<String>,
    /// Independent provider families committed in `gateway-manifest.json`.
    pub distinct_provider_families: Vec<String>,
    /// True iff all failover, CORS, stakeholder part, and 4-of-8 recovery assertions passed.
    pub all_passed: bool,
}

/// Native multi-gateway Stitch hosting operations dispatched via [`execute_gateway_op`].
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GatewayOp {
    /// Emits a self-contained drop-in multi-gateway bundle deployable identically to GitHub Pages,
    /// Cloudflare Pages/Workers, Netlify, IPFS/IPNS, OCI/Docker, and `verdant gateway serve`.
    PackBundle {
        /// Repository root containing `site/frame/`.
        repo_root: PathBuf,
        /// Packed Stitch release directory (`frame-pin.json`, `shards/*.stch`, `verdant-v*.holo`).
        release_dir: PathBuf,
        /// Output directory for the portable multi-gateway bundle.
        out_dir: PathBuf,
        /// Additional external gateway endpoints (Cloudflare, Netlify, IPFS, self-hosted, etc.).
        extra_gateways: Vec<StitchGatewayEndpoint>,
        /// Optional RFC 2789 Cargo Sparse Registry directory (`dist/crates`) to include.
        crates_dir: Option<PathBuf>,
    },
    /// Runs Verdant's native zero-dependency HTTP/1.1 + CORS `*` Stitch Gateway server.
    Serve {
        /// Multi-gateway bundle directory emitted by `PackBundle`.
        bundle_dir: PathBuf,
        /// TCP socket bind address (e.g. `127.0.0.1:8088` or `0.0.0.0:8080`).
        bind_addr: String,
        /// Optional bounded request count before graceful exit (`None` runs indefinitely).
        max_requests: Option<usize>,
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
            let (kind, family) = if host.ends_with(".github.io") || host == "github.io" {
                (GatewayProviderKind::GitHubPages, "github")
            } else if host.ends_with("githubusercontent.com") || host == "github.com" {
                (GatewayProviderKind::GitHubRawMirror, "github")
            } else if host.ends_with("jsdelivr.net") {
                (GatewayProviderKind::JsDelivrCdn, "jsdelivr")
            } else if host.ends_with(".pages.dev")
                || host.ends_with(".workers.dev")
                || host.contains("cloudflare")
            {
                (GatewayProviderKind::CloudflarePages, "cloudflare")
            } else if host.ends_with(".netlify.app") || host.contains("netlify") {
                (GatewayProviderKind::NetlifyEdge, "netlify")
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
            "multi-gateway resilience requires >= {} independent provider families (found {:?}); add an independent non-GitHub gateway (Cloudflare, Netlify, IPFS, jsDelivr, or self-hosted)",
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

/// Executes a native multi-gateway Stitch hosting operation (`PackBundle`, `Serve`, or
/// `VerifyFailover`), providing GitHub + Cloudflare + Netlify + IPFS + Self-Hosted parity (`VE-77`).
#[cfg(not(target_arch = "wasm32"))]
pub fn execute_gateway_op(
    op: &GatewayOp,
) -> Result<(GatewayOpOutcome, String), ObserverModelError> {
    let make_err = |detail: String| ObserverModelError::CommunicationEnvelopeRejected {
        channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
        detail,
    };

    match op {
        GatewayOp::PackBundle {
            repo_root,
            release_dir,
            out_dir,
            extra_gateways,
            crates_dir,
        } => {
            let pin_raw = std::fs::read_to_string(release_dir.join("frame-pin.json"))
                .map_err(|e| make_err(format!("read release frame-pin.json: {e}")))?;
            let mut pin: FramePinManifest = serde_json::from_str(&pin_raw)
                .map_err(|e| make_err(format!("parse release frame-pin.json: {e}")))?;
            let manifest = build_multi_gateway_manifest(&pin, extra_gateways)?;

            // Merge every verified gateway shard relay URL into the bundle's frame-pin.json:
            let mut merged_relays: Vec<String> = Vec::new();
            let mut seen_r: BTreeSet<String> = BTreeSet::new();
            for g in &manifest.gateways {
                if seen_r.insert(g.shard_relay_url.clone()) {
                    merged_relays.push(g.shard_relay_url.clone());
                }
            }
            if !merged_relays.is_empty() {
                pin.stitch_relays = merged_relays;
            }

            let shards_out = out_dir.join("shards");
            let versioned_shards_out = out_dir
                .join("stitch")
                .join(format!("v{}", pin.release_version))
                .join("shards");
            let holo_out = out_dir.join("holo");
            std::fs::create_dir_all(&shards_out)
                .map_err(|e| make_err(format!("create gateway shards dir: {e}")))?;
            std::fs::create_dir_all(&versioned_shards_out)
                .map_err(|e| make_err(format!("create versioned shards dir: {e}")))?;
            std::fs::create_dir_all(&holo_out)
                .map_err(|e| make_err(format!("create gateway holo dir: {e}")))?;

            // Copy Blank Frame static shell (`<= 30 KiB`) to bundle root:
            let frame_src = repo_root.join("site/frame");
            let mut frame_total_bytes = 0u64;
            for name in ["index.html", "frame-boot.mjs", "frame-sw.js"] {
                let bytes = std::fs::read(frame_src.join(name))
                    .map_err(|e| make_err(format!("read site/frame/{name}: {e}")))?;
                frame_total_bytes = frame_total_bytes.saturating_add(bytes.len() as u64);
                std::fs::write(out_dir.join(name), &bytes)
                    .map_err(|e| make_err(format!("write gateway {name}: {e}")))?;
            }
            let pin_pretty = format!(
                "{}\n",
                serde_json::to_string_pretty(&pin)
                    .map_err(|e| make_err(format!("serialize bundle frame-pin.json: {e}")))?
            );
            frame_total_bytes = frame_total_bytes.saturating_add(pin_pretty.len() as u64);
            if !gateway_blank_frame_budget_ok(frame_total_bytes) {
                return Err(make_err(format!(
                    "gateway Blank Frame exceeds 30 KiB budget ({frame_total_bytes} B)"
                )));
            }
            std::fs::write(out_dir.join("frame-pin.json"), pin_pretty.as_bytes())
                .map_err(|e| make_err(format!("write gateway frame-pin.json: {e}")))?;

            // Copy all 8 STCH2 shards into both `/shards/` (portable/edge/IPFS) & `/stitch/v<ver>/shards/` (GitHub Pages):
            for fname in &pin.shard_files {
                let raw = std::fs::read(release_dir.join("shards").join(fname))
                    .map_err(|e| make_err(format!("read release shard {fname}: {e}")))?;
                let _ = crate::network::StitchRsShard::from_binary_stch2(&raw)
                    .map_err(|e| make_err(format!("verify release shard {fname}: {e}")))?;
                std::fs::write(shards_out.join(fname), &raw)
                    .map_err(|e| make_err(format!("write gateway shard {fname}: {e}")))?;
                std::fs::write(versioned_shards_out.join(fname), &raw)
                    .map_err(|e| make_err(format!("write versioned shard {fname}: {e}")))?;
            }

            // Copy verified `.holo` capsule:
            let holo_name = format!("verdant-v{}.holo", pin.release_version);
            let holo_bytes = std::fs::read(release_dir.join(&holo_name))
                .map_err(|e| make_err(format!("read {holo_name}: {e}")))?;
            if sha256_hex(&holo_bytes) != pin.release_kappa_hex {
                return Err(make_err(format!(
                    "{holo_name} SHA-256 does not match frame-pin releaseKappaHex"
                )));
            }
            std::fs::write(holo_out.join(&holo_name), &holo_bytes)
                .map_err(|e| make_err(format!("write gateway {holo_name}: {e}")))?;

            // Emit static redirect shims under `/part/<part>/index.html` so GitHub Pages, IPFS,
            // S3, Cloudflare, and Netlify all serve clean stakeholder part links natively:
            for part in &manifest.stakeholder_parts {
                let pdir = out_dir.join("part").join(&part.part_id);
                std::fs::create_dir_all(&pdir)
                    .map_err(|e| make_err(format!("create part dir {}: {e}", part.part_id)))?;
                let target = format!("../../?pin={}", part.stitch_uri);
                let html = format!(
                    "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta http-equiv=\"refresh\" content=\"0;url={target}\"><title>Verdant — {}</title></head><body><p>Mounting <a href=\"{target}\">{}</a> via Stitch Blank Frame…</p><script>location.replace({:?});</script></body></html>\n",
                    part.label, part.stitch_uri, target
                );
                std::fs::write(pdir.join("index.html"), html.as_bytes()).map_err(|e| {
                    make_err(format!("write part/{}/index.html: {e}", part.part_id))
                })?;
            }

            // Emit `gateway-manifest.json`:
            let manifest_pretty = format!(
                "{}\n",
                serde_json::to_string_pretty(&manifest)
                    .map_err(|e| make_err(format!("serialize gateway-manifest.json: {e}")))?
            );
            std::fs::write(
                out_dir.join("gateway-manifest.json"),
                manifest_pretty.as_bytes(),
            )
            .map_err(|e| make_err(format!("write gateway-manifest.json: {e}")))?;

            // Emit Cloudflare Pages / Workers & Netlify shared `_headers` + `_redirects` + `wrangler.toml` + `netlify.toml`:
            let max_age = gateway_immutable_cache_max_age_secs();
            let headers_txt = format!(
                "/*\n  Access-Control-Allow-Origin: *\n  Cross-Origin-Resource-Policy: cross-origin\n  X-Verdant-Gateway: stitch-v1\n\n/shards/*\n  Cache-Control: public, max-age={max_age}, immutable\n  Content-Type: application/octet-stream\n\n/stitch/*\n  Cache-Control: public, max-age={max_age}, immutable\n  Content-Type: application/octet-stream\n\n/frame-pin.json\n  Cache-Control: no-store\n\n/gateway-manifest.json\n  Cache-Control: no-store\n"
            );
            std::fs::write(out_dir.join("_headers"), headers_txt.as_bytes())
                .map_err(|e| make_err(format!("write _headers: {e}")))?;

            let redirects_txt = "/part/:part /?pin=stitch://verdant/:part 302\n/healthz /gateway-manifest.json 200\n";
            std::fs::write(out_dir.join("_redirects"), redirects_txt.as_bytes())
                .map_err(|e| make_err(format!("write _redirects: {e}")))?;

            let wrangler_txt = format!(
                "# Verdant Multi-Gateway Stitch Bundle — Cloudflare Pages / Workers Adapter\nname = \"verdant-stitch-gateway\"\ncompatibility_date = \"2026-01-01\"\npages_build_output_dir = \".\"\n\n[vars]\nVERDANT_RELEASE_VERSION = \"{}\"\nVERDANT_RELEASE_EPOCH = \"{}\"\nVERDANT_RELEASE_KAPPA = \"{}\"\n",
                manifest.release_version, manifest.release_epoch, manifest.release_kappa_hex
            );
            std::fs::write(out_dir.join("wrangler.toml"), wrangler_txt.as_bytes())
                .map_err(|e| make_err(format!("write wrangler.toml: {e}")))?;

            let netlify_txt = format!(
                "# Verdant Multi-Gateway Stitch Bundle — Netlify Edge Adapter\n[build]\n  publish = \".\"\n\n[[headers]]\n  for = \"/*\"\n  [headers.values]\n    Access-Control-Allow-Origin = \"*\"\n    Cross-Origin-Resource-Policy = \"cross-origin\"\n    X-Verdant-Gateway = \"stitch-v1\"\n\n[[headers]]\n  for = \"/shards/*\"\n  [headers.values]\n    Cache-Control = \"public, max-age={max_age}, immutable\"\n\n[[redirects]]\n  from = \"/part/:part\"\n  to = \"/?pin=stitch://verdant/:part\"\n  status = 302\n\n[[redirects]]\n  from = \"/healthz\"\n  to = \"/gateway-manifest.json\"\n  status = 200\n"
            );
            std::fs::write(out_dir.join("netlify.toml"), netlify_txt.as_bytes())
                .map_err(|e| make_err(format!("write netlify.toml: {e}")))?;

            // Emit IPFS / IPNS content-addressed root descriptor (100% relative paths):
            let ipfs_desc = serde_json::json!({
                "spec": "verdant/ipfs-gateway-root/1",
                "releaseVersion": manifest.release_version,
                "releaseEpoch": manifest.release_epoch,
                "releaseKappaHex": manifest.release_kappa_hex,
                "relativeFramePin": "./frame-pin.json",
                "relativeShardsDir": "./shards",
                "relativeManifest": "./gateway-manifest.json",
                "compatiblePublicGateways": [
                    "https://ipfs.io/ipfs/<CID>/",
                    "https://dweb.link/ipfs/<CID>/",
                    "https://<CID>.ipfs.w3s.link/"
                ]
            });
            std::fs::write(
                out_dir.join(".ipfs-gateway-root.json"),
                format!(
                    "{}\n",
                    serde_json::to_string_pretty(&ipfs_desc).unwrap_or_default()
                ),
            )
            .map_err(|e| make_err(format!("write .ipfs-gateway-root.json: {e}")))?;

            // Emit portable OCI / Docker static gateway Dockerfile:
            let dockerfile = "FROM nginx:1.27-alpine\nCOPY . /usr/share/nginx/html\nRUN printf 'server { listen 8080; root /usr/share/nginx/html; index index.html; add_header Access-Control-Allow-Origin * always; add_header Cross-Origin-Resource-Policy cross-origin always; add_header X-Verdant-Gateway stitch-v1 always; location = /healthz { try_files /gateway-manifest.json =404; } location / { try_files $uri $uri/ =404; } }\\n' > /etc/nginx/conf.d/default.conf\nEXPOSE 8080\n";
            std::fs::write(out_dir.join("Dockerfile"), dockerfile.as_bytes())
                .map_err(|e| make_err(format!("write gateway Dockerfile: {e}")))?;

            // Copy optional RFC 2789 Cargo Sparse Registry tree into `out_dir/crates`:
            if let Some(src_crates) = crates_dir {
                if src_crates.is_dir() {
                    let mut stack: Vec<(PathBuf, PathBuf)> =
                        vec![(src_crates.clone(), out_dir.join("crates"))];
                    while let Some((src, dst)) = stack.pop() {
                        std::fs::create_dir_all(&dst)
                            .map_err(|e| make_err(format!("create crates subdir: {e}")))?;
                        let rd = std::fs::read_dir(&src)
                            .map_err(|e| make_err(format!("read crates dir: {e}")))?;
                        for entry in rd.flatten() {
                            let ep = entry.path();
                            let target = dst.join(entry.file_name());
                            if ep.is_dir() {
                                stack.push((ep, target));
                            } else if ep.is_file() {
                                std::fs::copy(&ep, &target)
                                    .map_err(|e| make_err(format!("copy crate asset: {e}")))?;
                            }
                        }
                    }
                }
            }

            let outcome = GatewayOpOutcome::Packed(manifest);
            let pretty = serde_json::to_string_pretty(&outcome)
                .map_err(|e| make_err(format!("serialize gateway outcome: {e}")))?;
            Ok((outcome, pretty))
        }

        GatewayOp::Serve {
            bundle_dir,
            bind_addr,
            max_requests,
        } => {
            let root = bundle_dir
                .canonicalize()
                .map_err(|e| make_err(format!("resolve gateway bundle dir: {e}")))?;
            let listener = TcpListener::bind(bind_addr)
                .map_err(|e| make_err(format!("bind Stitch gateway on {bind_addr}: {e}")))?;
            let bound_port = listener
                .local_addr()
                .map_err(|e| make_err(format!("read bound port: {e}")))?
                .port();

            let read_timeout_secs = u64::from(two_u32());
            let buf_len = 1usize << (eight_u32() + four_u32());
            let max_age = gateway_immutable_cache_max_age_secs();

            let mut requests_served = 0usize;
            let mut shard_requests = 0usize;
            let mut part_redirects = 0usize;
            let mut healthz_checks = 0usize;
            let mut manifest_requests = 0usize;
            let mut not_found_count = 0usize;

            for stream_res in listener.incoming() {
                if let Some(limit) = max_requests {
                    if requests_served >= *limit {
                        break;
                    }
                }
                let Ok(mut stream) = stream_res else {
                    continue;
                };
                let _ = stream.set_read_timeout(Some(Duration::from_secs(read_timeout_secs)));
                let mut req_buf = vec![0u8; buf_len];
                let n = stream.read(&mut req_buf).unwrap_or(0);
                if n == 0 {
                    continue;
                }
                requests_served = requests_served.saturating_add(1);
                let req_str = String::from_utf8_lossy(&req_buf[..n]);
                let first_line = req_str.lines().next().unwrap_or("");
                let mut parts = first_line.split_whitespace();
                let method = parts.next().unwrap_or("GET");
                let raw_target = parts.next().unwrap_or("/");
                let url_path = raw_target.split('?').next().unwrap_or("/");

                let cors_hdr = "Access-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, HEAD, OPTIONS\r\nAccess-Control-Allow-Headers: *\r\nCross-Origin-Resource-Policy: cross-origin\r\nX-Verdant-Gateway: stitch-v1\r\n";

                if method.eq_ignore_ascii_case("OPTIONS") {
                    let resp = format!(
                        "HTTP/1.1 204 No Content\r\n{cors_hdr}Content-Length: 0\r\nConnection: close\r\n\r\n"
                    );
                    let _ = stream.write_all(resp.as_bytes());
                    if let Some(limit) = max_requests {
                        if requests_served >= *limit {
                            break;
                        }
                    }
                    continue;
                }

                let rel = url_path.trim_start_matches('/');
                if rel.contains("..") || rel.contains('\\') {
                    let resp = format!(
                        "HTTP/1.1 400 Bad Request\r\n{cors_hdr}Content-Type: text/plain; charset=utf-8\r\nContent-Length: 8\r\nConnection: close\r\n\r\nbad path"
                    );
                    let _ = stream.write_all(resp.as_bytes());
                    continue;
                }

                if rel == "healthz" {
                    healthz_checks = healthz_checks.saturating_add(1);
                    let manifest_opt: Option<MultiGatewayManifest> =
                        std::fs::read_to_string(root.join("gateway-manifest.json"))
                            .ok()
                            .and_then(|s| serde_json::from_str(&s).ok());
                    let mut avail_shards = 0u32;
                    if let Some(ref m) = manifest_opt {
                        for sf in &m.shard_files {
                            if root.join("shards").join(sf).is_file() {
                                avail_shards = avail_shards.saturating_add(1);
                            }
                        }
                    }
                    let (status_line, body_json) = match manifest_opt {
                        Some(m) if avail_shards >= u32::from(m.data_shards_k) => (
                            "HTTP/1.1 200 OK",
                            serde_json::json!({
                                "status": "ok",
                                "provider": "self-hosted-stitch-node",
                                "releaseVersion": m.release_version,
                                "releaseEpoch": m.release_epoch,
                                "releaseKappaHex": m.release_kappa_hex,
                                "availableShards": avail_shards,
                                "requiredShardsK": m.data_shards_k,
                                "totalShardsN": m.total_shards_n,
                                "stakeholderParts": m.stakeholder_parts.iter().map(|p| p.part_id.clone()).collect::<Vec<_>>(),
                            }),
                        ),
                        _ => (
                            "HTTP/1.1 503 Service Unavailable",
                            serde_json::json!({
                                "status": "degraded",
                                "availableShards": avail_shards,
                            }),
                        ),
                    };
                    let body = body_json.to_string().into_bytes();
                    let hdr = format!(
                        "{status_line}\r\n{cors_hdr}Content-Type: application/json; charset=utf-8\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    let _ = stream.write_all(hdr.as_bytes());
                    let _ = stream.write_all(&body);
                    if let Some(limit) = max_requests {
                        if requests_served >= *limit {
                            break;
                        }
                    }
                    continue;
                }

                if let Some(part_raw) = rel.strip_prefix("part/") {
                    let slug = part_raw.trim_end_matches('/');
                    if matches!(
                        slug,
                        "studio" | "catalogs" | "demo" | "docs" | "starter" | "card"
                    ) {
                        part_redirects = part_redirects.saturating_add(1);
                        let loc = format!("/?pin=stitch://verdant/{slug}");
                        let resp = format!(
                            "HTTP/1.1 302 Found\r\n{cors_hdr}Location: {loc}\r\nCache-Control: no-store\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                        );
                        let _ = stream.write_all(resp.as_bytes());
                        if let Some(limit) = max_requests {
                            if requests_served >= *limit {
                                break;
                            }
                        }
                        continue;
                    }
                }

                let mapped_rel = if rel.is_empty() || rel == "frame" || rel == "frame/" {
                    "index.html"
                } else if let Some(after_frame) = rel.strip_prefix("frame/") {
                    after_frame
                } else {
                    rel
                };
                let full_path = root.join(mapped_rel);
                match std::fs::read(&full_path) {
                    Ok(body) => {
                        if mapped_rel.ends_with(".stch") {
                            shard_requests = shard_requests.saturating_add(1);
                        } else if mapped_rel == "gateway-manifest.json" {
                            manifest_requests = manifest_requests.saturating_add(1);
                        }
                        let content_type = if mapped_rel.ends_with(".html") {
                            "text/html; charset=utf-8"
                        } else if mapped_rel.ends_with(".mjs") || mapped_rel.ends_with(".js") {
                            "text/javascript; charset=utf-8"
                        } else if mapped_rel.ends_with(".json") {
                            "application/json; charset=utf-8"
                        } else if mapped_rel.ends_with(".crate") {
                            "application/gzip"
                        } else {
                            "application/octet-stream"
                        };
                        let cache_hdr = if mapped_rel.ends_with(".stch")
                            || mapped_rel.ends_with(".holo")
                            || mapped_rel.ends_with(".crate")
                        {
                            format!("public, max-age={max_age}, immutable")
                        } else {
                            "no-store".to_string()
                        };
                        let hdr = format!(
                            "HTTP/1.1 200 OK\r\n{cors_hdr}Content-Type: {content_type}\r\nCache-Control: {cache_hdr}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                            body.len()
                        );
                        let _ = stream.write_all(hdr.as_bytes());
                        if !method.eq_ignore_ascii_case("HEAD") {
                            let _ = stream.write_all(&body);
                        }
                    }
                    Err(_) => {
                        not_found_count = not_found_count.saturating_add(1);
                        let resp = format!(
                            "HTTP/1.1 404 Not Found\r\n{cors_hdr}Content-Type: text/plain; charset=utf-8\r\nContent-Length: 9\r\nConnection: close\r\n\r\nnot found"
                        );
                        let _ = stream.write_all(resp.as_bytes());
                    }
                }
                if let Some(limit) = max_requests {
                    if requests_served >= *limit {
                        break;
                    }
                }
            }

            let outcome = GatewayOpOutcome::Served(GatewayServeReport {
                bind_addr: bind_addr.clone(),
                bound_port,
                requests_served,
                shard_requests,
                part_redirects,
                healthz_checks,
                manifest_requests,
                not_found_count,
            });
            let pretty = serde_json::to_string_pretty(&outcome)
                .map_err(|e| make_err(format!("serialize serve report: {e}")))?;
            Ok((outcome, pretty))
        }

        GatewayOp::VerifyFailover { bundle_dir } => {
            let root = bundle_dir
                .canonicalize()
                .map_err(|e| make_err(format!("resolve bundle dir for verify: {e}")))?;
            let required_adapters = [
                "_headers",
                "_redirects",
                "wrangler.toml",
                "netlify.toml",
                ".ipfs-gateway-root.json",
                "Dockerfile",
            ];
            let mut adapters_verified = Vec::new();
            for adapter in required_adapters {
                if !root.join(adapter).is_file() {
                    return Err(make_err(format!(
                        "multi-gateway bundle missing required adapter `{adapter}`"
                    )));
                }
                adapters_verified.push(adapter.to_string());
            }

            let pin_raw = std::fs::read_to_string(root.join("frame-pin.json"))
                .map_err(|e| make_err(format!("read bundle frame-pin.json: {e}")))?;
            let pin: FramePinManifest = serde_json::from_str(&pin_raw)
                .map_err(|e| make_err(format!("parse bundle frame-pin.json: {e}")))?;
            let manifest_raw = std::fs::read_to_string(root.join("gateway-manifest.json"))
                .map_err(|e| make_err(format!("read bundle gateway-manifest.json: {e}")))?;
            let manifest: MultiGatewayManifest = serde_json::from_str(&manifest_raw)
                .map_err(|e| make_err(format!("parse bundle gateway-manifest.json: {e}")))?;
            if !gateway_provider_families_independent(
                manifest.distinct_provider_families.len() as u32
            ) {
                return Err(make_err(
                    "gateway-manifest.json failed >= 2 independent provider families check"
                        .to_string(),
                ));
            }
            for part in &manifest.stakeholder_parts {
                if !root
                    .join("part")
                    .join(&part.part_id)
                    .join("index.html")
                    .is_file()
                {
                    return Err(make_err(format!(
                        "bundle missing static stakeholder part redirect `part/{}/index.html`",
                        part.part_id
                    )));
                }
            }

            // Prepare a shadow secondary gateway directory with 4 of 8 shards DELIBERATELY WITHHELD:
            let shadow_dir =
                std::env::temp_dir().join(format!("verdant-gw-verify-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&shadow_dir);
            std::fs::create_dir_all(shadow_dir.join("shards"))
                .map_err(|e| make_err(format!("mkdir shadow gateway shards: {e}")))?;
            for f in [
                "index.html",
                "frame-boot.mjs",
                "frame-sw.js",
                "frame-pin.json",
                "gateway-manifest.json",
            ] {
                std::fs::copy(root.join(f), shadow_dir.join(f))
                    .map_err(|e| make_err(format!("copy shadow {f}: {e}")))?;
            }
            let stride = two_u32() as usize;
            for (idx, fname) in pin.shard_files.iter().enumerate() {
                if idx % stride == 1 {
                    std::fs::copy(
                        root.join("shards").join(fname),
                        shadow_dir.join("shards").join(fname),
                    )
                    .map_err(|e| make_err(format!("copy shadow shard {fname}: {e}")))?;
                }
            }

            // Gateway A (simulated GitHub primary outage -> HTTP 503):
            let primary_listener = TcpListener::bind("127.0.0.1:0")
                .map_err(|e| make_err(format!("bind simulated primary outage server: {e}")))?;
            let primary_port = primary_listener
                .local_addr()
                .map_err(|e| make_err(format!("read primary port: {e}")))?
                .port();
            let primary_stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let primary_stop_srv = std::sync::Arc::clone(&primary_stop);
            let primary_reqs = pin.shard_files.len();
            let primary_thread = std::thread::spawn(move || {
                let mut handled = 0usize;
                for stream_res in primary_listener.incoming() {
                    if primary_stop_srv.load(std::sync::atomic::Ordering::SeqCst) {
                        break;
                    }
                    let Ok(mut sock) = stream_res else {
                        continue;
                    };
                    let mut buf = [0u8; 1 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1)];
                    let n = sock.read(&mut buf).unwrap_or(0);
                    if n == 0 {
                        continue;
                    }
                    let _ = sock.write_all(
                        b"HTTP/1.1 503 Service Unavailable\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: 14\r\nConnection: close\r\n\r\nprimary outage",
                    );
                    handled = handled.saturating_add(1);
                    if handled >= primary_reqs {
                        break;
                    }
                }
            });

            // Gateway B (real Verdant native Stitch Gateway on loopback):
            let sec_probe_listener = TcpListener::bind("127.0.0.1:0")
                .map_err(|e| make_err(format!("allocate secondary gateway port: {e}")))?;
            let sec_port = sec_probe_listener
                .local_addr()
                .map_err(|e| make_err(format!("read secondary port: {e}")))?
                .port();
            drop(sec_probe_listener);

            let sec_expected_reqs = pin.shard_files.len() + (four_u32() as usize);
            let sec_dir = shadow_dir.clone();
            let sec_addr = format!("127.0.0.1:{sec_port}");
            let sec_thread = std::thread::spawn(move || {
                execute_gateway_op(&GatewayOp::Serve {
                    bundle_dir: sec_dir,
                    bind_addr: sec_addr,
                    max_requests: Some(sec_expected_reqs),
                })
            });

            // Wait briefly for secondary listener to bind:
            let retry_cap = eight_u32() + eight_u32();
            let mut connected = false;
            for _ in 0..retry_cap {
                if TcpStream::connect(format!("127.0.0.1:{sec_port}")).is_ok() {
                    // Opening and closing sends 0 bytes, which `Serve` skips cleanly without counting!
                    connected = true;
                    break;
                }
                std::thread::sleep(Duration::from_millis(u64::from(eight_u32() + eight_u32())));
            }
            if !connected {
                let _ = std::fs::remove_dir_all(&shadow_dir);
                return Err(make_err(
                    "secondary Stitch gateway did not bind in time".to_string(),
                ));
            }

            let http_req = |port: u16,
                            method: &str,
                            path: &str|
             -> Result<(u16, String, Vec<u8>), ObserverModelError> {
                let mut sock = TcpStream::connect(format!("127.0.0.1:{port}"))
                    .map_err(|e| make_err(format!("connect 127.0.0.1:{port}{path}: {e}")))?;
                let req = format!(
                    "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n"
                );
                sock.write_all(req.as_bytes())
                    .map_err(|e| make_err(format!("write HTTP request {path}: {e}")))?;
                let mut raw = Vec::new();
                sock.read_to_end(&mut raw)
                    .map_err(|e| make_err(format!("read HTTP response {path}: {e}")))?;
                let sep = b"\r\n\r\n";
                let split_at = raw
                    .windows(sep.len())
                    .position(|w| w == sep)
                    .ok_or_else(|| make_err(format!("malformed HTTP response for {path}")))?;
                let hdr = String::from_utf8_lossy(&raw[..split_at]).to_string();
                let body = raw[split_at + sep.len()..].to_vec();
                let code = hdr
                    .lines()
                    .next()
                    .and_then(|l| l.split_whitespace().nth(1))
                    .and_then(|s| s.parse::<u16>().ok())
                    .unwrap_or(0);
                Ok((code, hdr, body))
            };

            let verify_res = (|| -> Result<GatewayFailoverReport, ObserverModelError> {
                let (health_code, health_hdr, health_body) = http_req(sec_port, "GET", "/healthz")?;
                if !gateway_http_status_ok(health_code)
                    || !health_hdr
                        .to_ascii_lowercase()
                        .contains("access-control-allow-origin: *")
                {
                    return Err(make_err(format!(
                        "secondary gateway /healthz failed: HTTP {health_code}"
                    )));
                }
                let health_json: serde_json::Value = serde_json::from_slice(&health_body)
                    .map_err(|e| make_err(format!("parse /healthz JSON: {e}")))?;
                if health_json.get("status").and_then(|v| v.as_str()) != Some("ok") {
                    return Err(make_err(format!(
                        "secondary gateway /healthz not ok: {health_json}"
                    )));
                }

                let (man_code, _, man_body) = http_req(sec_port, "GET", "/gateway-manifest.json")?;
                if !gateway_http_status_ok(man_code) || man_body.is_empty() {
                    return Err(make_err(format!(
                        "secondary gateway /gateway-manifest.json failed: HTTP {man_code}"
                    )));
                }

                let (part_code, part_hdr, _) = http_req(sec_port, "GET", "/part/studio")?;
                if !gateway_http_status_redirect(part_code)
                    || !part_hdr.contains("Location: /?pin=stitch://verdant/studio")
                {
                    return Err(make_err(format!(
                        "secondary gateway /part/studio redirect failed: HTTP {part_code}"
                    )));
                }

                let (opt_code, opt_hdr, _) = http_req(sec_port, "OPTIONS", "/shards/shard-1.stch")?;
                if !gateway_http_status_no_content(opt_code)
                    || !opt_hdr
                        .to_ascii_lowercase()
                        .contains("access-control-allow-origin: *")
                {
                    return Err(make_err(format!(
                        "secondary gateway CORS preflight failed: HTTP {opt_code}"
                    )));
                }

                let mut primary_outage_status = 0u16;
                let mut surviving = Vec::new();
                let mut withheld_shards_tolerated = 0u32;
                for fname in &pin.shard_files {
                    let shard_url = format!("/shards/{fname}");
                    let (p_code, _, _) = http_req(primary_port, "GET", &shard_url)?;
                    if !gateway_http_status_unavailable(p_code) {
                        return Err(make_err(format!(
                            "expected primary simulated outage 503, got {p_code}"
                        )));
                    }
                    primary_outage_status = p_code;

                    // Failover to independent secondary Stitch gateway:
                    let (s_code, s_hdr, s_body) = http_req(sec_port, "GET", &shard_url)?;
                    if gateway_http_status_ok(s_code) {
                        if !s_hdr
                            .to_ascii_lowercase()
                            .contains("access-control-allow-origin: *")
                        {
                            return Err(make_err(format!(
                                "secondary shard {fname} missing CORS wildcard header"
                            )));
                        }
                        let parsed = crate::network::StitchRsShard::from_binary_stch2(&s_body)
                            .map_err(|e| {
                                make_err(format!("verify secondary shard {fname}: {e}"))
                            })?;
                        surviving.push(parsed);
                    } else {
                        withheld_shards_tolerated = withheld_shards_tolerated.saturating_add(1);
                    }
                }

                let mut trusted = BTreeMap::new();
                trusted.insert(
                    "prism:verdant-observer-executive".to_string(),
                    b"verdant-observer-executive-quorum-key".to_vec(),
                );
                trusted.insert(
                    "prism:verdant-release-bot".to_string(),
                    b"verdant-release-bot-quorum-key".to_vec(),
                );
                let recovered = crate::vcs::VerdantReleaseCapsule::recover_from_stitch(
                    &surviving,
                    &pin,
                    0,
                    Some(pin.prev_kappa_hex.as_str()),
                    &trusted,
                )
                .map_err(|e| make_err(format!("failover 4-of-8 capsule recovery failed: {e}")))?;

                Ok(GatewayFailoverReport {
                    spec: "verdant/gateway-failover-report/1".to_string(),
                    release_version: recovered.manifest.version,
                    release_epoch: recovered.manifest.release_epoch,
                    release_kappa_hex: recovered.capsule_sha256_hex,
                    primary_outage_status,
                    secondary_healthz_status: health_code,
                    part_redirect_status: part_code,
                    cors_preflight_status: opt_code,
                    surviving_shards_fetched: surviving.len() as u32,
                    withheld_shards_tolerated,
                    recovered_files_count: recovered.manifest.files.len(),
                    adapters_verified,
                    distinct_provider_families: manifest.distinct_provider_families,
                    all_passed: true,
                })
            })();

            primary_stop.store(true, std::sync::atomic::Ordering::SeqCst);
            let _ = TcpStream::connect(format!("127.0.0.1:{primary_port}"));
            let _ = primary_thread.join();
            if verify_res.is_ok() {
                let _ = sec_thread.join();
            } else {
                for _ in 0..sec_expected_reqs {
                    if let Ok(mut s) = TcpStream::connect(format!("127.0.0.1:{sec_port}")) {
                        let _ = s.write_all(
                            b"OPTIONS / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
                        );
                    } else {
                        break;
                    }
                }
                let _ = sec_thread.join();
            }
            let _ = std::fs::remove_dir_all(&shadow_dir);
            let outcome = GatewayOpOutcome::Verified(verify_res?);
            let pretty = serde_json::to_string_pretty(&outcome)
                .map_err(|e| make_err(format!("serialize failover report: {e}")))?;
            Ok((outcome, pretty))
        }
    }
}
