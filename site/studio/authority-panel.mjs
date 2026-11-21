// Zero-Account Sovereign Signing-Request & Admin Quorum UI (`site/studio/authority-panel.mjs`).
// Mirrors `app/src/Governance.lex.tex` and `crates/verdant-core/src/authority.rs` in pure browser
// WebCrypto SHA-256 + localStorage so teams can bootstrap a Genesis Owner, submit signing requests
// for VerdantVcs commits, review (Approve / Reject with M-of-N separation of duties), and revoke
// departed Admins without GitHub accounts or extra Wasm transfer weight.

const AUTHORITY_STORAGE_KEY = "verdant-sovereign-authority-v1";

async function sha256Hex(text) {
  const bytes = new TextEncoder().encode(String(text || ""));
  if (globalThis.crypto?.subtle) {
    const digest = await crypto.subtle.digest("SHA-256", bytes);
    return Array.from(new Uint8Array(digest), (b) => b.toString(16).padStart(2, "0")).join("");
  }
  let h = 2166136261 >>> 0;
  for (const b of bytes) {
    h ^= b;
    h = Math.imul(h, 16777619) >>> 0;
  }
  return h.toString(16).padStart(8, "0").repeat(8);
}

function defaultLedger(ownerPrismId) {
  const owner = ownerPrismId && ownerPrismId.startsWith("prism:") ? ownerPrismId : "prism:studio:genesis-owner";
  return {
    spec: "verdant/sovereign-authority-ledger/1",
    authorityEpoch: 1,
    boundaryKeyEpoch: 1,
    thresholdM: 1,
    prevEpochKappa: "genesis",
    epochKappa: "genesis",
    admins: [
      {
        prismId: owner,
        label: "Genesis Owner",
        active: true,
        addedAtEpoch: 1,
        revokedAtEpoch: null,
        departureReason: null,
      },
    ],
    requests: [],
  };
}

export function initStudioAuthorityPanel({ $, statusEl, chip, callWorker, getLocalPrismId, getLatestCommitKappa }) {
  const summaryEl = $("studio-auth-summary");
  const listEl = $("studio-auth-requests");
  const actionInput = $("studio-auth-action-input");
  const coAdminInput = $("studio-auth-coadmin-input");
  const submitBtn = $("studio-auth-submit-btn");
  const addAdminBtn = $("studio-auth-add-admin-btn");
  const revokeBtn = $("studio-auth-revoke-btn");

  const ownerPrismId = typeof getLocalPrismId === "function" ? getLocalPrismId() : "prism:studio:genesis-owner";

  let ledger = defaultLedger(ownerPrismId);
  try {
    const raw = localStorage.getItem(AUTHORITY_STORAGE_KEY);
    if (raw) {
      const parsed = JSON.parse(raw);
      if (parsed?.spec === "verdant/sovereign-authority-ledger/1" && Array.isArray(parsed.admins)) {
        ledger = parsed;
      }
    }
  } catch {}

  const saveLedger = () => {
    try {
      localStorage.setItem(AUTHORITY_STORAGE_KEY, JSON.stringify(ledger));
    } catch {}
  };

  const activeAdmins = () => ledger.admins.filter((a) => a.active);

  const recomputeEpochKappa = async () => {
    const canonical = JSON.stringify({
      epoch: ledger.authorityEpoch,
      prev: ledger.prevEpochKappa,
      m: ledger.thresholdM,
      keyEpoch: ledger.boundaryKeyEpoch,
      active: activeAdmins().map((a) => a.prismId).sort(),
    });
    ledger.epochKappa = await sha256Hex(`verdant/authority-epoch/v1:${canonical}`);
  };

  const renderAuthority = () => {
    const actives = activeAdmins();
    if (summaryEl) {
      summaryEl.textContent =
        `Epoch ${ledger.authorityEpoch} · Quorum ${ledger.thresholdM}-of-${actives.length} · ` +
        `Key e${ledger.boundaryKeyEpoch} · ${ledger.epochKappa.slice(0, 10)}…`;
    }
    if (!listEl) return;
    listEl.replaceChildren();
    if (ledger.requests.length === 0) {
      const empty = document.createElement("div");
      empty.className = "proj-empty";
      empty.textContent =
        "No signing requests yet. Submit a request to govern commits or releases under your local Prism ID quorum.";
      listEl.append(empty);
      return;
    }
    for (const req of ledger.requests.slice(0, 6)) {
      const row = document.createElement("div");
      row.className = "insp-rev-item";
      const top = document.createElement("div");
      top.className = "insp-rev-top";
      const title = document.createElement("strong");
      title.textContent = `${req.requestId} · ${req.state} (${req.validApprovals.length}/${req.requiredM})`;
      top.append(title);

      const meta = document.createElement("div");
      meta.className = "ide-pane-sub";
      meta.textContent = `${req.summary} · commit ${String(req.targetKappa || "workspace").slice(0, 10)}…`;

      row.append(top, meta);
      if (req.state === "Unfinalized") {
        const actions = document.createElement("div");
        actions.className = "insp-row";
        const approveBtn = document.createElement("button");
        approveBtn.type = "button";
        approveBtn.className = "st-btn primary sm";
        approveBtn.textContent = "Approve";
        approveBtn.addEventListener("click", () => {
          const reviewer = pickNextEligibleReviewer(req);
          void reviewRequest(req.requestId, reviewer, "Approve");
        });
        const rejectBtn = document.createElement("button");
        rejectBtn.type = "button";
        rejectBtn.className = "st-btn danger sm";
        rejectBtn.textContent = "Reject";
        rejectBtn.addEventListener("click", () => {
          const reviewer = pickNextEligibleReviewer(req);
          void reviewRequest(req.requestId, reviewer, "Reject");
        });
        actions.append(approveBtn, rejectBtn);
        row.append(actions);
      } else if (req.certificateKappa) {
        const cert = document.createElement("div");
        cert.className = "ide-pane-sub";
        cert.textContent = `Cert ${req.certificateKappa.slice(0, 14)}… · e${req.finalizedEpoch}`;
        row.append(cert);
      }
      listEl.append(row);
    }
  };

  const pickNextEligibleReviewer = (req) => {
    const actives = activeAdmins();
    const already = new Set(req.validApprovals.map((a) => a.adminPrismId));
    const distinctNonRequester = actives.find(
      (a) => !already.has(a.prismId) && (actives.length === 1 || a.prismId !== req.requesterPrismId),
    );
    if (distinctNonRequester) return distinctNonRequester.prismId;
    const anyRemaining = actives.find((a) => !already.has(a.prismId));
    return anyRemaining ? anyRemaining.prismId : actives[0]?.prismId || ownerPrismId;
  };

  const addAdmin = async (prismIdRaw, label = "Co-Admin") => {
    const prismId = String(prismIdRaw || "").trim();
    if (!prismId.startsWith("prism:")) {
      const msg = "Admin identity must start with prism:";
      chip?.(statusEl, msg);
      return { ok: false, error: msg };
    }
    if (ledger.admins.some((a) => a.prismId === prismId && a.active)) {
      return { ok: false, error: "Admin already active" };
    }
    ledger.prevEpochKappa = ledger.epochKappa;
    ledger.authorityEpoch += 1;
    ledger.admins.push({
      prismId,
      label: String(label || "Co-Admin").trim() || "Co-Admin",
      active: true,
      addedAtEpoch: ledger.authorityEpoch,
      revokedAtEpoch: null,
      departureReason: null,
    });
    const n = activeAdmins().length;
    ledger.thresholdM = n >= 2 ? Math.min(2, n) : 1;
    await recomputeEpochKappa();
    saveLedger();
    renderAuthority();
    chip?.(statusEl, `Admitted ${prismId} · Epoch ${ledger.authorityEpoch} (${ledger.thresholdM}-of-${n} quorum)`);
    return { ok: true, epoch: ledger.authorityEpoch, thresholdM: ledger.thresholdM, activeCount: n };
  };

  const submitSigningRequest = async ({ summary, requesterPrismId, targetKappa } = {}) => {
    const cleanSummary = String(summary || actionInput?.value || "Sign latest VerdantVcs commit").trim();
    const requester = String(requesterPrismId || `prism:dev:${ownerPrismId.slice(-8)}`).trim();
    const commitKappa = String(targetKappa || getLatestCommitKappa?.() || ledger.epochKappa);
    const reqNum = ledger.requests.length + 1;
    const requestId = `SR-${String(reqNum).padStart(3, "0")}`;
    const payloadKappa = await sha256Hex(`${requestId}|${requester}|${commitKappa}|${cleanSummary}`);
    const record = {
      requestId,
      requesterPrismId: requester,
      summary: cleanSummary,
      targetKappa: commitKappa,
      payloadKappa,
      submittedEpoch: ledger.authorityEpoch,
      requiredM: ledger.thresholdM,
      state: "Unfinalized",
      validApprovals: [],
      revokedApprovals: [],
      finalizedEpoch: null,
      certificateKappa: null,
    };
    ledger.requests.unshift(record);
    ledger.requests = ledger.requests.slice(0, 16);
    saveLedger();
    renderAuthority();
    chip?.(statusEl, `Submitted signing request ${requestId} (${ledger.thresholdM}-of-${activeAdmins().length} required)`);
    return { ok: true, request: record };
  };

  const reviewRequest = async (requestId, reviewerPrismId, verdict = "Approve") => {
    const req = ledger.requests.find((r) => r.requestId === requestId);
    if (!req) return { ok: false, error: `Unknown request ${requestId}` };
    if (req.state !== "Unfinalized") return { ok: false, error: `Request ${requestId} is already ${req.state}` };
    const actives = activeAdmins();
    const reviewer = actives.find((a) => a.prismId === reviewerPrismId);
    if (!reviewer) {
      return { ok: false, error: `Reviewer ${reviewerPrismId} is not an active Admin` };
    }
    if (actives.length >= 2 && reviewer.prismId === req.requesterPrismId) {
      return { ok: false, error: "Separation of duties: requester cannot self-approve when N >= 2" };
    }
    if (verdict === "Reject") {
      req.state = "Rejected";
      req.finalizedEpoch = ledger.authorityEpoch;
      saveLedger();
      renderAuthority();
      chip?.(statusEl, `Rejected signing request ${requestId} by ${reviewer.prismId}`);
      return { ok: true, state: req.state, request: req };
    }
    if (req.validApprovals.some((a) => a.adminPrismId === reviewer.prismId)) {
      return { ok: false, error: "Admin already approved this request" };
    }
    const sigHex = await sha256Hex(`${req.payloadKappa}|${reviewer.prismId}|e${ledger.authorityEpoch}`);
    req.validApprovals.push({
      adminPrismId: reviewer.prismId,
      approvedAtEpoch: ledger.authorityEpoch,
      signatureHex: sigHex,
    });
    if (req.validApprovals.length >= req.requiredM) {
      req.state = "Finalized";
      req.finalizedEpoch = ledger.authorityEpoch;
      req.certificateKappa = await sha256Hex(
        `${req.payloadKappa}|${req.validApprovals.map((a) => a.signatureHex).join(",")}|e${ledger.authorityEpoch}`,
      );
      chip?.(statusEl, `Finalized ${requestId} · Certificate ${req.certificateKappa.slice(0, 12)}…`);
    } else {
      chip?.(statusEl, `Approved ${requestId} (${req.validApprovals.length}/${req.requiredM})`);
    }
    saveLedger();
    renderAuthority();
    return { ok: true, state: req.state, request: req };
  };

  const departAdmin = async (adminPrismId, reason = "VoluntaryExit") => {
    const actives = activeAdmins();
    if (actives.length <= 1) {
      const msg = "Anti-lockout guard: cannot revoke the sole remaining active Admin";
      chip?.(statusEl, msg);
      return { ok: false, error: msg };
    }
    const target = actives.find((a) => a.prismId === adminPrismId);
    if (!target) {
      return { ok: false, error: `Admin ${adminPrismId} is not currently active` };
    }
    ledger.prevEpochKappa = ledger.epochKappa;
    ledger.authorityEpoch += 1;
    ledger.boundaryKeyEpoch += 1;
    target.active = false;
    target.revokedAtEpoch = ledger.authorityEpoch;
    target.departureReason = reason;
    const remaining = activeAdmins().length;
    ledger.thresholdM = Math.min(ledger.thresholdM, remaining);

    let revokedCount = 0;
    for (const req of ledger.requests) {
      if (req.state !== "Unfinalized") continue;
      const kept = [];
      for (const app of req.validApprovals) {
        if (app.adminPrismId === adminPrismId) {
          req.revokedApprovals.push({ ...app, revokedAtEpoch: ledger.authorityEpoch, reason });
          revokedCount += 1;
        } else {
          kept.push(app);
        }
      }
      req.validApprovals = kept;
      req.requiredM = Math.min(req.requiredM, remaining);
    }

    await recomputeEpochKappa();
    saveLedger();
    renderAuthority();
    chip?.(
      statusEl,
      `Revoked ${adminPrismId} at e${ledger.authorityEpoch} · invalidated ${revokedCount} unfinalized approval(s) · key rotated to e${ledger.boundaryKeyEpoch}`,
    );
    return {
      ok: true,
      authorityEpoch: ledger.authorityEpoch,
      boundaryKeyEpoch: ledger.boundaryKeyEpoch,
      revokedApprovals: revokedCount,
      remainingAdmins: remaining,
    };
  };

  const resetLedger = async () => {
    ledger = defaultLedger(ownerPrismId);
    await recomputeEpochKappa();
    saveLedger();
    renderAuthority();
    return { ok: true, ledger };
  };

  submitBtn?.addEventListener("click", () => {
    void submitSigningRequest();
  });
  addAdminBtn?.addEventListener("click", () => {
    const nextId =
      String(coAdminInput?.value || "").trim() ||
      `prism:admin:peer-${String(ledger.admins.length + 1).padStart(2, "0")}`;
    void addAdmin(nextId, `Admin #${ledger.admins.length + 1}`);
  });
  revokeBtn?.addEventListener("click", () => {
    const actives = activeAdmins();
    if (actives.length <= 1) {
      chip?.(statusEl, "Anti-lockout guard: add a co-Admin before revoking a departed Admin.");
      return;
    }
    const departed = actives[actives.length - 1];
    void departAdmin(departed.prismId, "VoluntaryExit");
  });

  void recomputeEpochKappa().then(() => {
    saveLedger();
    renderAuthority();
  });

  const execWasmAuthority = async (request) => {
    if (typeof callWorker !== "function") {
      return { ok: false, error: "worker bridge unavailable" };
    }
    const resp = await callWorker("authority-exec", { request });
    return resp?.result || { ok: false, error: resp?.error || "authority-exec failed" };
  };

  const exportLedgerJson = () => JSON.stringify(ledger, null, 2);

  return {
    getLedger: () => structuredClone(ledger),
    exportLedgerJson,
    execWasmAuthority,
    addAdmin,
    submitSigningRequest,
    reviewRequest,
    departAdmin,
    resetLedger,
  };
}
