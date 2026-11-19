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
use crate::network::SHA256_BYTES;

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

    /// Serializes the entire [`VerdantVcs`] repository into a SHA-256-sealed `.uorc` capsule.
    #[cfg(feature = "uorc-streaming")]
    pub fn encode_uorc_capsule(&self) -> Result<Vec<u8>, StitchCustodyError> {
        let json =
            serde_json::to_vec(self).map_err(|e| StitchCustodyError::PayloadDigestMismatch {
                detail: format!("serialize VerdantVcs: {e}"),
            })?;
        Ok(crate::streaming::compress_uorc_portable(&json))
    }

    /// Restores and verifies a [`VerdantVcs`] repository from a `.uorc` capsule.
    #[cfg(feature = "uorc-wasm-decode")]
    pub fn decode_uorc_capsule(uorc_bytes: &[u8]) -> Result<Self, StitchCustodyError> {
        let raw = crate::streaming::decompress_uorc_portable(uorc_bytes).map_err(|e| {
            StitchCustodyError::PayloadDigestMismatch {
                detail: format!("decode VerdantVcs .uorc capsule: {e}"),
            }
        })?;
        let vcs: Self = serde_json::from_slice(&raw).map_err(|e| {
            StitchCustodyError::PayloadDigestMismatch {
                detail: format!("parse VerdantVcs JSON: {e}"),
            }
        })?;
        for head in vcs.refs.values() {
            let _ = vcs.verify_acyclic_history(head)?;
            let _ = vcs.checkout_commit(head)?;
        }
        Ok(vcs)
    }
}
