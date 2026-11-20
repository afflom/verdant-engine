//! Model Context Protocol (MCP 2024-11-05, JSON-RPC 2.0 over stdio) handler for `verdant mcp`.

use super::*;

impl VerdantOperatorChannel {
    /// Standard Model Context Protocol (MCP `2024-11-05`, JSON-RPC 2.0 over stdio) request
    /// handler for `verdant mcp`. Returns `None` for notifications and `Some(json_line)` for
    /// requests.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn handle_mcp_jsonrpc_line(&mut self, repo_root: &Path, line: &str) -> Option<String> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }
        let kb = 1i64 << (1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1); // 1024
        let base32k = kb << (1 + 1 + 1 + 1 + 1); // 32768
        let sixty_four = 1i64 << (1 + 1 + 1 + 1 + 1 + 1);
        let err_parse = -(base32k - sixty_four - (1i64 << (1 + 1))); // -32700
        let err_method = -(base32k
            - (1i64 << (1 + 1 + 1 + 1 + 1 + 1 + 1))
            - (1i64 << (1 + 1 + 1 + 1 + 1))
            - (1i64 << (1 + 1))
            - (1i64 << 1)
            - 1); // -32601
        let err_params = err_method - 1; // -32602

        let parsed: serde_json::Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(e) => {
                return Some(
                    serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": serde_json::Value::Null,
                        "error": { "code": err_parse, "message": format!("Parse error: {e}") }
                    })
                    .to_string(),
                );
            }
        };
        let id = parsed.get("id").cloned();
        let method = parsed.get("method").and_then(|v| v.as_str()).unwrap_or("");
        let params = parsed
            .get("params")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));

        // JSON-RPC notifications (no `id`) expect no response:
        if id.is_none() || method.starts_with("notifications/") {
            return None;
        }
        let req_id = id.unwrap_or(serde_json::Value::Null);

        let ok_resp = |result: serde_json::Value| -> Option<String> {
            Some(
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": result,
                })
                .to_string(),
            )
        };
        let err_resp = |code: i64, msg: String| -> Option<String> {
            Some(
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "error": { "code": code, "message": msg },
                })
                .to_string(),
            )
        };

        match method {
            "initialize" => ok_resp(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": { "listChanged": false },
                    "resources": { "subscribe": false, "listChanged": false }
                },
                "serverInfo": {
                    "name": "verdant-engine-mcp",
                    "version": "0.1.0"
                }
            })),
            "ping" => ok_resp(serde_json::json!({})),
            "tools/list" => ok_resp(serde_json::json!({
                "tools": [
                    {
                        "name": "verdant_status",
                        "description": "Inspect all 8 VSM S1-S5 Managers, Total Ownership status, and View Immutability Seal health.",
                        "inputSchema": { "type": "object", "properties": {} }
                    },
                    {
                        "name": "verdant_prompt",
                        "description": "Submit a signed ISO 24617-2 directive to Verdant's Observer Executive and owning S1 Manager.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "viewpoint": { "type": "string" },
                                "directive": { "type": "string" },
                                "authority": { "type": "string" },
                                "actor": { "type": "string" }
                            },
                            "required": ["directive"]
                        }
                    },
                    {
                        "name": "verdant_synthesize",
                        "description": "Synthesize all Manager-owned declarative manifests in site/generated/ and update the cryptographic View Immutability Seal.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "viewpoint": { "type": "string" }
                            }
                        }
                    },
                    {
                        "name": "verdant_mutate_view",
                        "description": "The ONLY sanctioned path to modify a Manager-owned declarative view or file. Enforces AUTH-* citation, Total Ownership, DTCG hygiene, ISO 42010 purity, re-sealing, and S3* audit with rollback.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "viewpoint": { "type": "string" },
                                "directive": { "type": "string" },
                                "citedAuthority": { "type": "string" },
                                "path": { "type": "string" },
                                "mode": { "type": "string", "enum": ["replace_exact", "write_full"] },
                                "target": { "type": "string" },
                                "replacement": { "type": "string" },
                                "content": { "type": "string" },
                                "operatorPrismId": { "type": "string" }
                            },
                            "required": ["viewpoint", "directive", "citedAuthority", "path", "mode"]
                        }
                    },
                    {
                        "name": "verdant_publish_app",
                        "description": "Seal a multi-genre Verdant App as .uorc, shard it via afflom/stitch 2-of-3 parity, publish into the Verdant App Catalog, and re-seal immutability.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "title": { "type": "string" },
                                "genre": { "type": "string" },
                                "domain": { "type": "string" },
                                "summary": { "type": "string" },
                                "actor": { "type": "string" }
                            },
                            "required": ["title", "genre", "domain"]
                        }
                    },
                    {
                        "name": "verdant_verify_immutability",
                        "description": "Cryptographically verify every sealed Verdant view and generated artifact against site/generated/immutability-seal.json to detect any manual out-of-band edit.",
                        "inputSchema": { "type": "object", "properties": {} }
                    },
                    {
                        "name": "verdant_audit",
                        "description": "Run the VSM S3* independent repository audit across all 8 Managers, Total Ownership, 4-world differentiation, and cryptographic View Immutability Seal.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "viewpoint": { "type": "string" }
                            }
                        }
                    },
                    {
                        "name": "verdant_stitch_seal",
                        "description": "Zero-account Prism-ID Encrypt-then-MAC boundary seal (PrivateEncrypted, SelectiveShare, or Public) + systematic k-of-n Reed-Solomon GF(2^8) Stitch sharding (default 4-of-8) under AUTH-STITCH-SELF-HOSTING.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "boundaryId": { "type": "string" },
                                "ownerPrismId": { "type": "string" },
                                "visibility": { "type": "string", "enum": ["PrivateEncrypted", "SelectiveShare", "Public"] },
                                "secretPassphrase": { "type": "string" },
                                "payloadUtf8": { "type": "string" },
                                "dataShards": { "type": "integer" },
                                "totalShards": { "type": "integer" }
                            },
                            "required": ["boundaryId", "ownerPrismId", "payloadUtf8"]
                        }
                    },
                    {
                        "name": "verdant_stitch_recover",
                        "description": "Recover and authenticate a StitchBoundaryEnvelope from any >= k Reed-Solomon shards, enforcing Encrypt-then-MAC integrity and boundary epoch checks.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "boundaryId": { "type": "string" },
                                "ownerPrismId": { "type": "string" },
                                "secretPassphrase": { "type": "string" },
                                "shards": { "type": "array" }
                            },
                            "required": ["boundaryId", "ownerPrismId", "shards"]
                        }
                    },
                    {
                        "name": "verdant_vcs_commit",
                        "description": "Create an acyclic W3C PROV-O Merkle revision commit inside VerdantVcs from repo-relative files or inline blobs, signed with Prism ID and citing an AUTH-* authority, returning the sealed .uorc VCS capsule summary.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "refName": { "type": "string" },
                                "authorPrismId": { "type": "string" },
                                "managerId": { "type": "string" },
                                "message": { "type": "string" },
                                "citedAuthority": { "type": "string" },
                                "paths": { "type": "array", "items": { "type": "string" } },
                                "blobs": { "type": "object" }
                            },
                            "required": ["authorPrismId", "message", "citedAuthority"]
                        }
                    },
                    {
                        "name": "verdant_release_pack",
                        "description": "Lift verified Verdant Viewpoints, manifests, and Wasm into a self-verifying portable verdant-vX.Y.Z.holo release capsule, 4-of-8 STCH2 Reed-Solomon shards, VerdantVcs .uorc DAG, and quorum-signed frame-pin.json.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "version": { "type": "string" },
                                "epoch": { "type": "integer" },
                                "outDir": { "type": "string" },
                                "defaultStitchUri": { "type": "string" },
                                "stitchRelays": { "type": "array", "items": { "type": "string" } }
                            },
                            "required": ["version", "outDir"]
                        }
                    },
                    {
                        "name": "verdant_change_evaluate",
                        "description": "Observer queries the Change & Version Manager (mgr-change-release) to evaluate Ratchet + Release Gates and determine Major, Minor, Patch, or Hold release bump under SemVer 2.0.0 (AUTH-SEMVER-CHANGE-KPI).",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "ratchetApplied": { "type": "boolean" },
                                "pullThroughVerified": { "type": "boolean" },
                                "breakingContractChange": { "type": "boolean" },
                                "majorMilestoneCompleted": { "type": "boolean" },
                                "minorCapabilityAdded": { "type": "boolean" },
                                "stakeholderRequirementClosed": { "type": "boolean" },
                                "defectOrSecurityFix": { "type": "boolean" },
                                "summary": { "type": "string" }
                            }
                        }
                    },
                    {
                        "name": "verdant_change_advance",
                        "description": "Advance Verdant's semantic version and release epoch via mgr-change-release onto site/generated/release-ledger.json and re-seal immutability (refuses if Ratchet is not applied or any Release Gate fails).",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "ratchetApplied": { "type": "boolean" },
                                "pullThroughVerified": { "type": "boolean" },
                                "breakingContractChange": { "type": "boolean" },
                                "majorMilestoneCompleted": { "type": "boolean" },
                                "minorCapabilityAdded": { "type": "boolean" },
                                "stakeholderRequirementClosed": { "type": "boolean" },
                                "defectOrSecurityFix": { "type": "boolean" },
                                "summary": { "type": "string" }
                            },
                            "required": ["summary"]
                        }
                    },
                    {
                        "name": "verdant_observer_roadmap_kpis",
                        "description": "Observer queries all 13 VSM Managers, Observer parts, Knowledge Manager, and Roadmap/KPI Manager for live ISO 15939 KPIs, Major/Minor stakeholder roadmap progress, and release readiness.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "ratchetApplied": { "type": "boolean" },
                                "pullThroughVerified": { "type": "boolean" }
                            }
                        }
                    },
                    {
                        "name": "verdant_self_host_lift",
                        "description": "Lift 100% of Verdant's workspace (shipped Rust crates, all 26 Prism LexLean strata, governance/model catalogs, and distilled Viewpoints) into a 4-of-8 STCH2 sharded VerdantVcs Stitch capsule enforcing max_unlifted_workspace_modules = 0 (VE-46, M5).",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "version": { "type": "string" },
                                "epoch": { "type": "integer" },
                                "outDir": { "type": "string" }
                            },
                            "required": ["outDir"]
                        }
                    },
                    {
                        "name": "verdant_lifecycle_gate",
                        "description": "Execute Verdant's lifted self-verification, test-execution, and release-qualification lifecycle directly from within Verdant (mgr-verification-observatory & mgr-change-release), returning LifecycleGateReport JSON.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "runWorkspaceTests": { "type": "boolean" },
                                "runBrowserGates": { "type": "boolean" },
                                "expectedTag": { "type": "string" },
                                "writeReport": { "type": "boolean" }
                            }
                        }
                    },
                    {
                        "name": "verdant_observer_refine_plan",
                        "description": "Generatively diagnose all 13 VSM Managers, owned source files, frontier.toml blockers, fidelity closure, Wasm/capsule weights, and Blank Frame headroom, returning the content-addressed ObserverRefinementAgenda.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
                    },
                    {
                        "name": "verdant_automate",
                        "description": "Execute Verdant's full lifted self-automation & verification pipeline directly from within Verdant (kappa-memoized Wasm build, in-process ratchet-lift, site assembly, Cargo Sparse Registry packaging, in-process lifecycle-gate, and live browser/contract gates).",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "skipRust": { "type": "boolean" },
                                "forceWasm": { "type": "boolean" }
                            }
                        }
                    },
                    {
                        "name": "verdant_ratchet_cycle",
                        "description": "Execute Verdant's full lifted build/refresh/ratchet-lift/tighten/verify cycle directly from within Verdant (xtask check-model, prism-fidelity, kappa-memoized LexLean/uorust/Wasm, in-process ratchet-lift --tighten, prism-fidelity --check, and optional lifecycle-gate).",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "force": { "type": "boolean" },
                                "tighten": { "type": "boolean" },
                                "runSdkDocker": { "type": "boolean" },
                                "runLifecycleGate": { "type": "boolean" },
                                "runBrowserGates": { "type": "boolean" },
                                "advanceMajor": { "type": "boolean" },
                                "advanceMinor": { "type": "boolean" },
                                "advancePatch": { "type": "boolean" },
                                "summary": { "type": "string" }
                            }
                        }
                    },
                    {
                        "name": "verdant_ratchet_lift",
                        "description": "Autonomous closed-loop Verdant self-lift (VE-47): verifies 13-Manager Total Ownership & <=30KiB zero-account Blank Frame, synthesizes all Manager manifests, optionally advances SemVer via mgr-change-release, lifts 100% of Rust/LexLean/Viewpoint modules into VerdantVcs, packs & 4-of-8 verifies verdant-vX.Y.Z.holo, pins site/frame/frame-pin.json, and emits site/generated/self-host-ratchet-manifest.toml to satisfy prism/parametric-ratchet/2.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "outDir": { "type": "string" },
                                "writeRepoArtifacts": { "type": "boolean" },
                                "updateFramePin": { "type": "boolean" },
                                "stitchRelays": { "type": "array", "items": { "type": "string" } },
                                "advanceMajor": { "type": "boolean" },
                                "advanceMinor": { "type": "boolean" },
                                "advancePatch": { "type": "boolean" },
                                "summary": { "type": "string" }
                            }
                        }
                    }
                ]
            })),
            "tools/call" => {
                let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = params
                    .get("arguments")
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!({}));
                let tool_out = |is_err: bool, payload: serde_json::Value| -> Option<String> {
                    let txt =
                        serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "{}".to_string());
                    ok_resp(serde_json::json!({
                        "content": [{ "type": "text", "text": txt }],
                        "isError": is_err,
                    }))
                };
                match tool_name {
                    "verdant_status" => {
                        let st: serde_json::Value = serde_json::from_str(&self.status_json())
                            .unwrap_or(serde_json::Value::Null);
                        let imm = self.verify_immutability_seal(repo_root);
                        tool_out(
                            false,
                            serde_json::json!({ "status": st, "immutability": imm }),
                        )
                    }
                    "verdant_prompt" => {
                        let req = OperatorPromptRequest {
                            viewpoint: args
                                .get("viewpoint")
                                .and_then(|v| v.as_str())
                                .unwrap_or("auto")
                                .to_string(),
                            directive: args
                                .get("directive")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            cited_authority: args
                                .get("authority")
                                .and_then(|v| v.as_str())
                                .map(str::to_string),
                            operator_prism_id: args
                                .get("actor")
                                .and_then(|v| v.as_str())
                                .unwrap_or("prism:verdant-mcp")
                                .to_string(),
                        };
                        match self.submit_directive(&req) {
                            Ok(r) => tool_out(
                                !r.admitted,
                                serde_json::to_value(&r).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_synthesize" => {
                        let vp = args.get("viewpoint").and_then(|v| v.as_str());
                        match self.materialize_manager_artifacts(repo_root, vp) {
                            Ok(r) => tool_out(
                                !r.ok,
                                serde_json::to_value(&r).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_mutate_view" => {
                        match serde_json::from_value::<OwnedFileMutationRequest>(args) {
                            Ok(req) => match self.mutate_owned_file(repo_root, &req) {
                                Ok(r) => tool_out(
                                    !r.mutated,
                                    serde_json::to_value(&r).unwrap_or(serde_json::Value::Null),
                                ),
                                Err(e) => {
                                    tool_out(true, serde_json::json!({ "error": e.to_string() }))
                                }
                            },
                            Err(e) => {
                                err_resp(err_params, format!("invalid mutate_view args: {e}"))
                            }
                        }
                    }
                    "verdant_publish_app" => {
                        let title = args
                            .get("title")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Untitled Verdant App");
                        let genre = args
                            .get("genre")
                            .and_then(|v| v.as_str())
                            .unwrap_or("coastal-adventure");
                        let domain = args
                            .get("domain")
                            .and_then(|v| v.as_str())
                            .unwrap_or("sunspire-regatta");
                        let summary = args
                            .get("summary")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Published via Verdant MCP");
                        let actor = args
                            .get("actor")
                            .and_then(|v| v.as_str())
                            .unwrap_or("prism:verdant-mcp");
                        match self.publish_app_to_repo_catalog(
                            repo_root, title, genre, domain, summary, actor,
                        ) {
                            Ok(app) => tool_out(
                                false,
                                serde_json::to_value(&app).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_verify_immutability" => {
                        let rep = self.verify_immutability_seal(repo_root);
                        tool_out(
                            !rep.valid,
                            serde_json::to_value(&rep).unwrap_or(serde_json::Value::Null),
                        )
                    }
                    "verdant_audit" => {
                        let vp = args.get("viewpoint").and_then(|v| v.as_str());
                        let rep = self.audit_repository(repo_root, vp);
                        tool_out(
                            !rep.passed,
                            serde_json::to_value(&rep).unwrap_or(serde_json::Value::Null),
                        )
                    }
                    "verdant_stitch_seal" => {
                        let boundary_id = args
                            .get("boundaryId")
                            .and_then(|v| v.as_str())
                            .unwrap_or("stitch://boundary/default");
                        let owner = args
                            .get("ownerPrismId")
                            .and_then(|v| v.as_str())
                            .unwrap_or("prism:anonymous-stakeholder");
                        let visibility = match args
                            .get("visibility")
                            .and_then(|v| v.as_str())
                            .unwrap_or("PrivateEncrypted")
                        {
                            "SelectiveShare" => BoundaryVisibility::SelectiveShare,
                            "Public" => BoundaryVisibility::Public,
                            _ => BoundaryVisibility::PrivateEncrypted,
                        };
                        let passphrase = args
                            .get("secretPassphrase")
                            .and_then(|v| v.as_str())
                            .unwrap_or("prism-zero-account-local-root");
                        let payload = args
                            .get("payloadUtf8")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let k = args
                            .get("dataShards")
                            .and_then(|v| v.as_u64())
                            .and_then(|v| u8::try_from(v).ok())
                            .unwrap_or(RELEASE_RS_DATA_SHARDS);
                        let n = args
                            .get("totalShards")
                            .and_then(|v| v.as_u64())
                            .and_then(|v| u8::try_from(v).ok())
                            .unwrap_or(RELEASE_RS_TOTAL_SHARDS);
                        let res = BoundaryKeyring::new(boundary_id, owner, passphrase.as_bytes())
                            .and_then(|keyring| {
                                PrismBoundaryCipher::seal(
                                    &keyring,
                                    owner,
                                    visibility,
                                    payload.as_bytes(),
                                    boundary_id.as_bytes(),
                                )
                            })
                            .and_then(|env| {
                                let bytes = serde_json::to_vec(&env).unwrap_or_default();
                                let shards = ReedSolomonStitchCodec::shard_k_of_n(&bytes, k, n)?;
                                Ok((env, shards))
                            });
                        match res {
                            Ok((env, shards)) => tool_out(
                                false,
                                serde_json::json!({
                                    "boundaryId": env.boundary_id,
                                    "ownerPrismId": env.owner_prism_id,
                                    "visibility": env.visibility,
                                    "boundaryEpoch": env.boundary_epoch,
                                    "dataShards": k,
                                    "totalShards": n,
                                    "shards": shards,
                                }),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_stitch_recover" => {
                        let boundary_id = args
                            .get("boundaryId")
                            .and_then(|v| v.as_str())
                            .unwrap_or("stitch://boundary/default");
                        let owner = args
                            .get("ownerPrismId")
                            .and_then(|v| v.as_str())
                            .unwrap_or("prism:anonymous-stakeholder");
                        let passphrase = args
                            .get("secretPassphrase")
                            .and_then(|v| v.as_str())
                            .unwrap_or("prism-zero-account-local-root");
                        let shards: Option<Vec<StitchRsShard>> = args
                            .get("shards")
                            .cloned()
                            .and_then(|v| serde_json::from_value(v).ok());
                        match shards {
                            Some(shard_vec) => {
                                let res = BoundaryKeyring::new(
                                    boundary_id,
                                    owner,
                                    passphrase.as_bytes(),
                                )
                                .and_then(|keyring| {
                                    let raw = ReedSolomonStitchCodec::recover_k_of_n(&shard_vec)?;
                                    let env: StitchBoundaryEnvelope =
                                        serde_json::from_slice(&raw).map_err(|e| {
                                            crate::error::StitchCustodyError::PayloadDigestMismatch {
                                                detail: e.to_string(),
                                            }
                                        })?;
                                    let plain = PrismBoundaryCipher::open(&keyring, owner, &env)?;
                                    Ok((env, plain))
                                });
                                match res {
                                    Ok((env, plain)) => tool_out(
                                        false,
                                        serde_json::json!({
                                            "boundaryId": env.boundary_id,
                                            "ownerPrismId": env.owner_prism_id,
                                            "visibility": env.visibility,
                                            "boundaryEpoch": env.boundary_epoch,
                                            "payloadUtf8": String::from_utf8_lossy(&plain),
                                            "payloadBytes": plain.len(),
                                        }),
                                    ),
                                    Err(e) => tool_out(
                                        true,
                                        serde_json::json!({ "error": e.to_string() }),
                                    ),
                                }
                            }
                            None => tool_out(
                                true,
                                serde_json::json!({ "error": "missing or invalid shards array" }),
                            ),
                        }
                    }
                    "verdant_vcs_commit" => {
                        let ref_name = args
                            .get("refName")
                            .and_then(|v| v.as_str())
                            .unwrap_or("refs/heads/main");
                        let author = args
                            .get("authorPrismId")
                            .and_then(|v| v.as_str())
                            .unwrap_or("prism:verdant-mcp");
                        let manager_id = args
                            .get("managerId")
                            .and_then(|v| v.as_str())
                            .unwrap_or("mgr-observer-executive");
                        let message = args
                            .get("message")
                            .and_then(|v| v.as_str())
                            .unwrap_or("VerdantVcs declarative commit");
                        let auth = args
                            .get("citedAuthority")
                            .and_then(|v| v.as_str())
                            .unwrap_or("AUTH-STITCH-SELF-HOSTING");
                        let mut vcs = VerdantVcs::new();
                        let mut entries: BTreeMap<String, Vec<u8>> = BTreeMap::new();
                        if let Some(paths) = args.get("paths").and_then(|v| v.as_array()) {
                            for p in paths.iter().filter_map(|v| v.as_str()) {
                                if !p.contains("..") && !p.starts_with('/') {
                                    if let Ok(bytes) = std::fs::read(repo_root.join(p)) {
                                        entries.insert(p.to_string(), bytes);
                                    }
                                }
                            }
                        }
                        if let Some(blobs) = args.get("blobs").and_then(|v| v.as_object()) {
                            for (k, v) in blobs {
                                if let Some(s) = v.as_str() {
                                    entries.insert(k.clone(), s.as_bytes().to_vec());
                                }
                            }
                        }
                        match vcs
                            .commit_tree(
                                ref_name,
                                &entries,
                                author,
                                b"verdant-vcs-mcp-signing-root",
                                manager_id,
                                auth,
                                message,
                            )
                            .and_then(|commit| {
                                let uorc = vcs.encode_uorc_capsule()?;
                                Ok((commit, uorc.len()))
                            }) {
                            Ok((commit, uorc_bytes)) => tool_out(
                                false,
                                serde_json::json!({
                                    "commitKappa": commit.commit_kappa,
                                    "treeRootSha256": commit.tree_root_sha256,
                                    "epoch": commit.epoch,
                                    "authorPrismId": commit.author_prism_id,
                                    "managerId": commit.manager_id,
                                    "citedAuthority": commit.cited_authority,
                                    "message": commit.message,
                                    "entryCount": commit.tree_entries.len(),
                                    "uorcBytes": uorc_bytes,
                                }),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_release_pack" => {
                        let version = args
                            .get("version")
                            .and_then(|v| v.as_str())
                            .unwrap_or("0.1.0");
                        let epoch = args.get("epoch").and_then(|v| v.as_u64()).unwrap_or(1);
                        let out_dir_str = args
                            .get("outDir")
                            .and_then(|v| v.as_str())
                            .unwrap_or("dist/stitch-release");
                        let default_uri = args
                            .get("defaultStitchUri")
                            .and_then(|v| v.as_str())
                            .unwrap_or("stitch://verdant/studio");
                        let relays: Vec<String> = args
                            .get("stitchRelays")
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_str().map(str::to_string))
                                    .collect()
                            })
                            .unwrap_or_default();
                        let out_path = if out_dir_str.starts_with('/') {
                            std::path::PathBuf::from(out_dir_str)
                        } else {
                            repo_root.join(out_dir_str)
                        };
                        match self.pack_repo_release_capsule(
                            repo_root,
                            version,
                            epoch,
                            &out_path,
                            relays,
                            default_uri,
                        ) {
                            Ok(pin) => tool_out(
                                false,
                                serde_json::to_value(&pin).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_change_evaluate" | "verdant_change_advance" => {
                        let ratchet_applied = args
                            .get("ratchetApplied")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let pull_through = args
                            .get("pullThroughVerified")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let signals = ObserverProgressSignals {
                            breaking_contract_change: args
                                .get("breakingContractChange")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            major_milestone_completed: args
                                .get("majorMilestoneCompleted")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            minor_capability_added: args
                                .get("minorCapabilityAdded")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            stakeholder_requirement_closed: args
                                .get("stakeholderRequirementClosed")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            defect_or_security_fix: args
                                .get("defectOrSecurityFix")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            summary: args
                                .get("summary")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Observer Change Manager evaluation")
                                .to_string(),
                        };
                        let res = if tool_name == "verdant_change_advance" {
                            self.advance_release_version(
                                repo_root,
                                ratchet_applied,
                                pull_through,
                                &signals,
                            )
                        } else {
                            self.evaluate_change_release(
                                repo_root,
                                ratchet_applied,
                                pull_through,
                                &signals,
                            )
                        };
                        match res {
                            Ok(dec) => tool_out(
                                false,
                                serde_json::to_value(&dec).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_observer_roadmap_kpis" => {
                        let ratchet_applied = args
                            .get("ratchetApplied")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true);
                        let pull_through = args
                            .get("pullThroughVerified")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true);
                        match self.evaluate_observer_roadmap_kpis(
                            repo_root,
                            ratchet_applied,
                            pull_through,
                        ) {
                            Ok(rep) => tool_out(
                                false,
                                serde_json::to_value(&rep).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_self_host_lift" => {
                        let ledger = self.load_or_init_release_ledger(repo_root).ok();
                        let default_ver = ledger
                            .as_ref()
                            .map(|l| l.current_version.display_string())
                            .unwrap_or_else(|| "1.0.0".to_string());
                        let default_ep = ledger.as_ref().map(|l| l.release_epoch).unwrap_or(1);
                        let version = args
                            .get("version")
                            .and_then(|v| v.as_str())
                            .unwrap_or(&default_ver);
                        let epoch = args
                            .get("epoch")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(default_ep);
                        let out_dir_str = args
                            .get("outDir")
                            .and_then(|v| v.as_str())
                            .unwrap_or("dist/stitch-release");
                        let out_path = if out_dir_str.starts_with('/') {
                            std::path::PathBuf::from(out_dir_str)
                        } else {
                            repo_root.join(out_dir_str)
                        };
                        match self
                            .lift_full_workspace_into_vcs(repo_root, &out_path, version, epoch)
                        {
                            Ok(rep) => tool_out(
                                false,
                                serde_json::to_value(&rep).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_automate" => {
                        let opts = AutomateOptions {
                            skip_rust: args
                                .get("skipRust")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            force_wasm: args
                                .get("forceWasm")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                        };
                        match self.execute_automate_pipeline(repo_root, &opts) {
                            Ok((rep, _)) => tool_out(
                                !rep.all_passed,
                                serde_json::to_value(&rep).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_ratchet_cycle" => {
                        let adv_major = args
                            .get("advanceMajor")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let adv_minor = args
                            .get("advanceMinor")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let adv_patch = args
                            .get("advancePatch")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let advance_signals = if adv_major || adv_minor || adv_patch {
                            Some(ObserverProgressSignals {
                                breaking_contract_change: adv_major,
                                major_milestone_completed: adv_major,
                                minor_capability_added: adv_minor,
                                stakeholder_requirement_closed: adv_minor,
                                defect_or_security_fix: adv_patch,
                                summary: args
                                    .get("summary")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("MCP lifted ratchet-cycle")
                                    .to_string(),
                            })
                        } else {
                            None
                        };
                        let opts = RatchetCycleOptions {
                            force: args.get("force").and_then(|v| v.as_bool()).unwrap_or(false),
                            tighten: args
                                .get("tighten")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(true),
                            run_sdk_docker: args
                                .get("runSdkDocker")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(true),
                            run_lifecycle_gate: args
                                .get("runLifecycleGate")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            run_browser_gates: args
                                .get("runBrowserGates")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            advance_signals,
                        };
                        match self.execute_ratchet_cycle(repo_root, &opts) {
                            Ok((rep, _)) => tool_out(
                                !rep.all_passed,
                                serde_json::to_value(&rep).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_lifecycle_gate" => {
                        let opts = LifecycleGateOptions {
                            run_workspace_tests: args
                                .get("runWorkspaceTests")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            run_browser_gates: args
                                .get("runBrowserGates")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            expected_tag: args
                                .get("expectedTag")
                                .and_then(|v| v.as_str())
                                .map(str::to_string),
                            write_report: args
                                .get("writeReport")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                        };
                        match self.execute_lifecycle_gate(repo_root, &opts) {
                            Ok((rep, _)) => tool_out(
                                !rep.release_qualified,
                                serde_json::to_value(&rep).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_observer_refine_plan" => {
                        match self.synthesize_refinement_agenda(repo_root) {
                            Ok((agenda, _)) => tool_out(
                                false,
                                serde_json::to_value(&agenda).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    "verdant_ratchet_lift" => {
                        let out_dir_str = args
                            .get("outDir")
                            .and_then(|v| v.as_str())
                            .unwrap_or("dist/stitch-release");
                        let out_path = if out_dir_str.starts_with('/') {
                            std::path::PathBuf::from(out_dir_str)
                        } else {
                            repo_root.join(out_dir_str)
                        };
                        let write_repo_artifacts = args
                            .get("writeRepoArtifacts")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true);
                        let update_frame_pin = args
                            .get("updateFramePin")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true);
                        let tighten_ratchet = args
                            .get("tightenRatchet")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(write_repo_artifacts);
                        let stitch_relays: Vec<String> = args
                            .get("stitchRelays")
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_str().map(str::to_string))
                                    .collect()
                            })
                            .unwrap_or_default();
                        let adv_major = args
                            .get("advanceMajor")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let adv_minor = args
                            .get("advanceMinor")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let adv_patch = args
                            .get("advancePatch")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let advance_signals = if adv_major || adv_minor || adv_patch {
                            Some(ObserverProgressSignals {
                                breaking_contract_change: adv_major,
                                major_milestone_completed: adv_major,
                                minor_capability_added: adv_minor,
                                stakeholder_requirement_closed: adv_minor,
                                defect_or_security_fix: adv_patch,
                                summary: args
                                    .get("summary")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or(
                                        "Autonomous Verdant self-lift via verdant_ratchet_lift",
                                    )
                                    .to_string(),
                            })
                        } else {
                            None
                        };
                        let opts = AutonomousRatchetLiftOptions {
                            write_repo_artifacts,
                            update_frame_pin,
                            tighten_ratchet,
                            stitch_relays,
                            advance_signals,
                        };
                        match self.execute_autonomous_ratchet_lift(repo_root, &out_path, &opts) {
                            Ok(rep) => tool_out(
                                false,
                                serde_json::to_value(&rep).unwrap_or(serde_json::Value::Null),
                            ),
                            Err(e) => tool_out(true, serde_json::json!({ "error": e.to_string() })),
                        }
                    }
                    other => err_resp(err_params, format!("unknown MCP tool `{other}`")),
                }
            }
            "resources/list" => ok_resp(serde_json::json!({
                "resources": [
                    { "uri": "verdant://governance/vsm", "name": "VSM Cybernetics Authority (AUTH-VSM-CYBERNETICS)", "mimeType": "application/json" },
                    { "uri": "verdant://governance/viewpoints", "name": "ISO 42010 Viewpoints & Managers (AUTH-ISO-42010-VIEWPOINTS)", "mimeType": "application/json" },
                    { "uri": "verdant://governance/tensor-scaling", "name": "ISO 25010 & ITU-R BT.2020/2100 Tensor Scaling Authority (AUTH-ISO-25010-TENSOR-SCALING)", "mimeType": "application/json" },
                    { "uri": "verdant://governance/stitch-self-hosting", "name": "Zero-Account Stitch Self-Hosting & Boundary Privacy (AUTH-STITCH-SELF-HOSTING)", "mimeType": "application/json" },
                    { "uri": "verdant://governance/change-roadmap-kpi", "name": "SemVer Change Manager, Stakeholder Roadmap & ISO 15939 KPIs (AUTH-SEMVER-CHANGE-KPI)", "mimeType": "application/json" },
                    { "uri": "verdant://generated/viewpoint-contracts", "name": "Manager Viewpoint Contracts", "mimeType": "application/json" },
                    { "uri": "verdant://generated/experience-design-system", "name": "Design System & Communications Spec", "mimeType": "application/json" },
                    { "uri": "verdant://generated/world-differentiation-profiles", "name": "4-World Differentiation Profiles", "mimeType": "application/json" },
                    { "uri": "verdant://generated/tensor-scaling-contract", "name": "ISO 25010 Tensor Scaling & Paged Memory Contract", "mimeType": "application/json" },
                    { "uri": "verdant://generated/stitch-app-catalog", "name": "Stitch 2-of-3 Verdant App Catalog", "mimeType": "application/json" },
                    { "uri": "verdant://generated/release-ledger", "name": "Change & Version Manager SemVer Release Ledger", "mimeType": "application/json" },
                    { "uri": "verdant://generated/roadmap-kpi-report", "name": "Observer Stakeholder Roadmap & KPI Telemetry Report", "mimeType": "application/json" },
                    { "uri": "verdant://generated/immutability-seal", "name": "Cryptographic View Immutability Seal", "mimeType": "application/json" },
                    { "uri": "verdant://generated/self-host-ratchet-manifest", "name": "Self-Hosting Parametric Ratchet Manifest (TOML)", "mimeType": "application/toml" }
                ]
            })),
            "resources/read" => {
                let uri = params.get("uri").and_then(|v| v.as_str()).unwrap_or("");
                let rel_opt = match uri {
                    "verdant://governance/vsm" => {
                        Some("catalogs/governance/vsm-viable-system.json")
                    }
                    "verdant://governance/viewpoints" => {
                        Some("catalogs/governance/iso42010-viewpoints.json")
                    }
                    "verdant://governance/tensor-scaling" => {
                        Some("catalogs/governance/iso25010-tensor-scaling.json")
                    }
                    "verdant://governance/stitch-self-hosting" => {
                        Some("catalogs/governance/stitch-self-hosting.json")
                    }
                    "verdant://governance/change-roadmap-kpi" => {
                        Some("catalogs/governance/change-roadmap-kpi.json")
                    }
                    "verdant://generated/viewpoint-contracts" => {
                        Some("site/generated/viewpoint-contracts.json")
                    }
                    "verdant://generated/experience-design-system" => {
                        Some("site/generated/experience-design-system.json")
                    }
                    "verdant://generated/world-differentiation-profiles" => {
                        Some("site/generated/world-differentiation-profiles.json")
                    }
                    "verdant://generated/tensor-scaling-contract" => {
                        Some("site/generated/tensor-scaling-contract.json")
                    }
                    "verdant://generated/stitch-app-catalog" => {
                        Some("site/generated/stitch-app-catalog.json")
                    }
                    "verdant://generated/release-ledger" => {
                        Some("site/generated/release-ledger.json")
                    }
                    "verdant://generated/roadmap-kpi-report" => {
                        Some("site/generated/roadmap-kpi-report.json")
                    }
                    "verdant://generated/immutability-seal" => {
                        Some("site/generated/immutability-seal.json")
                    }
                    "verdant://generated/self-host-ratchet-manifest" => {
                        Some("site/generated/self-host-ratchet-manifest.toml")
                    }
                    _ => None,
                };
                match rel_opt.and_then(|rel| std::fs::read_to_string(repo_root.join(rel)).ok()) {
                    Some(text) => ok_resp(serde_json::json!({
                        "contents": [{ "uri": uri, "mimeType": "application/json", "text": text }]
                    })),
                    None => err_resp(
                        err_params,
                        format!("unknown or unreadable resource `{uri}`"),
                    ),
                }
            }
            other => err_resp(err_method, format!("Method not found: `{other}`")),
        }
    }
}
