//! **In-Verdant Content-Addressed Merkle Source & Revision Control (`vcs.rs`)** — W3C PROV-O
//! revision DAG hosted directly inside Verdant and serialized onto the Stitch network as
//! `.uorc`-sealed capsules (`AUTH-STITCH-SELF-HOSTING`, `AUTH-W3C-SOSA-PROVO`).
//!
//! Enables Verdant Studio and `verdant mcp` to version, branch, audit, and release every lifted
//! Viewpoint, design token, Wasm artifact, and source module inside Verdant itself without
//! requiring external git hosting for stakeholders.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::StitchCustodyError;
use crate::network::{
    ReedSolomonStitchCodec, StitchMonotonicPointer, StitchRsShard, RELEASE_RS_DATA_SHARDS,
    RELEASE_RS_TOTAL_SHARDS, SHA256_BYTES,
};

fn sha256_hex_raw(bytes: &[u8]) -> String {
    const HEX: &[u8; 1 << (1 + 1 + 1 + 1)] = b"0123456789abcdef";
    let shift = 1usize + 1usize + 1usize + 1usize;
    let mask = (1usize << shift) - 1usize;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut out = String::with_capacity(SHA256_BYTES * (1 + 1));
    for b in digest {
        let v = usize::from(b);
        out.push(HEX[v >> shift] as char);
        out.push(HEX[v & mask] as char);
    }
    out
}

fn sha256_hex_tagged(tag: &str, bytes: &[u8]) -> String {
    let mut framed = Vec::with_capacity(tag.len() + 1 + bytes.len());
    framed.extend_from_slice(tag.as_bytes());
    framed.push(b':');
    framed.extend_from_slice(bytes);
    sha256_hex_raw(&framed)
}

/// Content-addressed blob stored in [`VerdantVcs`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerdantVcsBlob {
    /// Content-addressed SHA-256 hex digest of `plain_bytes`.
    pub blob_sha256: String,
    /// Original byte length.
    pub byte_len: usize,
    /// Canonical blob payload bytes (or `.uorc`-sealed envelope bytes).
    pub payload: Vec<u8>,
}

/// Signed, W3C PROV-O-attributed Merkle commit inside [`VerdantVcs`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerdantVcsCommit {
    /// Content-addressed commit `kappa` hex identity.
    pub commit_kappa: String,
    /// Parent commit `kappa`s (`empty` for genesis; acyclic by construction).
    pub parent_kappas: Vec<String>,
    /// Strictly monotonic commit epoch (`parent_max_epoch + 1`).
    pub epoch: u64,
    /// Deterministic SHA-256 root over sorted `(path, blob_sha256)` entries.
    pub tree_root_sha256: String,
    /// Map of canonical workspace-relative paths to blob SHA-256 digests.
    pub tree_entries: BTreeMap<String, String>,
    /// Sovereign `Prism ID` (`prism:...`) of the committing stakeholder or operator.
    pub author_prism_id: String,
    /// Accountable VSM Manager ID authorizing this revision.
    pub manager_id: String,
    /// Cited authoritative standard (`AUTH-*`) justifying this change.
    pub cited_authority: String,
    /// Human-readable commit summary.
    pub message: String,
    /// Deterministic signature commitment over `(commit_kappa || author_prism_id || epoch)`.
    pub signature_hex: String,
}

/// In-Verdant content-addressed Merkle revision store (`VerdantVcs`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerdantVcs {
    /// Content-addressed blobs keyed by `blob_sha256`.
    pub blobs: BTreeMap<String, VerdantVcsBlob>,
    /// Verified commits keyed by `commit_kappa`.
    pub commits: BTreeMap<String, VerdantVcsCommit>,
    /// Named mutable refs (e.g. `refs/verdant/heads/main`, `refs/verdant/releases/stable`).
    pub refs: BTreeMap<String, String>,
}

impl VerdantVcs {
    /// Creates an empty content-addressed [`VerdantVcs`] repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes the deterministic SHA-256 Merkle tree root for `tree_entries`.
    #[must_use]
    pub fn compute_tree_root(tree_entries: &BTreeMap<String, String>) -> String {
        let mut buf = Vec::new();
        for (path, digest) in tree_entries {
            buf.extend_from_slice(path.as_bytes());
            buf.push(b'=');
            buf.extend_from_slice(digest.as_bytes());
            buf.push(b'\n');
        }
        sha256_hex_tagged("verdant-vcs-tree", &buf)
    }

    /// Stores a content-addressed blob and returns its `blob_sha256` hex digest.
    pub fn put_blob(&mut self, payload: &[u8]) -> String {
        let digest = sha256_hex_raw(payload);
        self.blobs
            .entry(digest.clone())
            .or_insert_with(|| VerdantVcsBlob {
                blob_sha256: digest.clone(),
                byte_len: payload.len(),
                payload: payload.to_vec(),
            });
        digest
    }

    /// Commits a complete file tree onto `ref_name`, enforcing sovereign `Prism ID` attribution,
    /// locked `AUTH-*` citation, acyclic parent lineage, and monotonic epoch advancement.
    #[allow(clippy::too_many_arguments)]
    pub fn commit_tree(
        &mut self,
        ref_name: &str,
        files: &BTreeMap<String, Vec<u8>>,
        author_prism_id: &str,
        device_secret: &[u8],
        manager_id: &str,
        cited_authority: &str,
        message: &str,
    ) -> Result<VerdantVcsCommit, StitchCustodyError> {
        if !author_prism_id.starts_with("prism:") || device_secret.is_empty() {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "VerdantVcs commit requires a prism: author identity and device secret"
                    .to_string(),
            });
        }
        if !cited_authority.starts_with("AUTH-") || manager_id.trim().is_empty() {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "VerdantVcs commit requires accountable manager_id and AUTH-* citation"
                    .to_string(),
            });
        }
        if files.is_empty() {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "cannot commit empty VerdantVcs tree".to_string(),
            });
        }

        let mut tree_entries = BTreeMap::new();
        for (path, bytes) in files {
            let clean = path.trim();
            if clean.is_empty() || clean.contains("..") || clean.starts_with('/') {
                return Err(StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("invalid VerdantVcs relative path `{path}`"),
                });
            }
            let blob_sha = self.put_blob(bytes);
            tree_entries.insert(clean.to_string(), blob_sha);
        }
        let tree_root_sha256 = Self::compute_tree_root(&tree_entries);

        let (parent_kappas, epoch) = if let Some(prev_kappa) = self.refs.get(ref_name) {
            let prev = self.commits.get(prev_kappa).ok_or_else(|| {
                StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("dangling VerdantVcs ref `{ref_name}` -> `{prev_kappa}`"),
                }
            })?;
            (
                vec![prev.commit_kappa.clone()],
                prev.epoch.saturating_add(1),
            )
        } else {
            (Vec::new(), 1)
        };

        let mut commit_preimage = Vec::new();
        commit_preimage.extend_from_slice(tree_root_sha256.as_bytes());
        commit_preimage.push(b'|');
        for p in &parent_kappas {
            commit_preimage.extend_from_slice(p.as_bytes());
            commit_preimage.push(b',');
        }
        commit_preimage.extend_from_slice(&epoch.to_le_bytes());
        commit_preimage.extend_from_slice(author_prism_id.as_bytes());
        commit_preimage.extend_from_slice(manager_id.as_bytes());
        commit_preimage.extend_from_slice(cited_authority.as_bytes());
        commit_preimage.extend_from_slice(message.as_bytes());
        let commit_kappa = sha256_hex_tagged("verdant-vcs-commit", &commit_preimage);

        let mut sig_msg = Vec::new();
        sig_msg.extend_from_slice(commit_kappa.as_bytes());
        sig_msg.push(b':');
        sig_msg.extend_from_slice(author_prism_id.as_bytes());
        sig_msg.push(b':');
        sig_msg.extend_from_slice(&epoch.to_le_bytes());
        sig_msg.push(b':');
        sig_msg.extend_from_slice(device_secret);
        let signature_hex = sha256_hex_tagged("verdant-vcs-sig", &sig_msg);

        let commit = VerdantVcsCommit {
            commit_kappa: commit_kappa.clone(),
            parent_kappas,
            epoch,
            tree_root_sha256,
            tree_entries,
            author_prism_id: author_prism_id.to_string(),
            manager_id: manager_id.to_string(),
            cited_authority: cited_authority.to_string(),
            message: message.to_string(),
            signature_hex,
        };
        self.commits.insert(commit_kappa.clone(), commit.clone());
        self.refs.insert(ref_name.to_string(), commit_kappa);
        Ok(commit)
    }

    /// Checks out and SHA-256-verifies every blob in `commit_kappa`.
    pub fn checkout_commit(
        &self,
        commit_kappa: &str,
    ) -> Result<BTreeMap<String, Vec<u8>>, StitchCustodyError> {
        let commit = self.commits.get(commit_kappa).ok_or_else(|| {
            StitchCustodyError::PayloadDigestMismatch {
                detail: format!("unknown VerdantVcs commit `{commit_kappa}`"),
            }
        })?;
        if Self::compute_tree_root(&commit.tree_entries) != commit.tree_root_sha256 {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: format!("VerdantVcs tree root mismatch on `{commit_kappa}`"),
            });
        }
        let mut out = BTreeMap::new();
        for (path, digest) in &commit.tree_entries {
            let blob = self.blobs.get(digest).ok_or_else(|| {
                StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("missing blob `{digest}` for `{path}`"),
                }
            })?;
            if sha256_hex_raw(&blob.payload) != *digest {
                return Err(StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("corrupted blob digest for `{path}`"),
                });
            }
            out.insert(path.clone(), blob.payload.clone());
        }
        Ok(out)
    }

    /// Walks the acyclic W3C PROV-O commit ancestry from `head_kappa` back to genesis, refusing cycles.
    pub fn verify_acyclic_history(
        &self,
        head_kappa: &str,
    ) -> Result<Vec<String>, StitchCustodyError> {
        let mut visited = BTreeSet::new();
        let mut order = Vec::new();
        let mut cursor = Some(head_kappa.to_string());
        while let Some(curr) = cursor {
            if !visited.insert(curr.clone()) {
                return Err(StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("cyclic VerdantVcs lineage detected at `{curr}`"),
                });
            }
            let commit = self.commits.get(&curr).ok_or_else(|| {
                StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("missing VerdantVcs ancestor `{curr}`"),
                }
            })?;
            order.push(curr);
            cursor = commit.parent_kappas.first().cloned();
        }
        Ok(order)
    }

    /// Serializes the entire [`VerdantVcs`] repository into a compact zero-JSON-bloat binary
    /// frame wrapped in a SHA-256-sealed `.uorc` capsule.
    #[cfg(feature = "uorc-streaming")]
    pub fn encode_uorc_capsule(&self) -> Result<Vec<u8>, StitchCustodyError> {
        let blob_index: Vec<(&str, usize)> = self
            .blobs
            .iter()
            .map(|(k, b)| (k.as_str(), b.payload.len()))
            .collect();
        let meta = serde_json::json!({
            "commits": self.commits,
            "refs": self.refs,
            "blobIndex": blob_index,
        });
        let meta_bytes =
            serde_json::to_vec(&meta).map_err(|e| StitchCustodyError::PayloadDigestMismatch {
                detail: format!("serialize VerdantVcs metadata: {e}"),
            })?;
        let meta_len = u32::try_from(meta_bytes.len()).map_err(|_| {
            StitchCustodyError::PayloadDigestMismatch {
                detail: "VerdantVcs metadata exceeds u32".to_string(),
            }
        })?;
        let total_blob_bytes: usize = self.blobs.values().map(|b| b.payload.len()).sum();
        let magic: &[u8; 1 << (1 + 1 + 1 + 1)] = b"VDNT_VCS_BIN_V1\0";
        let u32_len = 1usize + 1usize + 1usize + 1usize;
        let mut framed =
            Vec::with_capacity(magic.len() + u32_len + meta_bytes.len() + total_blob_bytes);
        framed.extend_from_slice(magic);
        framed.extend_from_slice(&meta_len.to_le_bytes());
        framed.extend_from_slice(&meta_bytes);
        for blob in self.blobs.values() {
            framed.extend_from_slice(&blob.payload);
        }
        Ok(crate::streaming::compress_uorc_portable(&framed))
    }

    /// Restores and verifies a [`VerdantVcs`] repository from a `.uorc` capsule.
    #[cfg(feature = "uorc-wasm-decode")]
    pub fn decode_uorc_capsule(uorc_bytes: &[u8]) -> Result<Self, StitchCustodyError> {
        let raw = crate::streaming::decompress_uorc_portable(uorc_bytes).map_err(|e| {
            StitchCustodyError::PayloadDigestMismatch {
                detail: format!("decode VerdantVcs .uorc capsule: {e}"),
            }
        })?;
        let magic: &[u8; 1 << (1 + 1 + 1 + 1)] = b"VDNT_VCS_BIN_V1\0";
        let u32_len = 1usize + 1usize + 1usize + 1usize;
        let header_len = magic.len() + u32_len;
        let vcs = if raw.len() >= header_len && &raw[..magic.len()] == magic {
            let mut len_buf = [0u8; 1 + 1 + 1 + 1];
            len_buf.copy_from_slice(&raw[magic.len()..header_len]);
            let meta_len = u32::from_le_bytes(len_buf) as usize;
            let body_start = header_len.saturating_add(meta_len);
            if body_start > raw.len() {
                return Err(StitchCustodyError::PayloadDigestMismatch {
                    detail: "truncated VerdantVcs binary metadata".to_string(),
                });
            }
            let meta: serde_json::Value = serde_json::from_slice(&raw[header_len..body_start])
                .map_err(|e| StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("parse VerdantVcs metadata JSON: {e}"),
                })?;
            let commits: BTreeMap<String, VerdantVcsCommit> = serde_json::from_value(
                meta.get("commits").cloned().unwrap_or_default(),
            )
            .map_err(|e| StitchCustodyError::PayloadDigestMismatch {
                detail: format!("parse VerdantVcs commits: {e}"),
            })?;
            let refs: BTreeMap<String, String> = serde_json::from_value(
                meta.get("refs").cloned().unwrap_or_default(),
            )
            .map_err(|e| StitchCustodyError::PayloadDigestMismatch {
                detail: format!("parse VerdantVcs refs: {e}"),
            })?;
            let blob_index: Vec<(String, usize)> =
                serde_json::from_value(meta.get("blobIndex").cloned().unwrap_or_default())
                    .map_err(|e| StitchCustodyError::PayloadDigestMismatch {
                        detail: format!("parse VerdantVcs blobIndex: {e}"),
                    })?;
            let mut blobs = BTreeMap::new();
            let mut cursor = body_start;
            for (digest, byte_len) in blob_index {
                let end = cursor.saturating_add(byte_len);
                if end > raw.len() {
                    return Err(StitchCustodyError::PayloadDigestMismatch {
                        detail: format!("truncated VerdantVcs blob `{digest}`"),
                    });
                }
                let payload = raw[cursor..end].to_vec();
                cursor = end;
                blobs.insert(
                    digest.clone(),
                    VerdantVcsBlob {
                        blob_sha256: digest,
                        byte_len,
                        payload,
                    },
                );
            }
            Self {
                blobs,
                commits,
                refs,
            }
        } else {
            serde_json::from_slice(&raw).map_err(|e| StitchCustodyError::PayloadDigestMismatch {
                detail: format!("parse VerdantVcs JSON: {e}"),
            })?
        };
        for head in vcs.refs.values() {
            let _ = vcs.verify_acyclic_history(head)?;
            let _ = vcs.checkout_commit(head)?;
        }
        Ok(vcs)
    }
}

/// Canonical 16-byte binary header for self-verifying portable Verdant Release `.holo` capsules.
pub const RELEASE_CAPSULE_MAGIC: &[u8; 1 << (1 + 1 + 1 + 1)] = b"VDNT_HOLO_REL_V1";

/// Maximum allowed Blank Frame total byte footprint (`30 KiB` = `30 * 1024 = 30720` bytes).
pub const MAX_BLANK_FRAME_BYTES: usize =
    ((1 << (1 + 1 + 1 + 1 + 1)) - (1 + 1)) << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1);

/// Deterministic MIME type lookup for lifted Virtual FS assets inside a `.holo` Release Capsule.
#[must_use]
pub fn mime_for_release_path(path: &str) -> &'static str {
    if path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".mjs") || path.ends_with(".js") {
        "text/javascript; charset=utf-8"
    } else if path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if path.ends_with(".json") || path.ends_with(".webmanifest") {
        "application/json; charset=utf-8"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".txt") || path.ends_with(".md") {
        "text/plain; charset=utf-8"
    } else {
        "application/octet-stream"
    }
}

/// Verified entry in a self-contained [`VerdantReleaseCapsule`] Virtual FS manifest.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerdantReleaseFileEntry {
    /// Virtual FS relative path (e.g., `studio/index.html`, `verdant-wasm.wasm`).
    pub path: String,
    /// HTTP Content-Type used by the Blank Frame `ServiceWorker` Virtual FS.
    pub mime_type: String,
    /// Exact byte length of the payload slice in the capsule body.
    pub byte_len: usize,
    /// Content-addressed SHA-256 hex digest of the file bytes.
    pub sha256_hex: String,
}

/// Canonical manifest embedded at the head of every portable `verdant-vX.Y.Z.holo` capsule.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerdantReleaseManifest {
    /// Fixed format identifier (`verdant/release-capsule/1`).
    pub spec: String,
    /// Semantic release version (e.g., `0.1.0`).
    pub version: String,
    /// Strictly monotonic release epoch (`>= 1`, bound to [`StitchMonotonicPointer`]).
    pub release_epoch: u64,
    /// Content-addressed [`VerdantVcs`] commit `kappa` covering this lifted release tree.
    pub vcs_commit_kappa: String,
    /// Cryptographic View Immutability Seal SHA-256 (`site/generated/immutability-seal.json`).
    pub immutability_seal_sha256: String,
    /// Default deep-link route mounted by the Blank Frame (e.g., `studio/index.html`).
    pub default_route: String,
    /// Sorted list of every lifted Virtual FS file and its exact SHA-256 digest.
    pub files: Vec<VerdantReleaseFileEntry>,
    /// Exact sum of all file payload byte lengths.
    pub total_payload_bytes: usize,
}

/// Self-verifying portable Verdant Release `.holo` capsule (`VE-44`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerdantReleaseCapsule {
    /// Parsed & verified release manifest.
    pub manifest: VerdantReleaseManifest,
    /// Verified Virtual FS file bytes keyed by relative path.
    pub payloads: BTreeMap<String, Vec<u8>>,
    /// Exact canonical binary `.holo` bytes.
    pub holo_bytes: Vec<u8>,
    /// Content-addressed SHA-256 hex digest of `holo_bytes` (the release `kappa`).
    pub capsule_sha256_hex: String,
}

impl VerdantReleaseCapsule {
    /// Packs a verified tree of lifted Verdant assets into a canonical binary `.holo` capsule.
    pub fn pack_from_files(
        version: &str,
        release_epoch: u64,
        vcs_commit_kappa: &str,
        immutability_seal_sha256: &str,
        default_route: &str,
        files: &BTreeMap<String, Vec<u8>>,
    ) -> Result<Self, StitchCustodyError> {
        if version.trim().is_empty()
            || release_epoch == 0
            || vcs_commit_kappa.trim().is_empty()
            || files.is_empty()
        {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "release capsule requires non-empty version, epoch >= 1, vcs_commit_kappa, and files".to_string(),
            });
        }
        let mut entries = Vec::with_capacity(files.len());
        let mut payloads = BTreeMap::new();
        let mut total_payload_bytes = 0usize;
        for (raw_path, bytes) in files {
            let clean = raw_path.trim().trim_start_matches("./");
            if clean.is_empty() || clean.contains("..") || clean.starts_with('/') {
                return Err(StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("unsafe Virtual FS release path `{raw_path}`"),
                });
            }
            entries.push(VerdantReleaseFileEntry {
                path: clean.to_string(),
                mime_type: mime_for_release_path(clean).to_string(),
                byte_len: bytes.len(),
                sha256_hex: sha256_hex_raw(bytes),
            });
            total_payload_bytes = total_payload_bytes.saturating_add(bytes.len());
            payloads.insert(clean.to_string(), bytes.clone());
        }
        let manifest = VerdantReleaseManifest {
            spec: "verdant/release-capsule/1".to_string(),
            version: version.trim().to_string(),
            release_epoch,
            vcs_commit_kappa: vcs_commit_kappa.trim().to_string(),
            immutability_seal_sha256: immutability_seal_sha256.trim().to_string(),
            default_route: default_route.trim().trim_start_matches('/').to_string(),
            files: entries,
            total_payload_bytes,
        };
        let manifest_json = serde_json::to_vec(&manifest).map_err(|e| {
            StitchCustodyError::PayloadDigestMismatch {
                detail: format!("serialize release manifest: {e}"),
            }
        })?;
        let manifest_len_u32 = u32::try_from(manifest_json.len()).map_err(|_| {
            StitchCustodyError::PayloadDigestMismatch {
                detail: "release manifest exceeds u32 length".to_string(),
            }
        })?;
        let header_extra = 1usize + 1usize + 1usize + 1usize;
        let mut holo_bytes = Vec::with_capacity(
            RELEASE_CAPSULE_MAGIC.len() + header_extra + manifest_json.len() + total_payload_bytes,
        );
        holo_bytes.extend_from_slice(RELEASE_CAPSULE_MAGIC);
        holo_bytes.extend_from_slice(&manifest_len_u32.to_le_bytes());
        holo_bytes.extend_from_slice(&manifest_json);
        for entry in &manifest.files {
            if let Some(bytes) = payloads.get(&entry.path) {
                holo_bytes.extend_from_slice(bytes);
            }
        }
        let capsule_sha256_hex = sha256_hex_raw(&holo_bytes);
        Ok(Self {
            manifest,
            payloads,
            holo_bytes,
            capsule_sha256_hex,
        })
    }

    /// Cryptographically verifies a raw binary `.holo` capsule, checking header magic, manifest
    /// bounds, zero trailing garbage, and every lifted file's exact SHA-256 digest (`Threat 15`).
    pub fn verify_and_unpack(raw_bytes: &[u8]) -> Result<Self, StitchCustodyError> {
        let magic_len = RELEASE_CAPSULE_MAGIC.len();
        let u32_len = 1usize + 1usize + 1usize + 1usize;
        let min_header = magic_len + u32_len;
        if raw_bytes.len() < min_header || &raw_bytes[..magic_len] != RELEASE_CAPSULE_MAGIC {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "invalid Verdant release capsule header magic".to_string(),
            });
        }
        let mut len_buf = [0u8; 1 + 1 + 1 + 1];
        len_buf.copy_from_slice(&raw_bytes[magic_len..min_header]);
        let manifest_len = u32::from_le_bytes(len_buf) as usize;
        let body_start = min_header.saturating_add(manifest_len);
        if manifest_len == 0 || body_start > raw_bytes.len() {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "truncated Verdant release capsule manifest".to_string(),
            });
        }
        let manifest: VerdantReleaseManifest =
            serde_json::from_slice(&raw_bytes[min_header..body_start]).map_err(|e| {
                StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("invalid release capsule manifest JSON: {e}"),
                }
            })?;
        if manifest.spec != "verdant/release-capsule/1" || manifest.files.is_empty() {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "unsupported release capsule spec or empty file table".to_string(),
            });
        }
        let mut cursor = body_start;
        let mut payloads = BTreeMap::new();
        let mut counted_bytes = 0usize;
        for entry in &manifest.files {
            if entry.path.is_empty() || entry.path.contains("..") || entry.path.starts_with('/') {
                return Err(StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("unsafe Virtual FS path `{}`", entry.path),
                });
            }
            let end = cursor.saturating_add(entry.byte_len);
            if end > raw_bytes.len() {
                return Err(StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("truncated payload for `{}`", entry.path),
                });
            }
            let slice = &raw_bytes[cursor..end];
            let actual_sha = sha256_hex_raw(slice);
            if actual_sha != entry.sha256_hex {
                return Err(StitchCustodyError::PayloadDigestMismatch {
                    detail: format!("SHA-256 mismatch on lifted file `{}`", entry.path),
                });
            }
            payloads.insert(entry.path.clone(), slice.to_vec());
            cursor = end;
            counted_bytes = counted_bytes.saturating_add(entry.byte_len);
        }
        if cursor != raw_bytes.len() || counted_bytes != manifest.total_payload_bytes {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "release capsule byte length mismatch or trailing unverified bytes"
                    .to_string(),
            });
        }
        Ok(Self {
            manifest,
            payloads,
            holo_bytes: raw_bytes.to_vec(),
            capsule_sha256_hex: sha256_hex_raw(raw_bytes),
        })
    }

    /// Shards this release capsule via systematic `k-of-n` Reed-Solomon (`4-of-8` default) and
    /// mints a signed [`StitchMonotonicPointer`] + [`FramePinManifest`] for the Blank Frame.
    #[allow(clippy::too_many_arguments)]
    pub fn shard_for_stitch(
        &self,
        data_shards_k: u8,
        total_shards_n: u8,
        prev_kappa_hex: &str,
        signers: &[(&str, &[u8])],
        min_quorum: u32,
        default_stitch_uri: &str,
        stitch_relays: Vec<String>,
    ) -> Result<(Vec<StitchRsShard>, FramePinManifest), StitchCustodyError> {
        let _ = parse_stitch_deep_link_uri(default_stitch_uri)?;
        let shards =
            ReedSolomonStitchCodec::shard_k_of_n(&self.holo_bytes, data_shards_k, total_shards_n)?;
        let pointer = StitchMonotonicPointer::sign_quorum(
            default_stitch_uri,
            self.manifest.release_epoch,
            prev_kappa_hex,
            &self.capsule_sha256_hex,
            min_quorum,
            signers,
        )?;
        let shard_files: Vec<String> = shards
            .iter()
            .map(|s| format!("shard-{}.stch", s.index))
            .collect();
        let pin = FramePinManifest {
            spec: "verdant/frame-pin/1".to_string(),
            release_version: self.manifest.version.clone(),
            release_epoch: self.manifest.release_epoch,
            release_kappa_hex: self.capsule_sha256_hex.clone(),
            prev_kappa_hex: prev_kappa_hex.to_string(),
            vcs_commit_kappa: self.manifest.vcs_commit_kappa.clone(),
            immutability_seal_sha256: self.manifest.immutability_seal_sha256.clone(),
            default_stitch_uri: default_stitch_uri.to_string(),
            data_shards_k,
            total_shards_n,
            capsule_byte_len: self.holo_bytes.len(),
            pointer,
            stitch_relays,
            shard_files,
        };
        Ok((shards, pin))
    }

    /// Recovers and cryptographically verifies a [`VerdantReleaseCapsule`] from any `>= k` Stitch
    /// shards, enforcing monotonic pointer anti-rollback (`Threat 14`) and SHA-256 integrity
    /// (`Threat 15`).
    pub fn recover_from_stitch(
        shards: &[StitchRsShard],
        pin: &FramePinManifest,
        persisted_epoch: u64,
        expected_prev_kappa: Option<&str>,
        trusted_quorum_secrets: &BTreeMap<String, Vec<u8>>,
    ) -> Result<Self, StitchCustodyError> {
        pin.pointer
            .verify_advance(persisted_epoch, expected_prev_kappa, trusted_quorum_secrets)?;
        let raw = ReedSolomonStitchCodec::recover_k_of_n(shards)?;
        let capsule = Self::verify_and_unpack(&raw)?;
        if capsule.capsule_sha256_hex != pin.release_kappa_hex
            || capsule.capsule_sha256_hex != pin.pointer.target_kappa_hex
        {
            return Err(StitchCustodyError::PayloadDigestMismatch {
                detail: "recovered .holo release kappa does not match signed FramePinManifest"
                    .to_string(),
            });
        }
        Ok(capsule)
    }
}

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
    (RELEASE_RS_DATA_SHARDS, RELEASE_RS_TOTAL_SHARDS)
}

fn hex_encode_bytes(bytes: &[u8]) -> String {
    const HEX: &[u8; 1 << (1 + 1 + 1 + 1)] = b"0123456789abcdef";
    let shift = 1usize + 1usize + 1usize + 1usize;
    let mask = (1usize << shift) - 1usize;
    let mut out = String::with_capacity(bytes.len() * (1 + 1));
    for &b in bytes {
        let v = usize::from(b);
        out.push(HEX[v >> shift] as char);
        out.push(HEX[v & mask] as char);
    }
    out
}

/// Zero-account in-browser Studio authoring bridge (`VE-49`): commits an authored 3D scene/app
/// into [`VerdantVcs`] with W3C PROV-O lineage, packs a self-verifying portable `.holo` capsule
/// (`VDNT_HOLO_REL_V1`), shards it `4-of-8` (`STCH2`), and returns a sanitized `stitch://` deep
/// link ready for Blank Frame mounting.
#[must_use]
pub fn studio_vcs_commit_and_pack_holo_json(raw_json: &str) -> String {
    let Ok(req) = serde_json::from_str::<serde_json::Value>(raw_json) else {
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
    let html = format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>{app_title}</title></head>\
         <body><main><h1>{app_title}</h1><p>Committed by {author} via VerdantVcs (zero-account portable .holo).</p>\
         <pre id=\"verdant-scene-data\">{scene_json}</pre></main></body></html>\n"
    );
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
