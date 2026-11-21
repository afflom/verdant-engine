//! Unified Wasm / CLI / MCP JSON kernel bridge and [`ApprovalCertificate`] for
//! `SovereignAuthorityLedger` (`VE-72`) so Browser Studio, CLI, and MCP execute the exact same
//! Rust governance kernel.

use serde::{Deserialize, Serialize};

use super::{AuthorityCommand, SigningActionKind, SovereignAuthorityLedger};

/// Content-addressed cryptographic approval certificate binding a finalized signing request
/// (`SovereignAuthorityLedger`) directly into `VerdantVcs` commits and `.holo` release capsules (`VE-72`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalCertificate {
    /// Fixed schema identifier (`verdant/approval-certificate/1`).
    pub spec: String,
    /// Finalized signing request `kappa`.
    pub request_kappa: String,
    /// Target `VerdantVcs` commit/tree or `.holo` release `kappa` authorized by this certificate.
    pub target_kappa: String,
    /// Authorized privileged action kind (`MergeCommit`, `CutRelease`, etc.).
    pub action: SigningActionKind,
    /// Authority epoch at which the signing request was finalized.
    pub finalized_epoch: u64,
    /// Active M-of-N quorum threshold satisfied at `finalized_epoch`.
    pub quorum_threshold_m: u64,
    /// Sorted list of distinct active Sovereign Admin `Prism ID`s that signed approval.
    pub approver_prism_ids: Vec<String>,
    /// Deterministic SHA-256 hex commitment over all certificate fields.
    pub certificate_kappa: String,
}

/// Executes a sovereign governance JSON request across Wasm, CLI, and MCP (`VE-72`):
/// accepts `{ "ledger": <optional>, "init": { "repoName", "ownerPrismId", "ownerSecret" }, "command": <optional AuthorityCommand> }`
/// and returns `{ "ok": true, "ledger": ..., "outcome": ... }`.
#[must_use]
pub fn sovereign_authority_exec_json(raw_json: &str) -> String {
    let Ok(req) = serde_json::from_str::<serde_json::Value>(raw_json) else {
        return serde_json::json!({ "ok": false, "error": "invalid authority JSON request" })
            .to_string();
    };
    let mut ledger = if let Some(lval) = req.get("ledger").filter(|v| !v.is_null()) {
        match serde_json::from_value::<SovereignAuthorityLedger>(lval.clone()) {
            Ok(l) => l,
            Err(e) => {
                return serde_json::json!({ "ok": false, "error": format!("parse ledger: {e}") })
                    .to_string();
            }
        }
    } else if let Some(init) = req.get("init").filter(|v| v.is_object()) {
        let repo = init
            .get("repoName")
            .and_then(|v| v.as_str())
            .unwrap_or("verdant-studio-project");
        let owner = init
            .get("ownerPrismId")
            .and_then(|v| v.as_str())
            .unwrap_or("prism:studio:genesis-owner");
        let secret = init
            .get("ownerSecret")
            .and_then(|v| v.as_str())
            .unwrap_or("studio-local-genesis-secret");
        match SovereignAuthorityLedger::init_genesis(repo, owner, secret) {
            Ok(l) => l,
            Err(e) => {
                return serde_json::json!({ "ok": false, "error": e.to_string() }).to_string();
            }
        }
    } else {
        return serde_json::json!({ "ok": false, "error": "expected `ledger` or `init` object" })
            .to_string();
    };

    if let Some(cmd_val) = req.get("command").filter(|v| !v.is_null()) {
        let cmd: AuthorityCommand = match serde_json::from_value(cmd_val.clone()) {
            Ok(c) => c,
            Err(e) => {
                return serde_json::json!({ "ok": false, "error": format!("parse command: {e}") })
                    .to_string();
            }
        };
        match ledger.execute(cmd) {
            Ok(outcome) => serde_json::json!({
                "ok": true,
                "ledger": ledger,
                "outcome": outcome,
            })
            .to_string(),
            Err(e) => serde_json::json!({
                "ok": false,
                "error": e.to_string(),
                "ledger": ledger,
            })
            .to_string(),
        }
    } else {
        serde_json::json!({
            "ok": true,
            "ledger": ledger,
        })
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::digest::sha256_hex;
    use crate::vcs::{VerdantReleaseCapsule, VerdantVcs};
    use std::collections::BTreeMap;

    #[test]
    fn sovereign_quorum_signing_requests_departure_revocation_and_quarantine() {
        // 1. Genesis owner anchors epoch 1 detached from GitHub:
        assert!(SovereignAuthorityLedger::init_genesis("verdant", "github:alice", "sec").is_err());
        let mut ledger =
            SovereignAuthorityLedger::init_genesis("verdant-engine", "prism:owner-alice", "sec-a")
                .expect("genesis");
        assert_eq!(ledger.current_epoch, 1);
        assert_eq!(ledger.quorum_threshold_m, 1);

        // Anti-lockout refuses last-admin departure:
        assert!(ledger
            .execute(AuthorityCommand::DepartOrRevokeAdmin {
                target_admin_prism_id: "prism:owner-alice".to_string(),
                reason: DepartureReason::VoluntaryResignation,
                compromise_floor_epoch: None,
                approving_admins: vec![("prism:owner-alice".to_string(), "sec-a".to_string())],
                next_quorum_threshold_m: 1,
            })
            .is_err());

        // 2. Admit Bob (2-of-2) and Carol (2-of-3); single-admin threshold M=1 refused once N>=2:
        assert!(ledger
            .execute(AuthorityCommand::AddAdmin {
                candidate_prism_id: "prism:admin-bob".to_string(),
                candidate_secret: "sec-b".to_string(),
                approving_admins: vec![("prism:owner-alice".to_string(), "sec-a".to_string())],
                next_quorum_threshold_m: 1,
            })
            .is_err());
        ledger
            .execute(AuthorityCommand::AddAdmin {
                candidate_prism_id: "prism:admin-bob".to_string(),
                candidate_secret: "sec-b".to_string(),
                approving_admins: vec![("prism:owner-alice".to_string(), "sec-a".to_string())],
                next_quorum_threshold_m: 2,
            })
            .expect("add bob 2-of-2");
        ledger
            .execute(AuthorityCommand::AddAdmin {
                candidate_prism_id: "prism:admin-carol".to_string(),
                candidate_secret: "sec-c".to_string(),
                approving_admins: vec![
                    ("prism:owner-alice".to_string(), "sec-a".to_string()),
                    ("prism:admin-bob".to_string(), "sec-b".to_string()),
                ],
                next_quorum_threshold_m: 2,
            })
            .expect("add carol 2-of-3");
        assert_eq!(ledger.current_epoch, 3);

        // 3. Contributor submits signing request; separation of duties + 2-of-3 review:
        let sub = ledger
            .execute(AuthorityCommand::SubmitSigningRequest {
                requester_prism_id: "prism:dev-ezra".to_string(),
                requester_secret: "sec-e".to_string(),
                action: SigningActionKind::MergeCommit,
                target_ref: "refs/heads/main".to_string(),
                target_kappa: "abc123commit".to_string(),
                expiry_epoch: 6,
            })
            .expect("submit");
        let req_k = sub.subject_kappa.clone();

        // Bob approves (1 of 2 — cannot finalize yet):
        ledger
            .execute(AuthorityCommand::ReviewSigningRequest {
                request_kappa: req_k.clone(),
                admin_prism_id: "prism:admin-bob".to_string(),
                admin_secret: "sec-b".to_string(),
                verdict: ReviewVerdictKind::Approve,
                rationale: "LGTM".to_string(),
            })
            .expect("bob approves");
        assert!(ledger
            .execute(AuthorityCommand::FinalizeSigningRequest {
                request_kappa: req_k.clone(),
            })
            .is_err());

        // Carol also approves -> QuorumApproved (still unfinalized!):
        ledger
            .execute(AuthorityCommand::ReviewSigningRequest {
                request_kappa: req_k.clone(),
                admin_prism_id: "prism:admin-carol".to_string(),
                admin_secret: "sec-c".to_string(),
                verdict: ReviewVerdictKind::Approve,
                rationale: "Verified ratchet".to_string(),
            })
            .expect("carol approves");
        assert_eq!(
            ledger.requests[&req_k].state,
            SigningRequestState::QuorumApproved
        );

        // Bob issues a sole-attested standing grant before leaving:
        let grant_out = ledger
            .execute(AuthorityCommand::GrantStandingAccess {
                grantee_prism_id: "prism:dev-ezra".to_string(),
                scope: "triage:issues".to_string(),
                attesting_admins: vec![("prism:admin-bob".to_string(), "sec-b".to_string())],
            })
            .expect("grant");

        // 4. Bob LEAVES Verdant -> pending approval invalidated, standing grant revoked, boundary rotated!
        let dep = ledger
            .execute(AuthorityCommand::DepartOrRevokeAdmin {
                target_admin_prism_id: "prism:admin-bob".to_string(),
                reason: DepartureReason::VoluntaryResignation,
                compromise_floor_epoch: None,
                approving_admins: vec![("prism:admin-bob".to_string(), "sec-b".to_string())],
                next_quorum_threshold_m: 2,
            })
            .expect("bob resigns cleanly");
        assert_eq!(dep.invalidated_requests, 1);
        assert_eq!(dep.revoked_grants, 1);
        assert_eq!(ledger.boundary_key_epoch, 2);
        assert_eq!(
            ledger.requests[&req_k].state,
            SigningRequestState::RevocationInvalidated
        );
        assert!(!ledger.standing_grants[&grant_out.subject_kappa].active);

        // Finalizing now MUST fail because Bob's approval was revoked!
        assert!(ledger
            .execute(AuthorityCommand::FinalizeSigningRequest {
                request_kappa: req_k.clone(),
            })
            .is_err());
        // Departed Bob trying to sign again MUST fail!
        assert!(ledger
            .execute(AuthorityCommand::ReviewSigningRequest {
                request_kappa: req_k.clone(),
                admin_prism_id: "prism:admin-bob".to_string(),
                admin_secret: "sec-b".to_string(),
                verdict: ReviewVerdictKind::Approve,
                rationale: "Replay".to_string(),
            })
            .is_err());

        // Active Admin Alice steps in and approves -> now Alice + Carol = 2-of-2 active quorum -> Finalizes!
        ledger
            .execute(AuthorityCommand::ReviewSigningRequest {
                request_kappa: req_k.clone(),
                admin_prism_id: "prism:owner-alice".to_string(),
                admin_secret: "sec-a".to_string(),
                verdict: ReviewVerdictKind::Approve,
                rationale: "Re-reviewed after Bob departed".to_string(),
            })
            .expect("alice approves");
        ledger
            .execute(AuthorityCommand::FinalizeSigningRequest {
                request_kappa: req_k.clone(),
            })
            .expect("finalizes with active quorum");
        assert_eq!(
            ledger.requests[&req_k].state,
            SigningRequestState::Finalized
        );

        // 5. Historical commit sealed at epoch 3 by Alice+Bob remains valid after Bob's clean resignation at epoch 4:
        ledger
            .execute(AuthorityCommand::VerifyHistoricalCommit {
                finalized_epoch: 3,
                signer_prism_ids: vec![
                    "prism:owner-alice".to_string(),
                    "prism:admin-bob".to_string(),
                ],
            })
            .expect("clean historical commit valid");

        // 6. GitHub-free group project coordination work item + chain verification:
        ledger
            .execute(AuthorityCommand::CoordinateWorkItem {
                author_prism_id: "prism:dev-ezra".to_string(),
                author_secret: "sec-e".to_string(),
                kind: "rfc".to_string(),
                title: "Sovereign Stitch Governance RFC".to_string(),
                target_path: "crates/verdant-core/src/authority.rs".to_string(),
                required_review_group: "mgr-security-trust".to_string(),
                milestone: "M18".to_string(),
                stitch_shard_count: 4,
            })
            .expect("work item");
        ledger
            .execute(AuthorityCommand::VerifyChain)
            .expect("monotonic chain valid");
    }

    #[test]
    fn approval_certificate_binds_vcs_commits_and_release_capsules() {
        let mut ledger = SovereignAuthorityLedger::init_genesis(
            "verdant-engine",
            "prism:owner-alice",
            "sec-alice",
        )
        .expect("genesis");
        ledger
            .execute(AuthorityCommand::AddAdmin {
                candidate_prism_id: "prism:admin-bob".to_string(),
                candidate_secret: "sec-bob".to_string(),
                approving_admins: vec![("prism:owner-alice".to_string(), "sec-alice".to_string())],
                next_quorum_threshold_m: 2,
            })
            .expect("add bob");

        let mut files = BTreeMap::new();
        files.insert("index.html".to_string(), b"<!doctype html>ok".to_vec());
        let mut entries = BTreeMap::new();
        entries.insert("index.html".to_string(), sha256_hex(b"<!doctype html>ok"));
        let tree_root = VerdantVcs::compute_tree_root(&entries);

        let sub = ledger
            .execute(AuthorityCommand::SubmitSigningRequest {
                requester_prism_id: "prism:dev-ezra".to_string(),
                requester_secret: "sec-ezra".to_string(),
                action: SigningActionKind::MergeCommit,
                target_ref: "refs/verdant/heads/main".to_string(),
                target_kappa: tree_root.clone(),
                expiry_epoch: 8,
            })
            .expect("submit commit req");
        let req_k = sub.subject_kappa;
        for (aid, sec) in [
            ("prism:owner-alice", "sec-alice"),
            ("prism:admin-bob", "sec-bob"),
        ] {
            ledger
                .execute(AuthorityCommand::ReviewSigningRequest {
                    request_kappa: req_k.clone(),
                    admin_prism_id: aid.to_string(),
                    admin_secret: sec.to_string(),
                    verdict: ReviewVerdictKind::Approve,
                    rationale: "approved".to_string(),
                })
                .expect("approve commit");
        }
        ledger
            .execute(AuthorityCommand::FinalizeSigningRequest {
                request_kappa: req_k.clone(),
            })
            .expect("finalize commit");
        let commit_cert = ledger
            .execute(AuthorityCommand::IssueApprovalCertificate {
                request_kappa: req_k,
            })
            .expect("issue commit cert")
            .certificate
            .expect("cert present");

        let mut vcs = VerdantVcs::new();
        let commit = vcs
            .commit_with_authority(
                "refs/verdant/heads/main",
                "prism:dev-ezra",
                b"sec-ezra",
                "mgr-change-release",
                "AUTH-STITCH-SELF-HOSTING",
                "governed commit",
                &files,
                &mut ledger,
                commit_cert.clone(),
            )
            .expect("commit with authority");
        assert_eq!(commit.approval_certificate, Some(commit_cert.clone()));

        // Negative: tampered target kappa MUST fail closed!
        let mut bad_cert = commit_cert;
        bad_cert.target_kappa = "0000tampered".to_string();
        assert!(vcs
            .commit_with_authority(
                "refs/verdant/heads/main",
                "prism:dev-ezra",
                b"sec-ezra",
                "mgr-change-release",
                "AUTH-STITCH-SELF-HOSTING",
                "bad commit",
                &files,
                &mut ledger,
                bad_cert,
            )
            .is_err());

        // Release capsule bound to CutRelease certificate:
        let rel_sub = ledger
            .execute(AuthorityCommand::SubmitSigningRequest {
                requester_prism_id: "prism:dev-ezra".to_string(),
                requester_secret: "sec-ezra".to_string(),
                action: SigningActionKind::CutRelease,
                target_ref: "refs/verdant/releases/v2.12.0".to_string(),
                target_kappa: commit.commit_kappa.clone(),
                expiry_epoch: 8,
            })
            .expect("submit release req");
        let rel_k = rel_sub.subject_kappa;
        for (aid, sec) in [
            ("prism:owner-alice", "sec-alice"),
            ("prism:admin-bob", "sec-bob"),
        ] {
            ledger
                .execute(AuthorityCommand::ReviewSigningRequest {
                    request_kappa: rel_k.clone(),
                    admin_prism_id: aid.to_string(),
                    admin_secret: sec.to_string(),
                    verdict: ReviewVerdictKind::Approve,
                    rationale: "release approved".to_string(),
                })
                .expect("approve release");
        }
        ledger
            .execute(AuthorityCommand::FinalizeSigningRequest {
                request_kappa: rel_k.clone(),
            })
            .expect("finalize release");
        let rel_cert = ledger
            .execute(AuthorityCommand::IssueApprovalCertificate {
                request_kappa: rel_k,
            })
            .expect("issue release cert")
            .certificate
            .expect("rel cert");

        let capsule = VerdantReleaseCapsule::pack_from_files_with_authority(
            "2.12.0",
            16,
            &commit.commit_kappa,
            "seal-sha",
            "index.html",
            &files,
            &mut ledger,
            rel_cert.clone(),
        )
        .expect("pack governed capsule");
        let unpacked =
            VerdantReleaseCapsule::verify_and_unpack(&capsule.holo_bytes).expect("unpack");
        assert_eq!(unpacked.manifest.approval_certificate, Some(rel_cert));

        // Unified JSON bridge works identically:
        let bridge_raw = sovereign_authority_exec_json(
            &serde_json::json!({
                "init": {
                    "repoName": "verdant-studio",
                    "ownerPrismId": "prism:owner-studio",
                    "ownerSecret": "sec-studio"
                },
                "command": { "cmd": "verifyChain" }
            })
            .to_string(),
        );
        let parsed: serde_json::Value = serde_json::from_str(&bridge_raw).expect("bridge json");
        assert_eq!(parsed["ok"], serde_json::Value::Bool(true));
    }
}
