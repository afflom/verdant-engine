//! **GitHub-Free Sovereign Authority Ledger, Signing Request Gate & Revocation Runtime (`authority.rs`)**
//!
//! Realizes the formally proved Prism governance model (`app/src/Governance.lex.tex`,
//! `Architecture::componentSovereignAuthorityLedger`, `Architecture::componentSigningRequestGate`,
//! and `Security::sovereignGovernanceThreatModelEvidenced` — `VE-70`):
//! - **Genesis Owner Authority Root**: Initializes `authorityEpoch = 1` anchored to the repository
//!   owner's non-extractable `Prism ID` (`prism:...`), zero GitHub dependency.
//! - **Content-Addressed Signing Requests**: Developers submit signed requests binding commit
//!   `kappa`, action scope, `submitted_epoch`, and `expiry_epoch`; active Sovereign Admins inspect
//!   and emit signed PROV-O `Approve` or `Reject` attestations.
//! - **M-of-N Admin Quorum & Separation of Duties**: Once `N >= 2` active Admins hold custody,
//!   privileged mutations enforce `2 <= M <= N` independent active Admin approvals excluding author
//!   self-approval.
//! - **Immediate Departed-Admin Approval Revocation**: When an Admin resigns or is revoked,
//!   `current_epoch` increments monotonically chained via `prev_epoch_kappa`, `boundary_key_epoch`
//!   rotates forward-secretly, future signatures from that Admin are refused, unfinalized pending
//!   requests drop that Admin's approvals (transitioning to `RevocationInvalidated` if active
//!   approvals fall below `M`), sole-attested standing grants revoke automatically, and anti-lockout
//!   refuses any departure that would leave `0` or `< M` active Admins.
//! - **Historical Lineage vs Compromise Quarantine**: Commits finalized prior to clean Admin
//!   departure stay verifiable at their historical epoch, while `KeyCompromise` departures
//!   quarantine commits at or after `compromise_floor_epoch`.
//! - **Decentralized Group Coordination**: Issues, RFC proposals, milestones, and path-scoped
//!   review obligations persist as signed W3C PROV-O envelopes backed by Stitch 4-of-8 custody.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use verdant_tensor::scalars::{
    admin_signature_valid_at_epoch, authority_epoch_advance_valid,
    departure_requires_compromise_quarantine, four_u64, genesis_bootstrap_valid,
    group_coordination_envelope_valid, historical_commit_valid,
    pending_approval_survives_departure, quorum_threshold_valid, signing_request_admissible,
    standing_grant_valid_after_departure, two_u64,
};

use crate::digest::sha256_hex_tagged;
use crate::error::StitchCustodyError;

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

/// Append-only sovereign post-GitHub authority ledger and signing-request gate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SovereignAuthorityLedger {
    /// Fixed schema identifier (`verdant/sovereign-authority-ledger/1`).
    pub spec: String,
    /// Content-addressed genesis authority root `kappa`.
    pub genesis_kappa: String,
    /// Repository owner's `Prism ID` anchoring genesis epoch `1`.
    pub genesis_owner_prism_id: String,
    /// Current monotonic authority epoch (`>= 1`).
    pub current_epoch: u64,
    /// Current forward-secret private boundary encryption key epoch (`>= 1`).
    pub boundary_key_epoch: u64,
    /// Current M-of-N quorum threshold (`1 <= M <= N`; `M >= 2` when `N >= 2`).
    pub quorum_threshold_m: u64,
    /// All known Sovereign Admins keyed by `Prism ID` (active and departed/revoked).
    pub admins: BTreeMap<String, AdminMemberRecord>,
    /// Monotonic `prev_epoch_kappa`-chained authority epoch history.
    pub epochs: Vec<AuthorityEpochRecord>,
    /// Content-addressed signing requests keyed by `request_kappa`.
    pub requests: BTreeMap<String, SigningRequestRecord>,
    /// Standing delegated access grants keyed by `grant_kappa`.
    pub standing_grants: BTreeMap<String, StandingGrantRecord>,
    /// GitHub-free W3C PROV-O work items keyed by `item_kappa`.
    pub work_items: BTreeMap<String, WorkItemRecord>,
}

impl SovereignAuthorityLedger {
    /// Initializes a GitHub-free sovereign governance ledger at epoch `1` with the repository
    /// owner as the first Sovereign Admin (`GenesisOwner`, 1-of-1 bootstrap quorum).
    pub fn init_genesis(
        repo_name: &str,
        owner_prism_id: &str,
        owner_secret: &str,
    ) -> Result<Self, StitchCustodyError> {
        let valid_prism = owner_prism_id.starts_with("prism:") && !owner_secret.trim().is_empty();
        if repo_name.trim().is_empty()
            || !genesis_bootstrap_valid(1, valid_prism, valid_prism, false)
        {
            return Err(StitchCustodyError::InvalidJournalSignature {
                detail: "genesis bootstrap requires non-empty repo name, prism: owner ID, and non-extractable key secret"
                    .to_string(),
            });
        }
        let owner_fp = sha256_hex_tagged(
            "verdant/authority/keyfp/1:",
            owner_prism_id,
            owner_secret.as_bytes(),
        );
        let genesis_preimage = format!("{repo_name}|{owner_prism_id}|{owner_fp}|epoch=1");
        let genesis_kappa = sha256_hex_tagged(
            "verdant/authority/genesis/1:",
            repo_name,
            genesis_preimage.as_bytes(),
        );
        let epoch_kappa = sha256_hex_tagged(
            "verdant/authority/epoch/1:",
            &genesis_kappa,
            format!("epoch=1|m=1|boundary=1|admins={owner_prism_id}").as_bytes(),
        );
        let mut admins = BTreeMap::new();
        admins.insert(
            owner_prism_id.to_string(),
            AdminMemberRecord {
                prism_id: owner_prism_id.to_string(),
                key_fingerprint: owner_fp,
                role: GovernanceRole::GenesisOwner,
                admitted_epoch: 1,
                revoked_at_epoch: None,
                departure_reason: None,
                compromise_floor_epoch: None,
            },
        );
        let epoch_one = AuthorityEpochRecord {
            epoch: 1,
            prev_epoch_kappa: genesis_kappa.clone(),
            epoch_kappa,
            active_admins: vec![owner_prism_id.to_string()],
            quorum_threshold_m: 1,
            boundary_key_epoch: 1,
            provo_summary: format!("Genesis repo owner {owner_prism_id} anchored epoch 1"),
            co_signers: vec![owner_prism_id.to_string()],
        };
        Ok(Self {
            spec: "verdant/sovereign-authority-ledger/1".to_string(),
            genesis_kappa,
            genesis_owner_prism_id: owner_prism_id.to_string(),
            current_epoch: 1,
            boundary_key_epoch: 1,
            quorum_threshold_m: 1,
            admins,
            epochs: vec![epoch_one],
            requests: BTreeMap::new(),
            standing_grants: BTreeMap::new(),
            work_items: BTreeMap::new(),
        })
    }

    /// Executes a deterministic [`AuthorityCommand`] against this ledger, enforcing every formal
    /// `Governance.lex.tex` invariant (`quorumThresholdValid`, `authorityEpochAdvanceValid`,
    /// `adminSignatureValidAtEpoch`, `pendingApprovalSurvivesDeparture`,
    /// `standingGrantValidAfterDeparture`, `historicalCommitValid`, `signingRequestAdmissible`,
    /// and `groupCoordinationEnvelopeValid`).
    pub fn execute(
        &mut self,
        cmd: AuthorityCommand,
    ) -> Result<AuthorityOutcome, StitchCustodyError> {
        let active_set: BTreeSet<String> = self
            .admins
            .values()
            .filter(|a| a.revoked_at_epoch.is_none())
            .map(|a| a.prism_id.clone())
            .collect();
        let active_n = active_set.len() as u64;

        match cmd {
            AuthorityCommand::SubmitSigningRequest {
                requester_prism_id,
                requester_secret,
                action,
                target_ref,
                target_kappa,
                expiry_epoch,
            } => {
                if !requester_prism_id.starts_with("prism:")
                    || requester_secret.trim().is_empty()
                    || target_ref.trim().is_empty()
                    || target_kappa.trim().is_empty()
                    || expiry_epoch < self.current_epoch
                {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "signing request requires valid prism: requester, non-empty target/kappa, and expiry_epoch >= current_epoch"
                            .to_string(),
                    });
                }
                let preimage = format!(
                    "{}|{:?}|{}|{}|{}|{}",
                    requester_prism_id,
                    action,
                    target_ref,
                    target_kappa,
                    self.current_epoch,
                    expiry_epoch
                );
                let request_kappa = sha256_hex_tagged(
                    "verdant/authority/request/1:",
                    &requester_prism_id,
                    preimage.as_bytes(),
                );
                let sig = sha256_hex_tagged(
                    "verdant/authority/sig/1:",
                    &request_kappa,
                    format!("{requester_prism_id}|{requester_secret}").as_bytes(),
                );
                self.requests.insert(
                    request_kappa.clone(),
                    SigningRequestRecord {
                        request_kappa: request_kappa.clone(),
                        requester_prism_id: requester_prism_id.clone(),
                        requester_signature_hex: sig,
                        action,
                        target_ref: target_ref.clone(),
                        target_kappa,
                        submitted_epoch: self.current_epoch,
                        expiry_epoch,
                        state: SigningRequestState::Submitted,
                        reviews: Vec::new(),
                        finalized_epoch: None,
                    },
                );
                Ok(AuthorityOutcome {
                    spec: "verdant/sovereign-authority-outcome/1".to_string(),
                    current_epoch: self.current_epoch,
                    boundary_key_epoch: self.boundary_key_epoch,
                    active_admin_count: active_n,
                    quorum_threshold_m: self.quorum_threshold_m,
                    subject_kappa: request_kappa,
                    invalidated_requests: 0,
                    revoked_grants: 0,
                    summary: format!(
                        "Submitted signing request by {requester_prism_id} for {target_ref}"
                    ),
                    certificate: None,
                })
            }

            AuthorityCommand::ReviewSigningRequest {
                request_kappa,
                admin_prism_id,
                admin_secret,
                verdict,
                rationale,
            } => {
                let member = self.admins.get(&admin_prism_id).ok_or_else(|| {
                    StitchCustodyError::InvalidJournalSignature {
                        detail: format!("unknown admin `{admin_prism_id}`"),
                    }
                })?;
                let expected_fp = sha256_hex_tagged(
                    "verdant/authority/keyfp/1:",
                    &admin_prism_id,
                    admin_secret.as_bytes(),
                );
                let is_revoked = member.revoked_at_epoch.is_some();
                if expected_fp != member.key_fingerprint
                    || !admin_signature_valid_at_epoch(
                        active_set.contains(&admin_prism_id),
                        is_revoked,
                        self.current_epoch,
                        self.current_epoch,
                    )
                {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: format!(
                            "admin `{admin_prism_id}` is revoked, inactive, or failed Prism ID key authentication"
                        ),
                    });
                }
                let req = self.requests.get_mut(&request_kappa).ok_or_else(|| {
                    StitchCustodyError::InvalidJournalSignature {
                        detail: format!("unknown signing request `{request_kappa}`"),
                    }
                })?;
                if matches!(
                    req.state,
                    SigningRequestState::Finalized
                        | SigningRequestState::Rejected
                        | SigningRequestState::Expired
                ) {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: format!("signing request `{request_kappa}` is already terminal"),
                    });
                }
                if self.current_epoch > req.expiry_epoch {
                    req.state = SigningRequestState::Expired;
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: format!("signing request `{request_kappa}` expired"),
                    });
                }
                // Replace any prior review by this same Admin in an older epoch or prevent duplicate vote:
                req.reviews.retain(|r| r.admin_prism_id != admin_prism_id);
                let sig = sha256_hex_tagged(
                    "verdant/authority/review/1:",
                    &request_kappa,
                    format!(
                        "{admin_prism_id}|{:?}|{}|{rationale}|{admin_secret}",
                        verdict, self.current_epoch
                    )
                    .as_bytes(),
                );
                req.reviews.push(ReviewAttestation {
                    admin_prism_id: admin_prism_id.clone(),
                    verdict,
                    signed_epoch: self.current_epoch,
                    rationale,
                    signature_hex: sig,
                });

                let rejected = req.reviews.iter().any(|r| {
                    r.verdict == ReviewVerdictKind::Reject && active_set.contains(&r.admin_prism_id)
                });
                let mut total_approvals = 0_u64;
                let mut departed_approvals = 0_u64;
                let mut author_self = false;
                for r in &req.reviews {
                    if r.verdict == ReviewVerdictKind::Approve {
                        total_approvals = total_approvals.saturating_add(1);
                        if !active_set.contains(&r.admin_prism_id) {
                            departed_approvals = departed_approvals.saturating_add(1);
                        } else if r.admin_prism_id == req.requester_prism_id {
                            author_self = true;
                        }
                    }
                }
                if rejected {
                    req.state = SigningRequestState::Rejected;
                } else if signing_request_admissible(
                    !req.requester_signature_hex.is_empty(),
                    false,
                    self.current_epoch,
                    req.expiry_epoch,
                    total_approvals,
                    departed_approvals,
                    self.quorum_threshold_m,
                    author_self,
                    active_n,
                ) {
                    req.state = SigningRequestState::QuorumApproved;
                } else {
                    req.state = SigningRequestState::UnderReview;
                }
                let state = req.state;
                Ok(AuthorityOutcome {
                    spec: "verdant/sovereign-authority-outcome/1".to_string(),
                    current_epoch: self.current_epoch,
                    boundary_key_epoch: self.boundary_key_epoch,
                    active_admin_count: active_n,
                    quorum_threshold_m: self.quorum_threshold_m,
                    subject_kappa: request_kappa,
                    invalidated_requests: 0,
                    revoked_grants: 0,
                    summary: format!(
                        "Admin {admin_prism_id} recorded {verdict:?}; state={state:?}"
                    ),
                    certificate: None,
                })
            }

            AuthorityCommand::FinalizeSigningRequest { request_kappa } => {
                let req = self.requests.get_mut(&request_kappa).ok_or_else(|| {
                    StitchCustodyError::InvalidJournalSignature {
                        detail: format!("unknown signing request `{request_kappa}`"),
                    }
                })?;
                if req.state == SigningRequestState::Finalized {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "signing request already finalized".to_string(),
                    });
                }
                if self.current_epoch > req.expiry_epoch {
                    req.state = SigningRequestState::Expired;
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "signing request expired before finalization".to_string(),
                    });
                }
                let rejected = req.reviews.iter().any(|r| {
                    r.verdict == ReviewVerdictKind::Reject && active_set.contains(&r.admin_prism_id)
                });
                let mut total_approvals = 0_u64;
                let mut departed_approvals = 0_u64;
                let mut author_self = false;
                for r in &req.reviews {
                    if r.verdict == ReviewVerdictKind::Approve {
                        total_approvals = total_approvals.saturating_add(1);
                        if !active_set.contains(&r.admin_prism_id) {
                            departed_approvals = departed_approvals.saturating_add(1);
                        } else if r.admin_prism_id == req.requester_prism_id {
                            author_self = true;
                        }
                    }
                }
                if !signing_request_admissible(
                    !req.requester_signature_hex.is_empty(),
                    rejected,
                    self.current_epoch,
                    req.expiry_epoch,
                    total_approvals,
                    departed_approvals,
                    self.quorum_threshold_m,
                    author_self,
                    active_n,
                ) {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "signing request failed M-of-N active quorum, separation of duties, or revocation check"
                            .to_string(),
                    });
                }
                req.state = SigningRequestState::Finalized;
                req.finalized_epoch = Some(self.current_epoch);
                Ok(AuthorityOutcome {
                    spec: "verdant/sovereign-authority-outcome/1".to_string(),
                    current_epoch: self.current_epoch,
                    boundary_key_epoch: self.boundary_key_epoch,
                    active_admin_count: active_n,
                    quorum_threshold_m: self.quorum_threshold_m,
                    subject_kappa: request_kappa,
                    invalidated_requests: 0,
                    revoked_grants: 0,
                    summary: format!("Finalized signing request at epoch {}", self.current_epoch),
                    certificate: None,
                })
            }

            AuthorityCommand::AddAdmin {
                candidate_prism_id,
                candidate_secret,
                approving_admins,
                next_quorum_threshold_m,
            } => {
                if !candidate_prism_id.starts_with("prism:")
                    || candidate_secret.trim().is_empty()
                    || active_set.contains(&candidate_prism_id)
                {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "candidate admin must have a fresh prism: ID and secret"
                            .to_string(),
                    });
                }
                let mut valid_signers = BTreeSet::new();
                for (aid, sec) in &approving_admins {
                    if let Some(m) = self.admins.get(aid) {
                        let fp =
                            sha256_hex_tagged("verdant/authority/keyfp/1:", aid, sec.as_bytes());
                        if fp == m.key_fingerprint
                            && admin_signature_valid_at_epoch(
                                active_set.contains(aid),
                                m.revoked_at_epoch.is_some(),
                                self.current_epoch,
                                self.current_epoch,
                            )
                        {
                            valid_signers.insert(aid.clone());
                        }
                    }
                }
                let quorum_met = (valid_signers.len() as u64) >= self.quorum_threshold_m;
                let next_epoch = self.current_epoch.saturating_add(1);
                let next_n = active_n.saturating_add(1);
                let prev_kappa = self
                    .epochs
                    .last()
                    .map(|e| e.epoch_kappa.clone())
                    .unwrap_or_default();
                if !authority_epoch_advance_valid(
                    self.current_epoch,
                    next_epoch,
                    !prev_kappa.is_empty(),
                    quorum_met,
                    next_n,
                    next_quorum_threshold_m,
                ) {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "AddAdmin refused: quorum not met or invalid next M-of-N threshold"
                            .to_string(),
                    });
                }
                let cand_fp = sha256_hex_tagged(
                    "verdant/authority/keyfp/1:",
                    &candidate_prism_id,
                    candidate_secret.as_bytes(),
                );
                self.admins.insert(
                    candidate_prism_id.clone(),
                    AdminMemberRecord {
                        prism_id: candidate_prism_id.clone(),
                        key_fingerprint: cand_fp,
                        role: GovernanceRole::SovereignAdmin,
                        admitted_epoch: next_epoch,
                        revoked_at_epoch: None,
                        departure_reason: None,
                        compromise_floor_epoch: None,
                    },
                );
                self.current_epoch = next_epoch;
                self.quorum_threshold_m = next_quorum_threshold_m;
                let next_active: Vec<String> = self
                    .admins
                    .values()
                    .filter(|a| a.revoked_at_epoch.is_none())
                    .map(|a| a.prism_id.clone())
                    .collect();
                let co_signers: Vec<String> = valid_signers.into_iter().collect();
                let epoch_kappa = sha256_hex_tagged(
                    "verdant/authority/epoch/1:",
                    &prev_kappa,
                    format!(
                        "epoch={next_epoch}|m={next_quorum_threshold_m}|boundary={}|admins={}",
                        self.boundary_key_epoch,
                        next_active.join(",")
                    )
                    .as_bytes(),
                );
                self.epochs.push(AuthorityEpochRecord {
                    epoch: next_epoch,
                    prev_epoch_kappa: prev_kappa,
                    epoch_kappa: epoch_kappa.clone(),
                    active_admins: next_active,
                    quorum_threshold_m: next_quorum_threshold_m,
                    boundary_key_epoch: self.boundary_key_epoch,
                    provo_summary: format!(
                        "Admitted Sovereign Admin {candidate_prism_id} at epoch {next_epoch}"
                    ),
                    co_signers,
                });
                Ok(AuthorityOutcome {
                    spec: "verdant/sovereign-authority-outcome/1".to_string(),
                    current_epoch: self.current_epoch,
                    boundary_key_epoch: self.boundary_key_epoch,
                    active_admin_count: next_n,
                    quorum_threshold_m: self.quorum_threshold_m,
                    subject_kappa: epoch_kappa,
                    invalidated_requests: 0,
                    revoked_grants: 0,
                    summary: format!(
                        "Admitted admin {candidate_prism_id} at epoch {next_epoch} (M={next_quorum_threshold_m}/N={next_n})"
                    ),
                    certificate: None,
                })
            }

            AuthorityCommand::DepartOrRevokeAdmin {
                target_admin_prism_id,
                reason,
                compromise_floor_epoch,
                approving_admins,
                next_quorum_threshold_m,
            } => {
                if !active_set.contains(&target_admin_prism_id) {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: format!("target `{target_admin_prism_id}` is not an active admin"),
                    });
                }
                let mut valid_signers = BTreeSet::new();
                for (aid, sec) in &approving_admins {
                    if let Some(m) = self.admins.get(aid) {
                        let fp =
                            sha256_hex_tagged("verdant/authority/keyfp/1:", aid, sec.as_bytes());
                        if fp == m.key_fingerprint
                            && admin_signature_valid_at_epoch(
                                active_set.contains(aid),
                                m.revoked_at_epoch.is_some(),
                                self.current_epoch,
                                self.current_epoch,
                            )
                        {
                            valid_signers.insert(aid.clone());
                        }
                    }
                }
                let self_resign = reason == DepartureReason::VoluntaryResignation
                    && valid_signers.contains(&target_admin_prism_id);
                let non_target_signers = valid_signers
                    .iter()
                    .filter(|s| s.as_str() != target_admin_prism_id.as_str())
                    .count() as u64;
                let quorum_met = self_resign
                    || non_target_signers >= self.quorum_threshold_m
                    || (active_n == two_u64()
                        && non_target_signers >= 1
                        && reason != DepartureReason::VoluntaryResignation);
                let remaining_admins = active_n.saturating_sub(1);
                let next_epoch = self.current_epoch.saturating_add(1);
                let prev_kappa = self
                    .epochs
                    .last()
                    .map(|e| e.epoch_kappa.clone())
                    .unwrap_or_default();
                if !authority_epoch_advance_valid(
                    self.current_epoch,
                    next_epoch,
                    !prev_kappa.is_empty(),
                    quorum_met,
                    remaining_admins,
                    next_quorum_threshold_m,
                ) {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "Admin departure/revocation refused: anti-lockout guard or quorum failed"
                            .to_string(),
                    });
                }
                let reason_code = match reason {
                    DepartureReason::VoluntaryResignation => 0_u8,
                    DepartureReason::QuorumRemoval => 1_u8,
                    DepartureReason::KeyCompromise => two_u64() as u8,
                };
                let comp_floor = if departure_requires_compromise_quarantine(reason_code) {
                    Some(compromise_floor_epoch.unwrap_or(1))
                } else {
                    None
                };
                if let Some(m) = self.admins.get_mut(&target_admin_prism_id) {
                    m.revoked_at_epoch = Some(next_epoch);
                    m.departure_reason = Some(reason);
                    m.compromise_floor_epoch = comp_floor;
                }
                self.current_epoch = next_epoch;
                self.quorum_threshold_m = next_quorum_threshold_m;
                // Forward-secret private workspace boundary key rotation:
                self.boundary_key_epoch = self.boundary_key_epoch.saturating_add(1);

                let remaining_set: BTreeSet<String> = self
                    .admins
                    .values()
                    .filter(|a| a.revoked_at_epoch.is_none())
                    .map(|a| a.prism_id.clone())
                    .collect();

                // Immediately re-evaluate ALL pending/unfinalized signing requests and invalidate
                // any whose active approvals drop below M after subtracting the departed Admin!
                let mut invalidated_requests = 0_u64;
                for req in self.requests.values_mut() {
                    if matches!(
                        req.state,
                        SigningRequestState::Submitted
                            | SigningRequestState::UnderReview
                            | SigningRequestState::QuorumApproved
                    ) {
                        let had_departed = req.reviews.iter().any(|r| {
                            r.admin_prism_id == target_admin_prism_id
                                && r.verdict == ReviewVerdictKind::Approve
                        });
                        if had_departed {
                            let mut total_approvals = 0_u64;
                            let mut departed_approvals = 0_u64;
                            let mut author_self = false;
                            for r in &req.reviews {
                                if r.verdict == ReviewVerdictKind::Approve {
                                    total_approvals = total_approvals.saturating_add(1);
                                    if !remaining_set.contains(&r.admin_prism_id) {
                                        departed_approvals = departed_approvals.saturating_add(1);
                                    } else if r.admin_prism_id == req.requester_prism_id {
                                        author_self = true;
                                    }
                                }
                            }
                            if !pending_approval_survives_departure(
                                total_approvals,
                                departed_approvals,
                                self.quorum_threshold_m,
                                author_self,
                                remaining_admins,
                            ) {
                                req.state = SigningRequestState::RevocationInvalidated;
                                invalidated_requests = invalidated_requests.saturating_add(1);
                            }
                        }
                    }
                }

                // Immediately revoke any standing grant whose sole/insufficient attestors are no
                // longer backed by the active quorum!
                let mut revoked_grants = 0_u64;
                for grant in self.standing_grants.values_mut() {
                    if grant.active && grant.attested_by.contains(&target_admin_prism_id) {
                        let remaining_attestors = grant
                            .attested_by
                            .iter()
                            .filter(|a| remaining_set.contains(*a))
                            .count() as u64;
                        let co_signed = remaining_attestors >= self.quorum_threshold_m;
                        if !standing_grant_valid_after_departure(true, co_signed, true) {
                            grant.active = false;
                            grant.revoked_reason = Some(format!(
                                "attesting admin {target_admin_prism_id} departed at epoch {next_epoch}"
                            ));
                            revoked_grants = revoked_grants.saturating_add(1);
                        }
                    }
                }

                let next_active: Vec<String> = remaining_set.into_iter().collect();
                let co_signers: Vec<String> = valid_signers.into_iter().collect();
                let epoch_kappa = sha256_hex_tagged(
                    "verdant/authority/epoch/1:",
                    &prev_kappa,
                    format!(
                        "epoch={next_epoch}|m={next_quorum_threshold_m}|boundary={}|admins={}",
                        self.boundary_key_epoch,
                        next_active.join(",")
                    )
                    .as_bytes(),
                );
                self.epochs.push(AuthorityEpochRecord {
                    epoch: next_epoch,
                    prev_epoch_kappa: prev_kappa,
                    epoch_kappa: epoch_kappa.clone(),
                    active_admins: next_active,
                    quorum_threshold_m: next_quorum_threshold_m,
                    boundary_key_epoch: self.boundary_key_epoch,
                    provo_summary: format!(
                        "Revoked admin {target_admin_prism_id} ({reason:?}) at epoch {next_epoch}; rotated boundary key to {}",
                        self.boundary_key_epoch
                    ),
                    co_signers,
                });
                Ok(AuthorityOutcome {
                    spec: "verdant/sovereign-authority-outcome/1".to_string(),
                    current_epoch: self.current_epoch,
                    boundary_key_epoch: self.boundary_key_epoch,
                    active_admin_count: remaining_admins,
                    quorum_threshold_m: self.quorum_threshold_m,
                    subject_kappa: epoch_kappa,
                    invalidated_requests,
                    revoked_grants,
                    summary: format!(
                        "Admin {target_admin_prism_id} departed ({reason:?}) at epoch {next_epoch}; invalidated {invalidated_requests} pending requests and {revoked_grants} standing grants"
                    ),
                    certificate: None,
                })
            }

            AuthorityCommand::GrantStandingAccess {
                grantee_prism_id,
                scope,
                attesting_admins,
            } => {
                if !grantee_prism_id.starts_with("prism:") || scope.trim().is_empty() {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "standing grant requires prism: grantee and non-empty scope"
                            .to_string(),
                    });
                }
                let mut valid_attestors = BTreeSet::new();
                for (aid, sec) in &attesting_admins {
                    if let Some(m) = self.admins.get(aid) {
                        let fp =
                            sha256_hex_tagged("verdant/authority/keyfp/1:", aid, sec.as_bytes());
                        if fp == m.key_fingerprint
                            && admin_signature_valid_at_epoch(
                                active_set.contains(aid),
                                m.revoked_at_epoch.is_some(),
                                self.current_epoch,
                                self.current_epoch,
                            )
                        {
                            valid_attestors.insert(aid.clone());
                        }
                    }
                }
                if valid_attestors.is_empty() {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "standing grant requires at least one authenticated active admin"
                            .to_string(),
                    });
                }
                let attested_by: Vec<String> = valid_attestors.into_iter().collect();
                let grant_kappa = sha256_hex_tagged(
                    "verdant/authority/grant/1:",
                    &grantee_prism_id,
                    format!(
                        "{scope}|epoch={}|by={}",
                        self.current_epoch,
                        attested_by.join(",")
                    )
                    .as_bytes(),
                );
                self.standing_grants.insert(
                    grant_kappa.clone(),
                    StandingGrantRecord {
                        grant_kappa: grant_kappa.clone(),
                        grantee_prism_id: grantee_prism_id.clone(),
                        scope: scope.clone(),
                        granted_epoch: self.current_epoch,
                        attested_by,
                        active: true,
                        revoked_reason: None,
                    },
                );
                Ok(AuthorityOutcome {
                    spec: "verdant/sovereign-authority-outcome/1".to_string(),
                    current_epoch: self.current_epoch,
                    boundary_key_epoch: self.boundary_key_epoch,
                    active_admin_count: active_n,
                    quorum_threshold_m: self.quorum_threshold_m,
                    subject_kappa: grant_kappa,
                    invalidated_requests: 0,
                    revoked_grants: 0,
                    summary: format!("Granted standing scope `{scope}` to {grantee_prism_id}"),
                    certificate: None,
                })
            }

            AuthorityCommand::CoordinateWorkItem {
                author_prism_id,
                author_secret,
                kind,
                title,
                target_path,
                required_review_group,
                milestone,
                stitch_shard_count,
            } => {
                let provo_ok = author_prism_id.starts_with("prism:")
                    && !author_secret.trim().is_empty()
                    && !title.trim().is_empty();
                let path_ok =
                    !target_path.trim().is_empty() && !required_review_group.trim().is_empty();
                if !group_coordination_envelope_valid(
                    provo_ok,
                    stitch_shard_count,
                    four_u64(),
                    path_ok,
                    false,
                ) {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "work-item coordination requires prism: author, non-empty title/path/review-group, and >= 4 Stitch shards"
                            .to_string(),
                    });
                }
                let preimage = format!(
                    "{author_prism_id}|{kind}|{title}|{target_path}|{required_review_group}|{milestone}|{stitch_shard_count}"
                );
                let item_kappa = sha256_hex_tagged(
                    "verdant/authority/workitem/1:",
                    &author_prism_id,
                    preimage.as_bytes(),
                );
                let sig = sha256_hex_tagged(
                    "verdant/authority/sig/1:",
                    &item_kappa,
                    format!("{author_prism_id}|{author_secret}").as_bytes(),
                );
                self.work_items.insert(
                    item_kappa.clone(),
                    WorkItemRecord {
                        item_kappa: item_kappa.clone(),
                        author_prism_id,
                        kind,
                        title: title.clone(),
                        target_path,
                        required_review_group,
                        milestone,
                        stitch_shard_count,
                        signature_hex: sig,
                    },
                );
                Ok(AuthorityOutcome {
                    spec: "verdant/sovereign-authority-outcome/1".to_string(),
                    current_epoch: self.current_epoch,
                    boundary_key_epoch: self.boundary_key_epoch,
                    active_admin_count: active_n,
                    quorum_threshold_m: self.quorum_threshold_m,
                    subject_kappa: item_kappa,
                    invalidated_requests: 0,
                    revoked_grants: 0,
                    summary: format!("Recorded GitHub-free PROV-O work item `{title}`"),
                    certificate: None,
                })
            }

            AuthorityCommand::VerifyHistoricalCommit {
                finalized_epoch,
                signer_prism_ids,
            } => {
                let epoch_rec = self
                    .epochs
                    .iter()
                    .find(|e| e.epoch == finalized_epoch)
                    .ok_or_else(|| StitchCustodyError::InvalidJournalSignature {
                        detail: format!("unknown historical authority epoch {finalized_epoch}"),
                    })?;
                let mut valid_hist_signers = BTreeSet::new();
                for sid in &signer_prism_ids {
                    if !epoch_rec.active_admins.contains(sid) {
                        continue;
                    }
                    if let Some(m) = self.admins.get(sid) {
                        if let Some(rev_ep) = m.revoked_at_epoch {
                            let was_comp =
                                m.departure_reason == Some(DepartureReason::KeyCompromise);
                            let comp_floor = m.compromise_floor_epoch.unwrap_or(rev_ep);
                            if !historical_commit_valid(
                                finalized_epoch,
                                rev_ep,
                                was_comp,
                                comp_floor,
                                true,
                            ) {
                                continue;
                            }
                        }
                        valid_hist_signers.insert(sid.clone());
                    }
                }
                if (valid_hist_signers.len() as u64) < epoch_rec.quorum_threshold_m {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: format!(
                            "historical commit at epoch {finalized_epoch} quarantined or below historical quorum {}",
                            epoch_rec.quorum_threshold_m
                        ),
                    });
                }
                Ok(AuthorityOutcome {
                    spec: "verdant/sovereign-authority-outcome/1".to_string(),
                    current_epoch: self.current_epoch,
                    boundary_key_epoch: self.boundary_key_epoch,
                    active_admin_count: active_n,
                    quorum_threshold_m: self.quorum_threshold_m,
                    subject_kappa: epoch_rec.epoch_kappa.clone(),
                    invalidated_requests: 0,
                    revoked_grants: 0,
                    summary: format!(
                        "Historical commit at epoch {finalized_epoch} verified ({} valid signers >= M={})",
                        valid_hist_signers.len(),
                        epoch_rec.quorum_threshold_m
                    ),
                    certificate: None,
                })
            }

            AuthorityCommand::VerifyChain => {
                if self.epochs.is_empty() || self.epochs[0].epoch != 1 {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "authority epoch chain missing genesis epoch 1".to_string(),
                    });
                }
                let mut prev_kappa = self.genesis_kappa.clone();
                let mut prev_epoch = 0_u64;
                for ep in &self.epochs {
                    let expected = sha256_hex_tagged(
                        "verdant/authority/epoch/1:",
                        &prev_kappa,
                        format!(
                            "epoch={}|m={}|boundary={}|admins={}",
                            ep.epoch,
                            ep.quorum_threshold_m,
                            ep.boundary_key_epoch,
                            ep.active_admins.join(",")
                        )
                        .as_bytes(),
                    );
                    if ep.prev_epoch_kappa != prev_kappa
                        || ep.epoch_kappa != expected
                        || ep.epoch != prev_epoch.saturating_add(1)
                        || !quorum_threshold_valid(
                            ep.quorum_threshold_m,
                            ep.active_admins.len() as u64,
                        )
                    {
                        return Err(StitchCustodyError::InvalidJournalSignature {
                            detail: format!(
                                "authority epoch chain integrity or rollback violation at epoch {}",
                                ep.epoch
                            ),
                        });
                    }
                    prev_kappa = ep.epoch_kappa.clone();
                    prev_epoch = ep.epoch;
                }
                if prev_epoch != self.current_epoch {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "authority ledger current_epoch disagrees with tip epoch"
                            .to_string(),
                    });
                }
                Ok(AuthorityOutcome {
                    spec: "verdant/sovereign-authority-outcome/1".to_string(),
                    current_epoch: self.current_epoch,
                    boundary_key_epoch: self.boundary_key_epoch,
                    active_admin_count: active_n,
                    quorum_threshold_m: self.quorum_threshold_m,
                    subject_kappa: prev_kappa,
                    invalidated_requests: 0,
                    revoked_grants: 0,
                    summary: format!(
                        "Verified monotonic authority chain across {} epochs (M={}/N={})",
                        self.epochs.len(),
                        self.quorum_threshold_m,
                        active_n
                    ),
                    certificate: None,
                })
            }

            AuthorityCommand::IssueApprovalCertificate { request_kappa } => {
                let req = self.requests.get(&request_kappa).ok_or_else(|| {
                    StitchCustodyError::InvalidJournalSignature {
                        detail: format!("unknown signing request `{request_kappa}`"),
                    }
                })?;
                if req.state != SigningRequestState::Finalized {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: format!(
                            "cannot issue ApprovalCertificate for non-finalized request `{request_kappa}` ({:?})",
                            req.state
                        ),
                    });
                }
                let finalized_epoch = req.finalized_epoch.unwrap_or(self.current_epoch);
                let mut approvers: Vec<String> = req
                    .reviews
                    .iter()
                    .filter(|r| r.verdict == ReviewVerdictKind::Approve)
                    .map(|r| r.admin_prism_id.clone())
                    .collect();
                approvers.sort();
                approvers.dedup();
                let ep_m = self
                    .epochs
                    .iter()
                    .find(|e| e.epoch == finalized_epoch)
                    .map_or(self.quorum_threshold_m, |e| e.quorum_threshold_m);
                if !verdant_tensor::scalars::approval_certificate_quorum_met(
                    approvers.len() as u64,
                    ep_m,
                ) {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "finalized signing request lacks active quorum approvals"
                            .to_string(),
                    });
                }
                let cert_preimage = format!(
                    "req={}|target={}|act={:?}|ep={finalized_epoch}|m={ep_m}|approvers={}",
                    req.request_kappa,
                    req.target_kappa,
                    req.action,
                    approvers.join(",")
                );
                let certificate_kappa = sha256_hex_tagged(
                    "verdant/authority/cert/1:",
                    &req.request_kappa,
                    cert_preimage.as_bytes(),
                );
                let cert = ApprovalCertificate {
                    spec: "verdant/approval-certificate/1".to_string(),
                    request_kappa: req.request_kappa.clone(),
                    target_kappa: req.target_kappa.clone(),
                    action: req.action,
                    finalized_epoch,
                    quorum_threshold_m: ep_m,
                    approver_prism_ids: approvers,
                    certificate_kappa: certificate_kappa.clone(),
                };
                Ok(AuthorityOutcome {
                    spec: "verdant/sovereign-authority-outcome/1".to_string(),
                    current_epoch: self.current_epoch,
                    boundary_key_epoch: self.boundary_key_epoch,
                    active_admin_count: active_n,
                    quorum_threshold_m: self.quorum_threshold_m,
                    subject_kappa: certificate_kappa,
                    invalidated_requests: 0,
                    revoked_grants: 0,
                    summary: format!(
                        "Issued ApprovalCertificate for {:?} target {} at epoch {finalized_epoch}",
                        cert.action, cert.target_kappa
                    ),
                    certificate: Some(cert),
                })
            }

            AuthorityCommand::VerifyApprovalCertificate {
                certificate,
                expected_target_kappa,
                expected_action,
            } => {
                if certificate.spec != "verdant/approval-certificate/1"
                    || certificate.target_kappa != expected_target_kappa
                    || certificate.action != expected_action
                    || !verdant_tensor::scalars::approval_certificate_epoch_valid(
                        certificate.finalized_epoch,
                        self.current_epoch,
                    )
                {
                    return Err(StitchCustodyError::InvalidJournalSignature {
                        detail: "ApprovalCertificate spec, target, action, or epoch mismatch"
                            .to_string(),
                    });
                }
                let mut sorted = certificate.approver_prism_ids.clone();
                sorted.sort();
                sorted.dedup();
                let cert_preimage = format!(
                    "req={}|target={}|act={:?}|ep={}|m={}|approvers={}",
                    certificate.request_kappa,
                    certificate.target_kappa,
                    certificate.action,
                    certificate.finalized_epoch,
                    certificate.quorum_threshold_m,
                    sorted.join(",")
                );
                let expected_cert_kappa = sha256_hex_tagged(
                    "verdant/authority/cert/1:",
                    &certificate.request_kappa,
                    cert_preimage.as_bytes(),
                );
                if expected_cert_kappa != certificate.certificate_kappa {
                    return Err(StitchCustodyError::PayloadDigestMismatch {
                        detail: "ApprovalCertificate cryptographic commitment mismatch".to_string(),
                    });
                }
                let _ = self.execute(AuthorityCommand::VerifyChain)?;
                let _ = self.execute(AuthorityCommand::VerifyHistoricalCommit {
                    finalized_epoch: certificate.finalized_epoch,
                    signer_prism_ids: sorted,
                })?;
                Ok(AuthorityOutcome {
                    spec: "verdant/sovereign-authority-outcome/1".to_string(),
                    current_epoch: self.current_epoch,
                    boundary_key_epoch: self.boundary_key_epoch,
                    active_admin_count: active_n,
                    quorum_threshold_m: self.quorum_threshold_m,
                    subject_kappa: certificate.certificate_kappa.clone(),
                    invalidated_requests: 0,
                    revoked_grants: 0,
                    summary: format!(
                        "Verified ApprovalCertificate {} for {:?} target {}",
                        certificate.certificate_kappa, expected_action, expected_target_kappa
                    ),
                    certificate: Some(certificate),
                })
            }
        }
    }
}

#[path = "authority_cert.rs"]
mod authority_cert;
pub use authority_cert::*;
