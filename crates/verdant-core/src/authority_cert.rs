//! Unified Wasm / CLI / MCP JSON kernel bridge and [`ApprovalCertificate`] for
//! `SovereignAuthorityLedger` (`VE-72`) so Browser Studio, CLI, and MCP execute the exact same
//! Rust governance kernel.

use serde::{Deserialize, Serialize};

use super::SovereignAuthorityLedger;

/// Sovereign governance role modeled in `Governance::GovernanceRole`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GovernanceRole {
    /// Initial repository owner anchoring genesis epoch `1`.
    GenesisOwner,
    /// Active member of the M-of-N Sovereign Admin quorum.
    SovereignAdmin,
    /// Developer submitting signed requests and group coordination envelopes.
    ProjectContributor,
    /// Read-only cryptographic verifier of epochs, approvals, and revocations.
    ObserverAuditor,
}

/// Privileged or coordination action bound by a signing request (`Governance::SigningActionKind`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SigningActionKind {
    /// Merge a verified `VerdantVcs` commit onto a protected ref.
    MergeCommit,
    /// Cut and publish a monotonic `.holo` / Stitch release pointer.
    CutRelease,
    /// Grant scoped contributor or maintainer capability.
    GrantRole,
    /// Admit a new Sovereign Admin into the authority quorum.
    AddAdmin,
    /// Revoke or process departure of an active Sovereign Admin.
    RevokeAdmin,
    /// Rotate quorum threshold or path-ownership policy.
    RotatePolicy,
    /// Coordinate an issue, RFC proposal, milestone, or review thread.
    CoordinateWorkItem,
}

/// Lifecycle state of a content-addressed signing request (`Governance::SigningRequestState`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SigningRequestState {
    /// Local unsigned draft.
    Draft,
    /// Submitted with valid requester Prism ID signature.
    Submitted,
    /// Active Admin review in progress.
    UnderReview,
    /// Active independent Admin approvals satisfy M-of-N quorum.
    QuorumApproved,
    /// Finalized and committed to the immutable ledger.
    Finalized,
    /// Explicitly rejected by an active Sovereign Admin.
    Rejected,
    /// Invalidated because an approving Admin departed before finalization.
    RevocationInvalidated,
    /// Expired past `expiry_epoch`.
    Expired,
}

/// Reason an Admin leaves the active quorum (`Governance::DepartureReason`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DepartureReason {
    /// Clean self-signed or mutually agreed resignation.
    VoluntaryResignation,
    /// Removal ratified by M-of-N remaining active Admins.
    QuorumRemoval,
    /// Revocation triggered by suspected key compromise (activates compromise-floor quarantine).
    KeyCompromise,
}

/// Admin review verdict on a submitted signing request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReviewVerdictKind {
    /// Signed cryptographic approval bound to the active authority epoch.
    Approve,
    /// Signed cryptographic rejection terminating the request.
    Reject,
}

/// Ledger record of a Sovereign Admin (active or departed/revoked).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminMemberRecord {
    /// Sovereign `Prism ID` (`prism:...`).
    pub prism_id: String,
    /// Public key fingerprint (`sha256(prism_id || secret)` hex).
    pub key_fingerprint: String,
    /// Assigned governance role.
    pub role: GovernanceRole,
    /// Authority epoch at which this Admin was admitted.
    pub admitted_epoch: u64,
    /// Authority epoch at which this Admin departed or was revoked, if no longer active.
    pub revoked_at_epoch: Option<u64>,
    /// Departure classification when revoked.
    pub departure_reason: Option<DepartureReason>,
    /// Optional compromise floor epoch for `KeyCompromise` revocations.
    pub compromise_floor_epoch: Option<u64>,
}

/// Immutable chained `AuthorityEpoch` snapshot (`prevEpochKappa -> epochKappa`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorityEpochRecord {
    /// Strictly monotonic authority epoch (`>= 1`).
    pub epoch: u64,
    /// Content-addressed commitment of the immediately preceding epoch (or genesis anchor).
    pub prev_epoch_kappa: String,
    /// Content-addressed SHA-256 commitment of this epoch record.
    pub epoch_kappa: String,
    /// Sorted list of active Sovereign Admin `Prism ID`s at this epoch.
    pub active_admins: Vec<String>,
    /// Active M-of-N quorum threshold (`1 <= M <= N`; `M >= 2` whenever `N >= 2`).
    pub quorum_threshold_m: u64,
    /// Forward-secret private workspace AEAD boundary key epoch.
    pub boundary_key_epoch: u64,
    /// W3C PROV-O human-readable transition summary.
    pub provo_summary: String,
    /// Active Admin `Prism ID`s that co-signed this epoch transition.
    pub co_signers: Vec<String>,
}

/// Signed PROV-O Admin review attestation on a [`SigningRequestRecord`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewAttestation {
    /// Reviewing Admin's `Prism ID`.
    pub admin_prism_id: String,
    /// Signed approve or reject verdict.
    pub verdict: ReviewVerdictKind,
    /// Authority epoch under which this signature was produced.
    pub signed_epoch: u64,
    /// Audit rationale.
    pub rationale: String,
    /// Domain-separated cryptographic signature hex.
    pub signature_hex: String,
}

/// Content-addressed contributor signing request governed by [`SovereignAuthorityLedger`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigningRequestRecord {
    /// Content-addressed SHA-256 request `kappa`.
    pub request_kappa: String,
    /// Submitting contributor or Admin `Prism ID`.
    pub requester_prism_id: String,
    /// Requester's cryptographic signature over the request preimage.
    pub requester_signature_hex: String,
    /// Action scope governed by this request.
    pub action: SigningActionKind,
    /// Target branch ref, release tag, or capability scope.
    pub target_ref: String,
    /// Content-addressed target commit or artifact `kappa`.
    pub target_kappa: String,
    /// Authority epoch active when submitted.
    pub submitted_epoch: u64,
    /// Final authority epoch at which this request remains admissible.
    pub expiry_epoch: u64,
    /// Current deterministic state machine status.
    pub state: SigningRequestState,
    /// Immutable chronological review attestations.
    pub reviews: Vec<ReviewAttestation>,
    /// Authority epoch at which this request finalized, if finalized.
    pub finalized_epoch: Option<u64>,
}

/// Standing delegated capability grant that auto-revokes when its sole attesting Admin departs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandingGrantRecord {
    /// Content-addressed grant `kappa`.
    pub grant_kappa: String,
    /// Grantee's `Prism ID`.
    pub grantee_prism_id: String,
    /// Scoped capability identifier (e.g. `branch:release`, `triage:issues`).
    pub scope: String,
    /// Authority epoch when granted.
    pub granted_epoch: u64,
    /// Active Admins who attested this grant.
    pub attested_by: Vec<String>,
    /// True while currently active and not invalidated by Admin departure.
    pub active: bool,
    /// Diagnostic reason if automatically revoked on Admin departure.
    pub revoked_reason: Option<String>,
}

/// GitHub-free W3C PROV-O group project coordination record (issue, RFC, milestone, review thread).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkItemRecord {
    /// Content-addressed work-item `kappa`.
    pub item_kappa: String,
    /// Author's `Prism ID`.
    pub author_prism_id: String,
    /// Work-item category (`issue`, `rfc`, `milestone`, `review-thread`).
    pub kind: String,
    /// Title or summary.
    pub title: String,
    /// Path-scoped ownership target (`CODEOWNERS` replacement).
    pub target_path: String,
    /// Required review group identifier.
    pub required_review_group: String,
    /// Associated milestone identifier.
    pub milestone: String,
    /// Verified Stitch erasure shard count (`>= 4` for 4-of-8 custody).
    pub stitch_shard_count: u64,
    /// Author's cryptographic signature hex.
    pub signature_hex: String,
}

/// Deterministic command dispatched to [`SovereignAuthorityLedger::execute`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum AuthorityCommand {
    /// Developer submits a signed content-addressed signing request.
    SubmitSigningRequest {
        /// Requester's `Prism ID` (`prism:...`).
        requester_prism_id: String,
        /// Requester's key material (for deterministic native/CLI signing).
        requester_secret: String,
        /// Requested action kind.
        action: SigningActionKind,
        /// Target ref or scope.
        target_ref: String,
        /// Target commit or artifact `kappa`.
        target_kappa: String,
        /// Expiry authority epoch (`>= current_epoch`).
        expiry_epoch: u64,
    },
    /// Active Verdant Sovereign Admin reviews and signs `Approve` or `Reject`.
    ReviewSigningRequest {
        /// Target signing request `kappa`.
        request_kappa: String,
        /// Reviewing Admin `Prism ID`.
        admin_prism_id: String,
        /// Reviewing Admin key material.
        admin_secret: String,
        /// Approve or Reject verdict.
        verdict: ReviewVerdictKind,
        /// Immutable PROV-O audit rationale.
        rationale: String,
    },
    /// Finalize a quorum-approved signing request if active approvals still satisfy M-of-N.
    FinalizeSigningRequest {
        /// Target signing request `kappa`.
        request_kappa: String,
    },
    /// Admit a new Sovereign Admin and advance the monotonic authority epoch.
    AddAdmin {
        /// New Admin's `Prism ID`.
        candidate_prism_id: String,
        /// New Admin's key material (for key-fingerprint registration).
        candidate_secret: String,
        /// Active Admins approving the addition (`(prism_id, secret)` pairs).
        approving_admins: Vec<(String, String)>,
        /// Next M-of-N quorum threshold (`>= 2` once `N >= 2`).
        next_quorum_threshold_m: u64,
    },
    /// Process an Admin's departure or revocation, advance epoch, rotate boundary key, and
    /// immediately invalidate their pending approvals and sole-attested standing grants.
    DepartOrRevokeAdmin {
        /// Departing or revoked Admin `Prism ID`.
        target_admin_prism_id: String,
        /// Departure classification (`VoluntaryResignation`, `QuorumRemoval`, `KeyCompromise`).
        reason: DepartureReason,
        /// Required for `KeyCompromise`: earliest suspect epoch to quarantine.
        compromise_floor_epoch: Option<u64>,
        /// Co-signing active Admins (`(prism_id, secret)` pairs).
        approving_admins: Vec<(String, String)>,
        /// Next valid M-of-N threshold for the remaining active Admin set.
        next_quorum_threshold_m: u64,
    },
    /// Issue a standing delegated access grant attested by one or more active Admins.
    GrantStandingAccess {
        /// Grantee's `Prism ID`.
        grantee_prism_id: String,
        /// Capability scope string.
        scope: String,
        /// Attesting active Admins (`(prism_id, secret)` pairs).
        attesting_admins: Vec<(String, String)>,
    },
    /// Record a GitHub-free W3C PROV-O group coordination item (issue, RFC, milestone).
    CoordinateWorkItem {
        /// Author's `Prism ID`.
        author_prism_id: String,
        /// Author's key material.
        author_secret: String,
        /// Work-item category (`issue`, `rfc`, `milestone`, `review-thread`).
        kind: String,
        /// Work-item title.
        title: String,
        /// Path-scoped ownership target.
        target_path: String,
        /// Required review group name.
        required_review_group: String,
        /// Target milestone.
        milestone: String,
        /// Stitch erasure shard count (`>= 4`).
        stitch_shard_count: u64,
    },
    /// Verify whether a historical finalized commit remains valid or is quarantined by compromise.
    VerifyHistoricalCommit {
        /// Epoch at which the commit was finalized.
        finalized_epoch: u64,
        /// Admin `Prism ID`s that signed the commit at `finalized_epoch`.
        signer_prism_ids: Vec<String>,
    },
    /// Verify the full monotonic epoch chain, `prev_epoch_kappa` links, and quorum invariants.
    VerifyChain,
    /// Issue a self-verifying [`ApprovalCertificate`] for a finalized signing request (`VE-72`).
    IssueApprovalCertificate {
        /// Finalized signing request `kappa`.
        request_kappa: String,
    },
    /// Cryptographically verify an [`ApprovalCertificate`] against target `kappa`, action kind,
    /// monotonic authority chain, active quorum threshold, and compromise quarantine floor (`VE-72`).
    VerifyApprovalCertificate {
        /// Candidate approval certificate to verify.
        certificate: ApprovalCertificate,
        /// Expected `VerdantVcs` commit or `.holo` capsule target `kappa`.
        expected_target_kappa: String,
        /// Expected privileged signing action kind.
        expected_action: SigningActionKind,
    },
}

/// Structured outcome returned by [`SovereignAuthorityLedger::execute`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorityOutcome {
    /// Fixed schema identifier (`verdant/sovereign-authority-outcome/1`).
    pub spec: String,
    /// Current authority epoch after execution.
    pub current_epoch: u64,
    /// Current forward-secret private workspace boundary key epoch.
    pub boundary_key_epoch: u64,
    /// Active Sovereign Admin count.
    pub active_admin_count: u64,
    /// Active M-of-N quorum threshold.
    pub quorum_threshold_m: u64,
    /// Primary content-addressed `kappa` created or verified by this command.
    pub subject_kappa: String,
    /// Count of pending signing requests invalidated by an Admin departure (if applicable).
    pub invalidated_requests: u64,
    /// Count of standing grants automatically revoked by an Admin departure (if applicable).
    pub revoked_grants: u64,
    /// Concise audit summary.
    pub summary: String,
    /// Optional issued or verified [`ApprovalCertificate`] when executing certificate commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<ApprovalCertificate>,
}

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
