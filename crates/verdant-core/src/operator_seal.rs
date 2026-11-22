//! Cryptographic View Immutability Seal & sanctioned owned-file mutation gate (`operator_seal.rs`).

use super::*;

pub(super) const MCP_RESOURCE_SPECS: &[(&str, &str, &str, &str)] = &[
    (
        "verdant://governance/vsm",
        "VSM Cybernetics Authority (AUTH-VSM-CYBERNETICS)",
        "application/json",
        "catalogs/governance/vsm-viable-system.json",
    ),
    (
        "verdant://governance/viewpoints",
        "ISO 42010 Viewpoints & Managers (AUTH-ISO-42010-VIEWPOINTS)",
        "application/json",
        "catalogs/governance/iso42010-viewpoints.json",
    ),
    (
        "verdant://governance/tensor-scaling",
        "ISO 25010 & ITU-R BT.2020/2100 Tensor Scaling Authority (AUTH-ISO-25010-TENSOR-SCALING)",
        "application/json",
        "catalogs/governance/iso25010-tensor-scaling.json",
    ),
    (
        "verdant://governance/stitch-self-hosting",
        "Zero-Account Stitch Self-Hosting & Boundary Privacy (AUTH-STITCH-SELF-HOSTING)",
        "application/json",
        "catalogs/governance/stitch-self-hosting.json",
    ),
    (
        "verdant://governance/change-roadmap-kpi",
        "SemVer Change Manager, Stakeholder Roadmap & ISO 15939 KPIs (AUTH-SEMVER-CHANGE-KPI)",
        "application/json",
        "catalogs/governance/change-roadmap-kpi.json",
    ),
    (
        "verdant://generated/viewpoint-contracts",
        "Manager Viewpoint Contracts",
        "application/json",
        "site/generated/viewpoint-contracts.json",
    ),
    (
        "verdant://generated/experience-design-system",
        "Design System & Communications Spec",
        "application/json",
        "site/generated/experience-design-system.json",
    ),
    (
        "verdant://generated/world-differentiation-profiles",
        "4-World Differentiation Profiles",
        "application/json",
        "site/generated/world-differentiation-profiles.json",
    ),
    (
        "verdant://generated/tensor-scaling-contract",
        "ISO 25010 Tensor Scaling & Paged Memory Contract",
        "application/json",
        "site/generated/tensor-scaling-contract.json",
    ),
    (
        "verdant://generated/stitch-app-catalog",
        "Stitch 2-of-3 Verdant App Catalog",
        "application/json",
        "site/generated/stitch-app-catalog.json",
    ),
    (
        "verdant://generated/release-ledger",
        "Change & Version Manager SemVer Release Ledger",
        "application/json",
        "site/generated/release-ledger.json",
    ),
    (
        "verdant://generated/roadmap-kpi-report",
        "Observer Stakeholder Roadmap & KPI Telemetry Report",
        "application/json",
        "site/generated/roadmap-kpi-report.json",
    ),
    (
        "verdant://generated/immutability-seal",
        "Cryptographic View Immutability Seal",
        "application/json",
        "site/generated/immutability-seal.json",
    ),
    (
        "verdant://generated/self-host-ratchet-manifest",
        "Self-Hosting Parametric Ratchet Manifest (TOML)",
        "application/toml",
        "site/generated/self-host-ratchet-manifest.toml",
    ),
];

impl VerdantOperatorChannel {
    /// Computes and writes the cryptographic View Immutability Seal
    /// (`site/generated/immutability-seal.json`) over every shipped Verdant view and
    /// Manager-synthesized artifact.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn seal_repository_views(
        &mut self,
        repo_root: &Path,
        directive_kappa_hex: &str,
    ) -> Result<MaterializedFileRecord, ObserverModelError> {
        let exec_kappa = self
            .managers
            .iter()
            .find(|m| m.spec.manager_id == "mgr-observer-executive")
            .map(|m| m.kappa_hex.clone())
            .unwrap_or_default();
        let mut entries = Vec::new();
        for &rel in SEALED_IMMUTABLE_VIEW_FILES {
            let full = repo_root.join(rel);
            let bytes = std::fs::read(&full).map_err(|e| {
                ObserverModelError::CommunicationEnvelopeRejected {
                    channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                    detail: format!("immutability seal failed reading `{rel}`: {e}"),
                }
            })?;
            let owner = self
                .managers
                .iter()
                .find(|m| m.spec.owned_files.iter().any(|f| f == rel))
                .ok_or_else(|| ObserverModelError::CommunicationEnvelopeRejected {
                    channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                    detail: format!("sealed view `{rel}` has no accountable Manager in AUTH-ISO-42010-VIEWPOINTS"),
                })?;
            entries.push(SealedFileEntry {
                path: rel.to_string(),
                owning_manager_id: owner.spec.manager_id.clone(),
                manager_kappa_hex: owner.kappa_hex.clone(),
                governing_authorities: owner.spec.governing_authorities.clone(),
                bytes: bytes.len(),
                sha256_hex: sha256_hex_raw(&bytes),
                last_directive_kappa_hex: directive_kappa_hex.to_string(),
            });
        }
        let seal_root = Self::compute_seal_root(&entries);
        let seal = ViewImmutabilitySeal {
            spec: "verdant/view-immutability-seal/1".to_string(),
            owning_manager: "mgr-observer-executive".to_string(),
            manager_kappa_hex: exec_kappa,
            seal_root_sha256_hex: seal_root,
            files: entries,
        };
        let rendered = serde_json::to_string_pretty(&seal).map_err(|e| {
            ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::OrganToOrgan.slug().to_string(),
                detail: e.to_string(),
            }
        })?;
        Self::write_manager_file(
            repo_root,
            "mgr-observer-executive",
            "site/generated/immutability-seal.json",
            &rendered,
        )
    }

    /// Cryptographically verifies `site/generated/immutability-seal.json` against every live
    /// shipped Verdant view and generated artifact, immediately flagging any out-of-band manual
    /// edit or seal tampering.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn verify_immutability_seal(&self, repo_root: &Path) -> ImmutabilityVerificationReport {
        let seal_path = repo_root.join("site/generated/immutability-seal.json");
        let Ok(raw) = std::fs::read_to_string(&seal_path) else {
            return ImmutabilityVerificationReport {
                valid: false,
                checked_files: 0,
                seal_root_sha256_hex: String::new(),
                violations: vec![
                    "missing `site/generated/immutability-seal.json` (run `verdant operator synthesize` or MCP `verdant_synthesize`)"
                        .to_string(),
                ],
            };
        };
        let Ok(seal) = serde_json::from_str::<ViewImmutabilitySeal>(&raw) else {
            return ImmutabilityVerificationReport {
                valid: false,
                checked_files: 0,
                seal_root_sha256_hex: String::new(),
                violations: vec!["malformed `site/generated/immutability-seal.json`".to_string()],
            };
        };
        let mut violations = Vec::new();
        if seal.spec != "verdant/view-immutability-seal/1" {
            violations.push(format!("unexpected immutability seal spec `{}`", seal.spec));
        }
        let expected_root = Self::compute_seal_root(&seal.files);
        if expected_root != seal.seal_root_sha256_hex {
            violations.push(format!(
                "immutability seal root tamper detected: expected `{expected_root}`, recorded `{}`",
                seal.seal_root_sha256_hex
            ));
        }
        let sealed_map: BTreeMap<&str, &SealedFileEntry> =
            seal.files.iter().map(|e| (e.path.as_str(), e)).collect();
        for &req_rel in SEALED_IMMUTABLE_VIEW_FILES {
            if !sealed_map.contains_key(req_rel) {
                violations.push(format!(
                    "required immutable view `{req_rel}` missing from `site/generated/immutability-seal.json`"
                ));
            }
        }
        for entry in &seal.files {
            let full = repo_root.join(&entry.path);
            match std::fs::read(&full) {
                Ok(bytes) => {
                    let observed = sha256_hex_raw(&bytes);
                    if observed != entry.sha256_hex {
                        violations.push(format!(
                            "out-of-band manual edit detected on immutable view `{}` (owner `{}`): sealed `{}`, observed `{observed}` — modify only via Verdant API/MCP",
                            entry.path, entry.owning_manager_id, entry.sha256_hex
                        ));
                    }
                }
                Err(e) => {
                    violations.push(format!("sealed view `{}` unreadable: {e}", entry.path));
                }
            }
        }
        ImmutabilityVerificationReport {
            valid: violations.is_empty(),
            checked_files: seal.files.len(),
            seal_root_sha256_hex: seal.seal_root_sha256_hex,
            violations,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn contains_raw_hex_color(src: &str) -> bool {
        let bytes = src.as_bytes();
        let three = 1usize + 1usize + 1usize;
        let eight = 1usize << three;
        let mut i = 0usize;
        while i < bytes.len() {
            if bytes[i] == b'#' {
                let mut j = i + 1;
                while j < bytes.len() && bytes[j].is_ascii_hexdigit() {
                    j += 1;
                }
                let run = j - (i + 1);
                let prev_ok =
                    i == 0 || matches!(bytes[i - 1], b':' | b' ' | b'\t' | b'\n' | b'(' | b',');
                let next_ok = j >= bytes.len()
                    || matches!(
                        bytes[j],
                        b';' | b' ' | b'\t' | b'\n' | b')' | b',' | b'"' | b'\''
                    );
                if prev_ok && next_ok && run >= three && run <= eight {
                    return true;
                }
                i = j;
            } else {
                i += 1;
            }
        }
        false
    }

    /// Executes a governed, standards-backed declarative mutation strictly inside the admitting
    /// Manager's `ownedFiles`, enforcing DMN 1.5, ISO 42010 Viewpoint purity, DTCG token hygiene,
    /// automatic Immutability Seal re-signing, and automatic rollback if the closing S3* audit fails.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn mutate_owned_file(
        &mut self,
        repo_root: &Path,
        req: &OwnedFileMutationRequest,
    ) -> Result<OwnedFileMutationReport, ObserverModelError> {
        let norm_path = req.path.trim().trim_start_matches("./");
        if norm_path.is_empty() || norm_path.contains("..") || norm_path.starts_with('/') {
            return Ok(OwnedFileMutationReport {
                mutated: false,
                manager_id: String::new(),
                path: req.path.clone(),
                comm_kappa_hex: String::new(),
                sha256_hex: String::new(),
                seal_root_sha256_hex: String::new(),
                diagnostic: "Refused: path must be a clean repository-relative path".to_string(),
            });
        }
        if BYTE_LOCKED_TRUST_ROOTS.contains(&norm_path) {
            return Ok(OwnedFileMutationReport {
                mutated: false,
                manager_id: "mgr-observer-executive".to_string(),
                path: norm_path.to_string(),
                comm_kappa_hex: String::new(),
                sha256_hex: String::new(),
                seal_root_sha256_hex: String::new(),
                diagnostic: format!("Refused: `{norm_path}` is a byte-locked trust root"),
            });
        }
        if req.cited_authority.trim().is_empty()
            || !REQUIRED_LOCKED_AUTHORITIES.contains(&req.cited_authority.trim())
        {
            return Ok(OwnedFileMutationReport {
                mutated: false,
                manager_id: "mgr-observer-executive".to_string(),
                path: norm_path.to_string(),
                comm_kappa_hex: String::new(),
                sha256_hex: String::new(),
                seal_root_sha256_hex: String::new(),
                diagnostic: format!(
                    "Refused (Anti-Contrivance): `{}` is not a locked AUTH-* authority",
                    req.cited_authority
                ),
            });
        }
        let decision = self.submit_directive(&OperatorPromptRequest {
            viewpoint: req.viewpoint.clone(),
            directive: req.directive.clone(),
            cited_authority: Some(req.cited_authority.clone()),
            operator_prism_id: req.operator_prism_id.clone(),
        })?;
        if !decision.admitted {
            return Ok(OwnedFileMutationReport {
                mutated: false,
                manager_id: decision.manager_id,
                path: norm_path.to_string(),
                comm_kappa_hex: decision.comm_kappa_hex,
                sha256_hex: String::new(),
                seal_root_sha256_hex: String::new(),
                diagnostic: decision.rationale,
            });
        }
        let mgr = self
            .managers
            .iter()
            .find(|m| m.spec.manager_id == decision.manager_id)
            .cloned()
            .ok_or_else(|| ObserverModelError::CommunicationEnvelopeRejected {
                channel: CommunicationChannelKind::AgentToAgent.slug().to_string(),
                detail: format!("manager `{}` missing", decision.manager_id),
            })?;
        if !mgr.spec.owned_files.iter().any(|f| f == norm_path) {
            return Ok(OwnedFileMutationReport {
                mutated: false,
                manager_id: mgr.spec.manager_id,
                path: norm_path.to_string(),
                comm_kappa_hex: decision.comm_kappa_hex,
                sha256_hex: String::new(),
                seal_root_sha256_hex: String::new(),
                diagnostic: format!(
                    "Refused (Total Ownership boundary): `{norm_path}` is not owned by `{}`",
                    decision.manager_id
                ),
            });
        }
        let full_path = repo_root.join(norm_path);
        let original_bytes = std::fs::read(&full_path).ok();
        let next_content = match req.mode.as_str() {
            "replace_exact" => {
                let cur = original_bytes
                    .as_deref()
                    .and_then(|b| std::str::from_utf8(b).ok())
                    .unwrap_or("");
                let target = req.target.as_deref().unwrap_or("");
                let replacement = req.replacement.as_deref().unwrap_or("");
                if target.is_empty() || !cur.contains(target) {
                    return Ok(OwnedFileMutationReport {
                        mutated: false,
                        manager_id: mgr.spec.manager_id,
                        path: norm_path.to_string(),
                        comm_kappa_hex: decision.comm_kappa_hex,
                        sha256_hex: String::new(),
                        seal_root_sha256_hex: String::new(),
                        diagnostic: format!(
                            "Refused: replace_exact target not found in `{norm_path}`"
                        ),
                    });
                }
                cur.replacen(target, replacement, 1)
            }
            "write_full" => req.content.clone().unwrap_or_default(),
            other => {
                return Ok(OwnedFileMutationReport {
                    mutated: false,
                    manager_id: mgr.spec.manager_id,
                    path: norm_path.to_string(),
                    comm_kappa_hex: decision.comm_kappa_hex,
                    sha256_hex: String::new(),
                    seal_root_sha256_hex: String::new(),
                    diagnostic: format!(
                        "Refused: unknown mutation mode `{other}` (expected replace_exact|write_full)"
                    ),
                });
            }
        };

        // Pre-write DTCG / Viewpoint Purity gates (writes 0 bytes on violation):
        if (norm_path.ends_with(".html") || norm_path.ends_with(".css"))
            && next_content
                .to_ascii_lowercase()
                .contains("backdrop-filter: blur")
        {
            return Ok(OwnedFileMutationReport {
                mutated: false,
                manager_id: mgr.spec.manager_id,
                path: norm_path.to_string(),
                comm_kappa_hex: decision.comm_kappa_hex,
                sha256_hex: String::new(),
                seal_root_sha256_hex: String::new(),
                diagnostic: "Refused (DTCG S2 Coordinator): `backdrop-filter: blur` is forbidden"
                    .to_string(),
            });
        }
        if norm_path.ends_with(".css") && Self::contains_raw_hex_color(&next_content) {
            return Ok(OwnedFileMutationReport {
                mutated: false,
                manager_id: mgr.spec.manager_id,
                path: norm_path.to_string(),
                comm_kappa_hex: decision.comm_kappa_hex,
                sha256_hex: String::new(),
                seal_root_sha256_hex: String::new(),
                diagnostic:
                    "Refused (DTCG S2 Coordinator): raw #hex color literal forbidden — use var(--...) tokens"
                        .to_string(),
            });
        }
        if mgr.spec.owned_files.first().map(String::as_str) == Some(norm_path)
            && norm_path.ends_with(".html")
        {
            for req_el in &mgr.spec.required_elements {
                if !next_content.contains(req_el) {
                    return Ok(OwnedFileMutationReport {
                        mutated: false,
                        manager_id: mgr.spec.manager_id,
                        path: norm_path.to_string(),
                        comm_kappa_hex: decision.comm_kappa_hex,
                        sha256_hex: String::new(),
                        seal_root_sha256_hex: String::new(),
                        diagnostic: format!(
                            "Refused (ISO 42010): mutation would remove required element `{req_el}`"
                        ),
                    });
                }
            }
            for bad in &mgr.spec.forbidden_cross_contamination {
                if next_content.contains(bad) {
                    return Ok(OwnedFileMutationReport {
                        mutated: false,
                        manager_id: mgr.spec.manager_id,
                        path: norm_path.to_string(),
                        comm_kappa_hex: decision.comm_kappa_hex,
                        sha256_hex: String::new(),
                        seal_root_sha256_hex: String::new(),
                        diagnostic: format!(
                            "Refused (ISO 42010): mutation introduces forbidden cross-contamination `{bad}`"
                        ),
                    });
                }
            }
        }

        // Write validated file, re-seal Immutability Seal, and close via VSM S3* audit (with rollback):
        Self::write_manager_file(repo_root, &mgr.spec.manager_id, norm_path, &next_content)?;
        let _ = self.seal_repository_views(repo_root, &decision.comm_kappa_hex)?;
        let audit = self.audit_repository(repo_root, Some(&mgr.spec.viewpoint_id));
        if !audit.passed {
            if let Some(orig) = original_bytes {
                let _ = std::fs::write(&full_path, &orig);
                let _ = self.seal_repository_views(repo_root, &decision.comm_kappa_hex);
            }
            return Ok(OwnedFileMutationReport {
                mutated: false,
                manager_id: mgr.spec.manager_id,
                path: norm_path.to_string(),
                comm_kappa_hex: decision.comm_kappa_hex,
                sha256_hex: String::new(),
                seal_root_sha256_hex: String::new(),
                diagnostic: format!("Rolled back after VSM S3* audit failure: {audit:?}"),
            });
        }
        let seal_check = self.verify_immutability_seal(repo_root);
        Ok(OwnedFileMutationReport {
            mutated: true,
            manager_id: mgr.spec.manager_id,
            path: norm_path.to_string(),
            comm_kappa_hex: decision.comm_kappa_hex,
            sha256_hex: sha256_hex_raw(next_content.as_bytes()),
            seal_root_sha256_hex: seal_check.seal_root_sha256_hex,
            diagnostic: format!(
                "Mutated `{norm_path}` via `{}` and re-sealed Immutability Seal cleanly",
                decision.manager_id
            ),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn collect_distilled_site_tree(
        site_root: &Path,
        curr: &Path,
        out: &mut BTreeMap<String, Vec<u8>>,
    ) {
        let Ok(rd) = std::fs::read_dir(curr) else {
            return;
        };
        let mut entries: Vec<_> = rd.filter_map(Result::ok).collect();
        entries.sort_by_key(|e| e.path());
        for entry in entries {
            let p = entry.path();
            let Ok(rel) = p.strip_prefix(site_root) else {
                continue;
            };
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            if rel_str == "frame"
                || rel_str.starts_with("frame/")
                || rel_str == "stitch"
                || rel_str.starts_with("stitch/")
                || rel_str == "crates"
                || rel_str.starts_with("crates/")
                || rel_str.ends_with(".test.mjs")
                || rel_str == "precache.json"
                || rel_str == "generated/self-host-ratchet-manifest.toml"
                || rel_str == "generated/immutability-seal.json"
                || rel_str == "generated/release-ledger.json"
                || rel_str == "generated/tensor-scaling-contract.json"
                || rel_str == "generated/viewpoint-contracts.json"
                || rel_str == "spec.html"
                || rel_str == "spec.css"
                || rel_str == "spec.mjs"
                || rel_str == "og.png"
                || rel_str == "sw-gate.mjs"
                || rel_str == "worker-gate.mjs"
                || rel_str == "page.template.html"
                || rel_str == "sw.js"
            {
                continue;
            }
            if p.is_dir() {
                Self::collect_distilled_site_tree(site_root, &p, out);
            } else if p.is_file() {
                if let Ok(bytes) = std::fs::read(&p) {
                    out.insert(rel_str, bytes);
                }
            }
        }
    }

    pub(super) fn compute_seal_root(entries: &[SealedFileEntry]) -> String {
        let mut sorted: Vec<&SealedFileEntry> = entries.iter().collect();
        sorted.sort_by(|a, b| a.path.cmp(&b.path));
        let mut buf = String::new();
        for e in sorted {
            buf.push_str(&e.path);
            buf.push('|');
            buf.push_str(&e.owning_manager_id);
            buf.push('|');
            buf.push_str(&e.manager_kappa_hex);
            buf.push('|');
            buf.push_str(&e.sha256_hex);
            buf.push('\n');
        }
        sha256_hex_raw(buf.as_bytes())
    }

    /// Processes a single NDJSON line inside `verdant operator session` (stateful multi-turn
    /// Operator RPC channel). Returns `(should_continue, single_line_json_response)`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::WorldSnapshot;
    use crate::streaming::StandardSceneState;

    #[test]
    fn total_ownership_invariant_covers_all_parts_and_catches_orphans() {
        let ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");
        let rep = ch.check_total_ownership();
        assert!(rep.complete, "Total Ownership must pass cleanly: {rep:?}");
        assert!(rep.orphans.is_empty());
        assert!(rep.duplicate_owners.is_empty());
        assert_eq!(rep.managers_checked, 13);

        // Negative test: planting an unmanaged orphan route or crate must fail Total Ownership!
        let bad = audit_total_ownership(
            &ch.viewpoints,
            &["/", "/unmanaged-orphan/"],
            REQUIRED_SHIPPED_CRATES,
            &[],
            &[],
            REQUIRED_LOCKED_AUTHORITIES,
        );
        assert!(!bad.complete);
        assert!(bad.orphans.iter().any(|o| o.contains("/unmanaged-orphan/")));
    }

    #[test]
    fn operator_channel_admits_standards_backed_prompts_and_refuses_contrived_or_cross_contaminated(
    ) {
        let mut ch = VerdantOperatorChannel::new("sunspire-regatta").expect("operator channel");

        // 1. Positive standards-backed in-viewpoint prompt:
        let ok = ch
            .submit_directive(&OperatorPromptRequest {
                viewpoint: "creator-studio".to_string(),
                directive:
                    "Publish sealed .uorc FPS application to Stitch App Catalog under ISO 12207"
                        .to_string(),
                cited_authority: Some("AUTH-ISO-42010-VIEWPOINTS".to_string()),
                operator_prism_id: "prism:creator-alex".to_string(),
            })
            .expect("directive");
        assert!(ok.admitted);
        assert_eq!(ok.dialogue_act, "Inform");
        assert_eq!(ok.manager_id, "mgr-studio-alm");
        assert!(!ok.comm_kappa_hex.is_empty());

        // 2. Negative refusal: cross-contaminated Holo proof / UHD diagnostic inside Studio:
        let cross = ch
            .submit_directive(&OperatorPromptRequest {
                viewpoint: "creator-studio".to_string(),
                directive: "Add a Verify UHD 4K Envelope button to Studio".to_string(),
                cited_authority: None,
                operator_prism_id: "prism:creator-alex".to_string(),
            })
            .expect("cross refusal");
        assert!(!cross.admitted);
        assert_eq!(cross.dialogue_act, "Refusal");
        assert_eq!(
            cross.recommended_viewpoint.as_deref(),
            Some("verification-observatory")
        );

        // 3. Negative refusal: contrived hand-coded ungrounded feature:
        let contrived = ch
            .submit_directive(&OperatorPromptRequest {
                viewpoint: "playable-world".to_string(),
                directive: "Sprinkle random sparkly widgets everywhere".to_string(),
                cited_authority: None,
                operator_prism_id: "prism:creator-alex".to_string(),
            })
            .expect("contrived refusal");
        assert!(!contrived.admitted);
        assert_eq!(contrived.dialogue_act, "Refusal");

        // 4. Stitch 2-of-3 Verdant App Catalog publishing:
        let world = StandardSceneState::synthesize_at_lod(7, 2).expect("world");
        let uorc = WorldSnapshot::encode_uorc(&world).expect("uorc");
        let entry = ch
            .publish_stitch_app_catalog_entry(
                "Neon Rift FPS",
                "first-person-shooter",
                "High-definition arena shooter authored in Verdant Studio",
                "prism:creator-alex",
                &uorc,
            )
            .expect("publish stitch app");
        assert_eq!(entry.schema_org_type, "VideoGame");
        assert_eq!(entry.stitch_fragments.len(), 3);
        assert!(entry.app_iri.starts_with("urn:verdant:stitch-app:"));
    }
}
