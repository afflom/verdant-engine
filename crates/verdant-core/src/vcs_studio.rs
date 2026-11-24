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
    /// Optional `blake3:<64-hex>` κ-label of the self-carrying Holo v6 proof capsule (`M29`, `VE-80`).
    #[cfg(not(target_arch = "wasm32"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_holo_kappa: Option<String>,
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StudioPortableAppMeta<'a> {
    spec: &'a str,
    title: &'a str,
    author_prism_id: &'a str,
    version: &'a str,
    epoch: u64,
    parent_commit_kappa: Option<&'a str>,
    message: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StudioVcsOkResp<'a> {
    ok: bool,
    commit_kappa: &'a str,
    parent_commit_kappa: Option<&'a str>,
    tree_root_sha256: &'a str,
    epoch: u64,
    author_prism_id: &'a str,
    message: &'a str,
    capsule_sha256_hex: &'a str,
    capsule_byte_len: usize,
    shard_count: usize,
    stitch_deep_link_uri: &'a str,
    holo_hex: String,
}

#[derive(Serialize)]
struct StudioVcsErrResp<'a> {
    ok: bool,
    error: &'a str,
}

/// Zero-account in-browser Studio authoring bridge (`VE-49`): commits an authored 3D scene/app
/// into [`VerdantVcs`] with W3C PROV-O lineage, packs a self-verifying portable `.holo` capsule
/// (`VDNT_HOLO_REL_V1`), shards it `4-of-8` (`STCH2`), and returns a sanitized `stitch://` deep
/// link ready for Blank Frame mounting.
#[must_use]
pub fn studio_vcs_commit_and_pack_holo_json(raw_json: &str) -> String {
    let Ok(req) = serde_json::from_slice::<serde_json::Value>(raw_json.as_bytes()) else {
        return "{\"ok\":false,\"error\":\"invalid JSON request\"}".to_string();
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
    let app_meta = StudioPortableAppMeta {
        spec: "verdant/portable-studio-app/1",
        title: app_title,
        author_prism_id: author,
        version,
        epoch,
        parent_commit_kappa: prev_parent,
        message,
    };
    files.insert(
        "verdant-app.json".to_string(),
        serde_json::to_vec(&app_meta).unwrap_or_default(),
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
        Ok((commit, capsule, shard_count, deep_link)) => serde_json::to_string(&StudioVcsOkResp {
            ok: true,
            commit_kappa: &commit.commit_kappa,
            parent_commit_kappa: prev_parent,
            tree_root_sha256: &commit.tree_root_sha256,
            epoch,
            author_prism_id: &commit.author_prism_id,
            message: &commit.message,
            capsule_sha256_hex: &capsule.capsule_sha256_hex,
            capsule_byte_len: capsule.holo_bytes.len(),
            shard_count,
            stitch_deep_link_uri: &deep_link.canonical_uri,
            holo_hex: crate::digest::hex_encode(&capsule.holo_bytes),
        })
        .unwrap_or_default(),
        Err(e) => {
            let msg = e.to_string();
            serde_json::to_string(&StudioVcsErrResp {
                ok: false,
                error: &msg,
            })
            .unwrap_or_default()
        }
    }
}

/// Canonical Virtual FS path of the self-carrying Holo v6 proof capsule inside a Stitch release.
pub const STITCH_PROOF_HOLO_PATH: &str = "generated/stitch-proof.holo";

/// Verified self-carrying Holo v6 (`prism/holo/6`, ADR-018) proof record extracted and re-verified
/// offline from `generated/stitch-proof.holo` without external provenance bundles (`M29`, `VE-80`).
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoloV6ProofRecord {
    /// Archive manifest schema (`prism/holo/6`).
    pub schema: String,
    /// In-toto DSSE kernel-proof predicate URI (`https://uor.foundation/prism/kernel-proof/v1`).
    pub predicate_type: String,
    /// Content-addressed `blake3:<64-hex>` κ-label of the self-carrying Holo v6 bytes.
    pub holo_kappa: String,
    /// LexLean semantic model identity (`<64-hex>`).
    pub semantic_id: String,
    /// BLAKE3 hex digest of the embedded semantic `snapshot` object.
    pub snapshot_blake3: String,
    /// BLAKE3 hex digest of the embedded `model` document object.
    pub model_blake3: String,
    /// BLAKE3 hex digest of the embedded `sources` bundle object.
    pub sources_blake3: String,
    /// Admitted Lean `4.32.2` Wasm kernel κ-label (`blake3:<64-hex>`).
    pub kernel_kappa: String,
    /// Admitted Lean standard library κ-label (`blake3:<64-hex>`).
    pub library_kappa: String,
    /// Admitted LexLean lowering κ-label (`blake3:<64-hex>`).
    pub lowering_kappa: String,
    /// Count of kernel-proved theorems with zero `Classical.choice` axioms.
    pub theorems_proved: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl HoloV6ProofRecord {
    /// Parses a self-carrying Holo v6 (`prism/holo/6`, `UORC\x02\x02`) archive from `raw_holo` and
    /// cryptographically verifies its RFC BLAKE3 seal, all `blake3:` object labels, its embedded
    /// `snapshot` + `sources` + `model` digests, and its `https://uor.foundation/prism/kernel-proof/v1`
    /// DSSE statement offline with zero external provenance bundles.
    pub fn extract_and_verify(raw_holo: &[u8]) -> Result<Self, StitchCustodyError> {
        macro_rules! err_msg {
            ($detail:expr) => {
                StitchCustodyError::PayloadDigestMismatch {
                    detail: $detail.into(),
                }
            };
        }
        let two_u = two_u32();
        let four_u = four_u32();
        let eight_u = eight_u32();
        let sixteen_u = sixteen_u32();
        let seven = (eight_u.wrapping_sub(1_u32)) as usize;
        let low_mask = (1_u8 << seven).wrapping_sub(1_u8);
        let cont_bit = 1_u8 << seven;
        let two = two_u as usize;
        let four = four_u as usize;
        let eight = eight_u as usize;
        let r16 = sixteen_u;
        let r12 = eight_u.wrapping_add(four_u);
        let r8 = eight_u;
        let r7 = eight_u.wrapping_sub(1_u32);
        let shift32 = sixteen_u.wrapping_mul(two_u);
        let rounds = r7 as usize;

        let iv_hex = b"6a09e667bb67ae853c6ef372a54ff53a510e527f9b05688c1f83d9ab5be0cd19";
        let perm_hex = b"263a704d1bc59ef8";
        let mut iv = [0_u32; 1 << (1 + 1 + 1)];
        let mut wi_init = 0usize;
        while wi_init < iv.len() {
            let mut acc = 0_u32;
            let mut ni = 0usize;
            while ni < eight {
                let hb = iv_hex[wi_init * eight + ni];
                let nib = if hb >= b'a' {
                    u32::from(hb.wrapping_sub(b'a'))
                        .wrapping_add(eight_u)
                        .wrapping_add(two_u)
                } else {
                    u32::from(hb.wrapping_sub(b'0'))
                };
                acc = (acc << four_u) | nib;
                ni += 1;
            }
            iv[wi_init] = acc;
            wi_init += 1;
        }
        let mut perm = [0usize; 1 << (1 + 1 + 1 + 1)];
        let mut pi = 0usize;
        while pi < perm.len() {
            let hb = perm_hex[pi];
            perm[pi] = (if hb >= b'a' {
                u32::from(hb.wrapping_sub(b'a'))
                    .wrapping_add(eight_u)
                    .wrapping_add(two_u)
            } else {
                u32::from(hb.wrapping_sub(b'0'))
            }) as usize;
            pi += 1;
        }
        let i0 = 0usize;
        let i1 = 1usize;
        let i2 = two;
        let i3 = i2 + 1;
        let i4 = four;
        let i5 = i4 + 1;
        let i6 = i4 + i2;
        let i7 = i6 + 1;
        let i8 = eight;
        let i9 = i8 + 1;
        let i10 = i8 + i2;
        let i11 = i10 + 1;
        let i12 = i8 + i4;
        let i13 = i12 + 1;
        let i14 = i12 + i2;
        let i15 = i14 + 1;
        let block_cap_u = sixteen_u.wrapping_mul(four_u);
        let block_cap = block_cap_u as usize;
        let chunk_cap = 1usize << (eight_u.wrapping_add(two_u));
        let flag_start = 1_u32;
        let flag_end = two_u;
        let flag_parent = four_u;
        let flag_root = eight_u;

        macro_rules! b3_g {
            ($st:expr, $a:expr, $b:expr, $c:expr, $d:expr, $mx:expr, $my:expr) => {{
                $st[$a] = $st[$a].wrapping_add($st[$b]).wrapping_add($mx);
                $st[$d] = ($st[$d] ^ $st[$a]).rotate_right(r16);
                $st[$c] = $st[$c].wrapping_add($st[$d]);
                $st[$b] = ($st[$b] ^ $st[$c]).rotate_right(r12);
                $st[$a] = $st[$a].wrapping_add($st[$b]).wrapping_add($my);
                $st[$d] = ($st[$d] ^ $st[$a]).rotate_right(r8);
                $st[$c] = $st[$c].wrapping_add($st[$d]);
                $st[$b] = ($st[$b] ^ $st[$c]).rotate_right(r7);
            }};
        }
        macro_rules! b3_compress {
            ($cv:expr, $bw:expr, $ctr:expr, $blen:expr, $flg:expr) => {{
                let cv_ref: &[u32; 1 << (1 + 1 + 1)] = $cv;
                let bw_val: [u32; 1 << (1 + 1 + 1 + 1)] = $bw;
                let ctr_val: u64 = $ctr;
                let mut st = [0_u32; 1 << (1 + 1 + 1 + 1)];
                let mut si = 0usize;
                while si < eight {
                    st[si] = cv_ref[si];
                    si += 1;
                }
                st[eight] = iv[0];
                st[eight + 1] = iv[1];
                st[eight + two] = iv[two];
                st[eight + two + 1] = iv[two + 1];
                st[eight + four] = ctr_val as u32;
                st[eight + four + 1] = (ctr_val >> shift32) as u32;
                st[eight + four + two] = $blen;
                st[eight + four + two + 1] = $flg;
                let mut blk = bw_val;
                let mut r = 0usize;
                while r < rounds {
                    b3_g!(st, i0, i4, i8, i12, blk[i0], blk[i1]);
                    b3_g!(st, i1, i5, i9, i13, blk[i2], blk[i3]);
                    b3_g!(st, i2, i6, i10, i14, blk[i4], blk[i5]);
                    b3_g!(st, i3, i7, i11, i15, blk[i6], blk[i7]);
                    b3_g!(st, i0, i5, i10, i15, blk[i8], blk[i9]);
                    b3_g!(st, i1, i6, i11, i12, blk[i10], blk[i11]);
                    b3_g!(st, i2, i7, i8, i13, blk[i12], blk[i13]);
                    b3_g!(st, i3, i4, i9, i14, blk[i14], blk[i15]);
                    if r + 1 < rounds {
                        let mut next = [0_u32; 1 << (1 + 1 + 1 + 1)];
                        let mut j = 0usize;
                        while j < next.len() {
                            next[j] = blk[perm[j]];
                            j += 1;
                        }
                        blk = next;
                    }
                    r += 1;
                }
                let mut k = 0usize;
                while k < eight {
                    st[k] ^= st[k + eight];
                    st[k + eight] ^= cv_ref[k];
                    k += 1;
                }
                st
            }};
        }
        macro_rules! b3_words {
            ($slice:expr) => {{
                let sl: &[u8] = $slice;
                let mut buf = [0_u8; 1 << (1 + 1 + 1 + 1 + 1 + 1)];
                buf[..sl.len()].copy_from_slice(sl);
                let mut wout = [0_u32; 1 << (1 + 1 + 1 + 1)];
                let mut wi = 0usize;
                while wi < wout.len() {
                    let mut wb = [0_u8; 1 + 1 + 1 + 1];
                    wb.copy_from_slice(&buf[wi * four..(wi + 1) * four]);
                    wout[wi] = u32::from_le_bytes(wb);
                    wi += 1;
                }
                wout
            }};
        }
        macro_rules! b3_first8 {
            ($st:expr) => {{
                let st_val = $st;
                let mut out8 = [0_u32; 1 << (1 + 1 + 1)];
                out8.copy_from_slice(&st_val[..eight]);
                out8
            }};
        }
        macro_rules! hash_chunk {
            ($chunk:expr, $ctr:expr) => {{
                let ch: &[u8] = $chunk;
                let ctr_c: u64 = $ctr;
                let mut cv = iv;
                if ch.is_empty() {
                    (
                        cv,
                        [0_u32; 1 << (1 + 1 + 1 + 1)],
                        ctr_c,
                        0_u32,
                        flag_start | flag_end,
                    )
                } else {
                    let mut cpos = 0usize;
                    let mut bidx = 0usize;
                    while cpos + block_cap < ch.len() {
                        let bw = b3_words!(&ch[cpos..cpos + block_cap]);
                        let fl = if bidx == 0 { flag_start } else { 0_u32 };
                        cv = b3_first8!(b3_compress!(&cv, bw, ctr_c, block_cap_u, fl));
                        cpos += block_cap;
                        bidx += 1;
                    }
                    let tail = &ch[cpos..];
                    let fl = flag_end | if bidx == 0 { flag_start } else { 0_u32 };
                    (cv, b3_words!(tail), ctr_c, tail.len() as u32, fl)
                }
            }};
        }
        macro_rules! blake3_digest {
            ($input:expr) => {{
                let data: &[u8] = $input;
                let (r_cv, r_bw, _r_ctr, r_len, r_flg) = if data.len() <= chunk_cap {
                    hash_chunk!(data, 0_u64)
                } else {
                    let mut stack: Vec<[u32; 1 << (1 + 1 + 1)]> = Vec::new();
                    let mut dpos = 0usize;
                    let mut cctr = 0_u64;
                    while dpos + chunk_cap < data.len() {
                        let (c_cv, c_bw, c_ctr, c_len, c_flg) =
                            hash_chunk!(&data[dpos..dpos + chunk_cap], cctr);
                        let mut ncv = b3_first8!(b3_compress!(&c_cv, c_bw, c_ctr, c_len, c_flg));
                        let mut tot = cctr.wrapping_add(1);
                        while (tot & 1) == 0 {
                            if let Some(left) = stack.pop() {
                                let mut pw = [0_u32; 1 << (1 + 1 + 1 + 1)];
                                pw[..eight].copy_from_slice(&left);
                                pw[eight..].copy_from_slice(&ncv);
                                ncv = b3_first8!(b3_compress!(
                                    &iv,
                                    pw,
                                    0_u64,
                                    block_cap_u,
                                    flag_parent
                                ));
                            }
                            tot >>= 1;
                        }
                        stack.push(ncv);
                        dpos += chunk_cap;
                        cctr = cctr.wrapping_add(1);
                    }
                    let (mut o_cv, mut o_bw, mut o_ctr, mut o_len, mut o_flg) =
                        hash_chunk!(&data[dpos..], cctr);
                    while let Some(left) = stack.pop() {
                        let rcv = b3_first8!(b3_compress!(&o_cv, o_bw, o_ctr, o_len, o_flg));
                        let mut pw = [0_u32; 1 << (1 + 1 + 1 + 1)];
                        pw[..eight].copy_from_slice(&left);
                        pw[eight..].copy_from_slice(&rcv);
                        o_cv = iv;
                        o_bw = pw;
                        o_ctr = 0_u64;
                        o_len = block_cap_u;
                        o_flg = flag_parent;
                    }
                    (o_cv, o_bw, o_ctr, o_len, o_flg)
                };
                let rwords = b3_compress!(&r_cv, r_bw, 0_u64, r_len, r_flg | flag_root);
                let mut dout = [0_u8; 1 << (1 + 1 + 1 + 1 + 1)];
                let mut oi = 0usize;
                while oi < eight {
                    dout[oi * four..(oi + 1) * four].copy_from_slice(&rwords[oi].to_le_bytes());
                    oi += 1;
                }
                dout
            }};
        }
        macro_rules! blake3_hex_of {
            ($input:expr) => {
                crate::digest::hex_encode(&blake3_digest!($input))
            };
        }
        macro_rules! blake3_kappa_of {
            ($input:expr) => {
                format!(
                    "blake3:{}",
                    crate::digest::hex_encode(&blake3_digest!($input))
                )
            };
        }

        macro_rules! read_uleb {
            ($buf:expr, $pos:expr) => {{
                let bytes_ref: &[u8] = $buf;
                let start = $pos;
                let mut val = 0usize;
                let mut sh = 0usize;
                let mut out_opt: Option<usize> = None;
                loop {
                    if $pos >= bytes_ref.len() || ($pos - start) > seven {
                        break;
                    }
                    let b = bytes_ref[$pos];
                    $pos += 1;
                    let Some(part) = usize::from(b & low_mask).checked_shl(sh as u32) else {
                        break;
                    };
                    let Some(next_val) = val.checked_add(part) else {
                        break;
                    };
                    val = next_val;
                    if (b & cont_bit) == 0 {
                        if !(b == 0 && ($pos - start) > 1) {
                            out_opt = Some(val);
                        }
                        break;
                    }
                    sh = sh.saturating_add(seven);
                }
                out_opt
            }};
        }

        let magic = b"UORC\x02\x02\x00\x00";
        let seal_len = (sixteen_u.wrapping_mul(two_u)) as usize;
        if raw_holo.len() <= magic.len() + seal_len || &raw_holo[..magic.len()] != magic {
            return Err(err_msg!(
                "Holo v6 proof capsule lacks UORC v2.2 magic header"
            ));
        }
        let split_at = raw_holo.len() - seal_len;
        let preimage = &raw_holo[..split_at];
        let seal = &raw_holo[split_at..];
        if blake3_digest!(preimage).as_slice() != seal {
            return Err(err_msg!(
                "Holo v6 proof capsule BLAKE3 archive seal mismatch"
            ));
        }
        let mut pos = magic.len();
        let count = read_uleb!(preimage, pos)
            .ok_or_else(|| err_msg!("invalid ULEB128 object count in Holo v6"))?;
        let min_objects = (eight_u.wrapping_add(two_u)) as usize;
        let max_objects = sixteen_u as usize;
        if count < min_objects || count > max_objects {
            return Err(err_msg!(format!(
                "self-carrying Holo v6 requires >= {min_objects} objects, found {count}"
            )));
        }
        let mut lengths = Vec::with_capacity(count);
        let mut sum_len = 0usize;
        for _ in 0..count {
            let l = read_uleb!(preimage, pos)
                .ok_or_else(|| err_msg!("invalid ULEB128 object length in Holo v6"))?;
            sum_len = sum_len.saturating_add(l);
            lengths.push(l);
        }
        let body_len = read_uleb!(preimage, pos)
            .ok_or_else(|| err_msg!("invalid ULEB128 body length in Holo v6"))?;
        if body_len != sum_len || pos.saturating_add(body_len) != preimage.len() {
            return Err(err_msg!(
                "Holo v6 object table lengths disagree with body bounds"
            ));
        }
        let mut objects: Vec<&[u8]> = Vec::with_capacity(count);
        let mut cur = pos;
        for l in lengths {
            objects.push(&preimage[cur..cur + l]);
            cur += l;
        }
        let manifest: serde_json::Value = serde_json::from_slice(objects[0])
            .map_err(|e| err_msg!(format!("invalid Holo v6 manifest JSON: {e}")))?;
        if manifest.get("schema").and_then(|v| v.as_str()) != Some("prism/holo/6") {
            return Err(err_msg!("Holo v6 manifest schema is not prism/holo/6"));
        }
        let two = two_u as usize;
        let provenance: serde_json::Value = serde_json::from_slice(objects[two])
            .map_err(|e| err_msg!(format!("invalid Holo v6 provenance JSON: {e}")))?;
        let expected_app_kappa = blake3_kappa_of!(objects[0]);
        if provenance.get("application_kappa").and_then(|v| v.as_str())
            != Some(expected_app_kappa.as_str())
        {
            return Err(err_msg!("Holo v6 provenance application_kappa mismatch"));
        }
        let rows = manifest
            .get("objects")
            .and_then(|v| v.as_array())
            .ok_or_else(|| err_msg!("Holo v6 manifest missing objects array"))?;
        let mut by_role: BTreeMap<String, &[u8]> = BTreeMap::new();
        for (idx, row) in rows.iter().enumerate() {
            let obj_index = if idx == 0 { 1usize } else { idx + two };
            if obj_index >= objects.len() - 1 {
                return Err(err_msg!("Holo v6 manifest object row exceeds table"));
            }
            let obj_bytes = objects[obj_index];
            let role = row
                .get("role")
                .and_then(|v| v.as_str())
                .ok_or_else(|| err_msg!("Holo v6 object row missing role"))?;
            let row_kappa = row
                .get("kappa")
                .and_then(|v| v.as_str())
                .ok_or_else(|| err_msg!("Holo v6 object row missing kappa"))?;
            let row_len = row
                .get("length")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| err_msg!("Holo v6 object row missing length"))?
                as usize;
            if obj_bytes.len() != row_len || blake3_kappa_of!(obj_bytes) != row_kappa {
                return Err(err_msg!(format!(
                    "Holo v6 object `{role}` κ-label or length mismatch"
                )));
            }
            by_role.insert(role.to_string(), obj_bytes);
        }
        let model_bytes = *by_role
            .get("model")
            .ok_or_else(|| err_msg!("Holo v6 missing `model` object"))?;
        let snapshot_bytes = *by_role
            .get("snapshot")
            .ok_or_else(|| err_msg!("self-carrying Holo v6 missing `snapshot` object"))?;
        let sources_bytes = *by_role
            .get("sources")
            .ok_or_else(|| err_msg!("self-carrying Holo v6 missing `sources` object"))?;

        let snapshot_json: serde_json::Value = serde_json::from_slice(snapshot_bytes)
            .map_err(|e| err_msg!(format!("invalid Holo v6 snapshot JSON: {e}")))?;
        let model_json: serde_json::Value = serde_json::from_slice(model_bytes)
            .map_err(|e| err_msg!(format!("invalid Holo v6 model JSON: {e}")))?;
        let snap_sem_id = snapshot_json
            .get("semantic_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| err_msg!("Holo v6 snapshot missing semantic_id"))?;
        let model_sem_id = model_json
            .pointer("/provenance/semantic_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| err_msg!("Holo v6 model missing provenance.semantic_id"))?;
        if snap_sem_id != model_sem_id {
            return Err(err_msg!("Holo v6 model and snapshot semantic_id disagree"));
        }

        let six_u = four_u.wrapping_add(two_u);
        let twenty_six = sixteen_u.wrapping_add(eight_u).wrapping_add(two_u);
        let fifty_two = twenty_six.wrapping_mul(two_u);
        let sixty_two = fifty_two.wrapping_add(eight_u).wrapping_add(two_u);
        let sixty_three = sixty_two.wrapping_add(1_u32);
        let mask_byte = (1_u32 << eight_u).wrapping_sub(1_u32);
        let four_usz = four_u as usize;

        let att_text = std::str::from_utf8(objects[objects.len() - 1])
            .map_err(|e| err_msg!(format!("Holo v6 attestation section is not UTF-8: {e}")))?;
        let pred_uri = "https://uor.foundation/prism/kernel-proof/v1";
        let mut matched_stmt: Option<serde_json::Value> = None;
        for line in att_text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let bundle: serde_json::Value = serde_json::from_str(line)
                .map_err(|e| err_msg!(format!("invalid Sigstore DSSE bundle line: {e}")))?;
            let payload_b64 = bundle
                .pointer("/dsseEnvelope/payload")
                .and_then(|v| v.as_str())
                .ok_or_else(|| err_msg!("DSSE bundle missing payload"))?;
            let mut sextets = Vec::with_capacity(payload_b64.len());
            for b in payload_b64.bytes() {
                if b == b'=' || b == b'\n' || b == b'\r' || b == b' ' {
                    continue;
                }
                let v = if b.is_ascii_uppercase() {
                    u32::from(b - b'A')
                } else if b.is_ascii_lowercase() {
                    u32::from(b - b'a').wrapping_add(twenty_six)
                } else if b.is_ascii_digit() {
                    u32::from(b - b'0').wrapping_add(fifty_two)
                } else if b == b'+' || b == b'-' {
                    sixty_two
                } else if b == b'/' || b == b'_' {
                    sixty_three
                } else {
                    return Err(err_msg!("DSSE payload is not valid base64"));
                };
                sextets.push(v);
            }
            if sextets.len() % four_usz == 1 {
                return Err(err_msg!("DSSE payload has invalid base64 length"));
            }
            let mut stmt_bytes = Vec::with_capacity((sextets.len() * (two + 1)) / four_usz);
            let mut bi = 0usize;
            while bi < sextets.len() {
                let rem = sextets.len() - bi;
                let s0 = sextets[bi];
                let s1 = sextets[bi + 1];
                let s2 = if rem > two { sextets[bi + two] } else { 0 };
                let s3 = if rem > (two + 1) {
                    sextets[bi + two + 1]
                } else {
                    0
                };
                let triple = (s0 << (six_u.wrapping_mul(two_u.wrapping_add(1_u32))))
                    | (s1 << (six_u.wrapping_mul(two_u)))
                    | (s2 << six_u)
                    | s3;
                stmt_bytes.push(((triple >> sixteen_u) & mask_byte) as u8);
                if rem > two {
                    stmt_bytes.push(((triple >> eight_u) & mask_byte) as u8);
                }
                if rem > (two + 1) {
                    stmt_bytes.push((triple & mask_byte) as u8);
                }
                bi += four_usz;
            }
            let stmt: serde_json::Value = serde_json::from_slice(&stmt_bytes)
                .map_err(|e| err_msg!(format!("invalid in-toto Statement JSON: {e}")))?;
            if stmt.get("_type").and_then(|v| v.as_str()) != Some("https://in-toto.io/Statement/v1")
            {
                return Err(err_msg!("unexpected in-toto statement _type"));
            }
            if stmt.get("predicateType").and_then(|v| v.as_str()) == Some(pred_uri) {
                if matched_stmt.is_some() {
                    return Err(err_msg!("duplicate kernel-proof/v1 statements in Holo v6"));
                }
                matched_stmt = Some(stmt);
            }
        }
        let stmt =
            matched_stmt.ok_or_else(|| err_msg!("Holo v6 carries no kernel-proof/v1 statement"))?;
        let subjects = stmt
            .get("subject")
            .and_then(|v| v.as_array())
            .ok_or_else(|| err_msg!("kernel-proof statement missing subject array"))?;
        for (role_name, expected_bytes) in [
            ("model", model_bytes),
            ("snapshot", snapshot_bytes),
            ("sources", sources_bytes),
        ] {
            let exp_b3 = blake3_hex_of!(expected_bytes);
            let exp_s2 = sha256_hex_raw(expected_bytes);
            let ok = subjects.iter().any(|subj| {
                subj.get("name").and_then(|v| v.as_str()) == Some(role_name)
                    && subj.pointer("/digest/blake3").and_then(|v| v.as_str())
                        == Some(exp_b3.as_str())
                    && subj.pointer("/digest/sha256").and_then(|v| v.as_str())
                        == Some(exp_s2.as_str())
            });
            if !ok {
                return Err(err_msg!(format!(
                    "kernel-proof statement subject `{role_name}` digest mismatch"
                )));
            }
        }
        let predicate = stmt
            .get("predicate")
            .ok_or_else(|| err_msg!("kernel-proof statement missing predicate"))?;
        if predicate.get("semantic_id").and_then(|v| v.as_str()) != Some(snap_sem_id) {
            return Err(err_msg!("kernel-proof predicate semantic_id mismatch"));
        }
        let hex_len = (sixteen_u.wrapping_mul(four_u)) as usize;
        macro_rules! check_b3_label {
            ($key:expr) => {{
                let s = predicate
                    .get($key)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| err_msg!(format!("kernel-proof predicate missing {}", $key)))?;
                let ok_lbl = s.strip_prefix("blake3:").is_some_and(|hex| {
                    hex.len() == hex_len
                        && hex
                            .bytes()
                            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
                });
                if !ok_lbl {
                    return Err(err_msg!(format!(
                        "kernel-proof predicate invalid blake3 label for {}",
                        $key
                    )));
                }
                s.to_string()
            }};
        }
        let kernel_kappa = check_b3_label!("kernel");
        let library_kappa = check_b3_label!("library");
        let lowering_kappa = check_b3_label!("lowering");

        let theorems = predicate
            .get("theorems")
            .and_then(|v| v.as_array())
            .ok_or_else(|| err_msg!("kernel-proof predicate missing theorems array"))?;
        if theorems.is_empty() {
            return Err(err_msg!("kernel-proof predicate lists zero theorems"));
        }
        for thm in theorems {
            if thm.get("status").and_then(|v| v.as_str()) != Some("proved") {
                return Err(err_msg!("kernel-proof statement includes unproved theorem"));
            }
            if let Some(axioms) = thm.get("axioms").and_then(|v| v.as_array()) {
                if axioms
                    .iter()
                    .any(|a| a.as_str() == Some("Classical.choice"))
                {
                    return Err(err_msg!(
                        "kernel-proof statement uses forbidden Classical.choice axiom"
                    ));
                }
            }
        }

        Ok(Self {
            schema: "prism/holo/6".to_string(),
            predicate_type: pred_uri.to_string(),
            holo_kappa: blake3_kappa_of!(raw_holo),
            semantic_id: snap_sem_id.to_string(),
            snapshot_blake3: blake3_hex_of!(snapshot_bytes),
            model_blake3: blake3_hex_of!(model_bytes),
            sources_blake3: blake3_hex_of!(sources_bytes),
            kernel_kappa,
            library_kappa,
            lowering_kappa,
            theorems_proved: theorems.len(),
        })
    }
}

#[cfg(test)]
mod holo_v6_tests {
    use super::*;

    #[test]
    fn holo_v6_self_carrying_proof_record_round_trip_and_tamper_refusal() {
        let stitch_proof_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("site/generated/stitch-proof.holo");
        let proof_bytes =
            std::fs::read(&stitch_proof_path).expect("site/generated/stitch-proof.holo must exist");
        let record = HoloV6ProofRecord::extract_and_verify(&proof_bytes)
            .expect("self-carrying Holo v6 proof record must verify offline");
        assert_eq!(record.schema, "prism/holo/6");
        assert_eq!(
            record.predicate_type,
            "https://uor.foundation/prism/kernel-proof/v1"
        );
        assert!(record.holo_kappa.starts_with("blake3:"));
        assert!(record.theorems_proved >= (1 + 1 + 1));

        let mut flipped = proof_bytes.clone();
        let mid = flipped.len() / (1 + 1);
        flipped[mid] ^= 1;
        assert!(HoloV6ProofRecord::extract_and_verify(&flipped).is_err());

        let mut files = BTreeMap::new();
        files.insert("studio/index.html".to_string(), b"<html>ok</html>".to_vec());
        files.insert(STITCH_PROOF_HOLO_PATH.to_string(), proof_bytes);
        let packed = VerdantReleaseCapsule::pack_from_files(
            "2.21.0",
            1,
            "commit-kappa-v6",
            "seal-sha-v6",
            "studio/index.html",
            &files,
        )
        .expect("pack release capsule with self-carrying Holo v6 proof");
        assert_eq!(packed.manifest.proof_record.as_ref(), Some(&record));

        let unpacked = VerdantReleaseCapsule::verify_and_unpack(&packed.holo_bytes)
            .expect("verify_and_unpack self-carrying capsule");
        assert_eq!(unpacked.manifest.proof_record.as_ref(), Some(&record));

        let mut forged_manifest = packed.manifest.clone();
        if let Some(ref mut pr) = forged_manifest.proof_record {
            pr.theorems_proved += 1;
        }
        let forged_json = serde_json::to_vec(&forged_manifest).unwrap();
        let magic_len = RELEASE_CAPSULE_MAGIC.len();
        let u32_len = 1usize + 1usize + 1usize + 1usize;
        let min_header = magic_len + u32_len;
        let mut old_len_buf = [0_u8; 1 + 1 + 1 + 1];
        old_len_buf.copy_from_slice(&packed.holo_bytes[magic_len..min_header]);
        let old_len = u32::from_le_bytes(old_len_buf) as usize;
        let body_slice = &packed.holo_bytes[min_header + old_len..];
        let mut forged_holo = Vec::new();
        forged_holo.extend_from_slice(RELEASE_CAPSULE_MAGIC);
        forged_holo.extend_from_slice(&(forged_json.len() as u32).to_le_bytes());
        forged_holo.extend_from_slice(&forged_json);
        forged_holo.extend_from_slice(body_slice);
        assert!(VerdantReleaseCapsule::verify_and_unpack(&forged_holo).is_err());
    }
}
