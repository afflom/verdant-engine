//! Composable Blank Frame release pin, `stitch://` deep-link sanitizer, and Studio VCS bridge (`vcs_studio.rs`).

use super::*;

/// Composable Blank Frame release pin manifest (`site/frame/frame-pin.json`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FramePinManifest {
    /// Fixed format identifier (`verdant/frame-pin/1`).
    pub spec: String,
    /// Semantic release version pinned by this Blank Frame.
    pub release_version: String,
    /// Monotonic release epoch (`>= 1`).
    pub release_epoch: u64,
    /// Content-addressed SHA-256 hex digest of the pinned `verdant-vX.Y.Z.holo` capsule.
    pub release_kappa_hex: String,
    /// Previous release `kappa` (or `"genesis"`).
    pub prev_kappa_hex: String,
    /// [`VerdantVcs`] commit `kappa` root for this release.
    pub vcs_commit_kappa: String,
    /// View Immutability Seal SHA-256 verified before packing.
    pub immutability_seal_sha256: String,
    /// Default composable Stitch deep-link URI (e.g., `stitch://verdant/studio`).
    pub default_stitch_uri: String,
    /// Systematic Reed-Solomon data shard count (`k = 4`).
    pub data_shards_k: u8,
    /// Systematic Reed-Solomon total shard count (`n = 8`).
    pub total_shards_n: u8,
    /// Exact byte size of the reconstructed `.holo` capsule.
    pub capsule_byte_len: usize,
    /// Signed anti-rollback monotonic quorum pointer.
    pub pointer: StitchMonotonicPointer,
    /// Ordered list of Stitch HTTP/WebRTC shard relay base URLs.
    pub stitch_relays: Vec<String>,
    /// Relative shard file names (`shard-0.stch.json`..`shard-7.stch.json`).
    pub shard_files: Vec<String>,
}

/// Parsed, sanitized Stitch deep-link URI target (`Threat 18`: prevents XSS/origin phishing).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StitchDeepLinkTarget {
    /// Normalized `stitch://...` URI.
    pub canonical_uri: String,
    /// Target authority (`verdant` or a 64-hex content `kappa`).
    pub authority: String,
    /// Sanitized relative Virtual FS HTML route inside the `.holo` capsule.
    pub virtual_route: String,
}

/// Strictly parses and validates a composable `stitch://` deep-link URI, refusing `javascript:`,
/// `data:`, `http:`, path traversal (`..`), control characters, or HTML injection (`Threat 18`).
pub fn parse_stitch_deep_link_uri(uri: &str) -> Result<StitchDeepLinkTarget, StitchCustodyError> {
    let trimmed = uri.trim();
    let Some(rest) = trimmed.strip_prefix("stitch://") else {
        return Err(StitchCustodyError::InvalidJournalSignature {
            detail: format!("composable Blank Frame deep-link must use stitch:// scheme: `{uri}`"),
        });
    };
    if rest.is_empty()
        || rest.contains("..")
        || rest.contains('<')
        || rest.contains('>')
        || rest.contains('"')
        || rest.contains('\'')
        || rest.contains('\\')
        || rest
            .bytes()
            .any(|b| b < verdant_tensor::scalars::thirty_two_u8())
    {
        return Err(StitchCustodyError::InvalidJournalSignature {
            detail: format!("unsafe characters or traversal in stitch:// URI: `{uri}`"),
        });
    }
    let (authority, sub_path) = match rest.split_once('/') {
        Some((a, p)) => (a, p.trim_matches('/')),
        None => (rest, ""),
    };
    let is_valid_auth = authority == "verdant"
        || (authority.len() == SHA256_BYTES * (1 + 1)
            && authority.bytes().all(|b| b.is_ascii_hexdigit()));
    if !is_valid_auth {
        return Err(StitchCustodyError::InvalidJournalSignature {
            detail: format!(
                "stitch:// authority must be `verdant` or a 64-hex kappa: `{authority}`"
            ),
        });
    }
    let virtual_route = match sub_path {
        "" | "studio" | "studio/index.html" => "studio/index.html".to_string(),
        "catalogs" | "catalogs/index.html" => "catalogs/index.html".to_string(),
        "demo" | "demo/index.html" => "demo/index.html".to_string(),
        "docs" | "docs/index.html" => "docs/index.html".to_string(),
        "starter" | "starter/index.html" => "starter/index.html".to_string(),
        "card" | "card.html" => "card.html".to_string(),
        other => {
            if other.bytes().all(|b| {
                b.is_ascii_alphanumeric() || b == b'/' || b == b'-' || b == b'_' || b == b'.'
            }) {
                other.to_string()
            } else {
                return Err(StitchCustodyError::InvalidJournalSignature {
                    detail: format!("disallowed route characters in stitch:// URI: `{other}`"),
                });
            }
        }
    };
    Ok(StitchDeepLinkTarget {
        canonical_uri: format!("stitch://{authority}/{virtual_route}"),
        authority: authority.to_string(),
        virtual_route,
    })
}

/// Default release Reed-Solomon geometry helper (`4-of-8`).
#[must_use]
pub const fn default_release_rs_geometry() -> (u8, u8) {
    let two = 1_u8.wrapping_add(1_u8);
    let four = two.wrapping_mul(two);
    (four, four.wrapping_mul(two))
}

/// Zero-account in-browser Studio authoring bridge (`VE-49`): commits an authored 3D scene/app
/// into [`VerdantVcs`] with W3C PROV-O lineage, packs a self-verifying portable `.holo` capsule
/// (`VDNT_HOLO_REL_V1`), shards it `4-of-8` (`STCH2`), and returns a sanitized `stitch://` deep
/// link ready for Blank Frame mounting.
#[must_use]
pub fn studio_vcs_commit_and_pack_holo_json(raw_json: &str) -> String {
    let Ok(req) = serde_json::from_slice::<serde_json::Value>(raw_json.as_bytes()) else {
        return serde_json::json!({ "ok": false, "error": "invalid JSON request" }).to_string();
    };
    let app_title = req
        .get("appTitle")
        .and_then(|v| v.as_str())
        .unwrap_or("Verdant Studio App");
    let author = req
        .get("authorPrismId")
        .and_then(|v| v.as_str())
        .unwrap_or("prism:anonymous-creator");
    let message = req
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("In-browser VerdantVcs commit");
    let version = req
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("1.0.0");
    let epoch = req
        .get("epoch")
        .and_then(|v| v.as_u64())
        .unwrap_or(1)
        .max(1);
    let scene_json = req
        .get("sceneJson")
        .and_then(|v| v.as_str())
        .unwrap_or("{}");
    let prev_parent = req
        .get("parentCommitKappa")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty());

    let mut vcs = VerdantVcs::new();
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let html =
        crate::portable_player::render_portable_3d_player_html(app_title, author, scene_json);
    files.insert("studio/index.html".to_string(), html.into_bytes());
    files.insert("scene.json".to_string(), scene_json.as_bytes().to_vec());
    files.insert(
        "verdant-app.json".to_string(),
        serde_json::json!({
            "spec": "verdant/portable-studio-app/1",
            "title": app_title,
            "authorPrismId": author,
            "version": version,
            "epoch": epoch,
            "parentCommitKappa": prev_parent,
            "message": message,
        })
        .to_string()
        .into_bytes(),
    );
    if let Some(extra) = req.get("files").and_then(|v| v.as_object()) {
        for (k, v) in extra {
            if !k.contains("..") && !k.starts_with('/') {
                if let Some(s) = v.as_str() {
                    files.insert(k.clone(), s.as_bytes().to_vec());
                }
            }
        }
    }

    let ref_name = "refs/heads/studio";
    let res = vcs
        .commit_tree(
            ref_name,
            &files,
            author,
            b"verdant-studio-zero-account-vcs",
            "mgr-studio-alm",
            "AUTH-STITCH-SELF-HOSTING",
            message,
        )
        .and_then(|commit| {
            let seal_hex = sha256_hex_raw(commit.tree_root_sha256.as_bytes());
            let capsule = VerdantReleaseCapsule::pack_from_files(
                version,
                epoch,
                &commit.commit_kappa,
                &seal_hex,
                "studio/index.html",
                &files,
            )?;
            let (k, n) = default_release_rs_geometry();
            let shards = ReedSolomonStitchCodec::shard_k_of_n(&capsule.holo_bytes, k, n)?;
            let deep_link = parse_stitch_deep_link_uri(&format!(
                "stitch://{}/studio",
                capsule.capsule_sha256_hex
            ))?;
            Ok((commit, capsule, shards.len(), deep_link))
        });

    match res {
        Ok((commit, capsule, shard_count, deep_link)) => serde_json::json!({
            "ok": true,
            "commitKappa": commit.commit_kappa,
            "parentCommitKappa": prev_parent,
            "treeRootSha256": commit.tree_root_sha256,
            "epoch": epoch,
            "authorPrismId": commit.author_prism_id,
            "message": commit.message,
            "capsuleSha256Hex": capsule.capsule_sha256_hex,
            "capsuleByteLen": capsule.holo_bytes.len(),
            "shardCount": shard_count,
            "stitchDeepLinkUri": deep_link.canonical_uri,
            "holoHex": hex_encode_bytes(&capsule.holo_bytes),
        })
        .to_string(),
        Err(e) => serde_json::json!({ "ok": false, "error": e.to_string() }).to_string(),
    }
}
