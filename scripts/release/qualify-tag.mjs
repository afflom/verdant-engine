#!/usr/bin/env node
/**
 * Release Tag Qualification Gate (`VE-52`, `mgr-change-release`).
 *
 * Refuses to cut or publish a release unless:
 *   1. `Cargo.toml` `[workspace.package].version` matches `site/generated/release-ledger.json`
 *      and `site/generated/self-host-ratchet-manifest.toml` and `crates/verdant-wasm/Cargo.toml`.
 *   2. If a target tag (`--tag vX.Y.Z` or `GITHUB_REF_NAME`) is supplied, it matches `v<version>`.
 *   3. `frontier.toml` (`spec = "uorust/register/1"`) is present with valid `[lift]` telemetry.
 *   4. In GitHub Actions (when `GH_TOKEN`, `GITHUB_REPOSITORY`, and `GITHUB_SHA` are present and
 *      `--offline` is not set), confirms `bootstrap.yml` completed with `conclusion == "success"`
 *      on the exact tagged commit SHA.
 */

import { readFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";

const args = process.argv.slice(2);
const flagVal = (name) => {
  const idx = args.indexOf(name);
  return idx !== -1 && idx + 1 < args.length ? args[idx + 1] : null;
};

const repoRoot = resolve(flagVal("--root") || process.cwd());
const offline = args.includes("--offline");
const expectedTag =
  flagVal("--tag") ||
  (process.env.GITHUB_REF_TYPE === "tag" ? process.env.GITHUB_REF_NAME : null);

function fail(msg) {
  console.error(`qualify-tag: REFUSED — ${msg}`);
  process.exit(1);
}

// 1. Parse Cargo.toml [workspace.package].version
const cargoTomlPath = join(repoRoot, "Cargo.toml");
if (!existsSync(cargoTomlPath)) fail("missing Cargo.toml");
const cargoToml = readFileSync(cargoTomlPath, "utf8");
const wsSection = cargoToml.split("[workspace.package]")[1] || "";
const verMatch = wsSection.match(/^\s*version\s*=\s*"([^"]+)"/m);
if (!verMatch) fail("could not locate [workspace.package].version in Cargo.toml");
const cargoVersion = verMatch[1].trim();

// 2. Parse crates/verdant-wasm/Cargo.toml wasm32 verdant-core version
const wasmTomlPath = join(repoRoot, "crates", "verdant-wasm", "Cargo.toml");
if (!existsSync(wasmTomlPath)) fail("missing crates/verdant-wasm/Cargo.toml");
const wasmToml = readFileSync(wasmTomlPath, "utf8");
const wasmDepMatch = wasmToml.match(/verdant-core\s*=\s*\{\s*version\s*=\s*"([^"]+)"/);
if (!wasmDepMatch) fail("missing wasm32 verdant-core version in crates/verdant-wasm/Cargo.toml");
if (wasmDepMatch[1] !== cargoVersion) {
  fail(
    `crates/verdant-wasm/Cargo.toml verdant-core version (${wasmDepMatch[1]}) != Cargo.toml (${cargoVersion})`,
  );
}

// 3. Parse site/generated/release-ledger.json
const ledgerPath = join(repoRoot, "site", "generated", "release-ledger.json");
if (!existsSync(ledgerPath)) fail("missing site/generated/release-ledger.json");
const ledger = JSON.parse(readFileSync(ledgerPath, "utf8"));
const ledgerVersion = `${ledger.currentVersion.major}.${ledger.currentVersion.minor}.${ledger.currentVersion.patch}`;
if (ledgerVersion !== cargoVersion) {
  fail(`release-ledger.json version (${ledgerVersion}) != Cargo.toml (${cargoVersion})`);
}

// 4. Parse site/generated/self-host-ratchet-manifest.toml
const manifestPath = join(repoRoot, "site", "generated", "self-host-ratchet-manifest.toml");
if (!existsSync(manifestPath)) fail("missing site/generated/self-host-ratchet-manifest.toml");
const manifestToml = readFileSync(manifestPath, "utf8");
const manVerMatch = manifestToml.match(/^\s*version\s*=\s*"([^"]+)"/m);
if (!manVerMatch || manVerMatch[1] !== cargoVersion) {
  fail(
    `self-host-ratchet-manifest.toml version (${manVerMatch?.[1] || "missing"}) != Cargo.toml (${cargoVersion})`,
  );
}

// 5. Verify frontier.toml (uorust MIR proof frontier register)
const frontierPath = join(repoRoot, "frontier.toml");
if (!existsSync(frontierPath)) fail("missing frontier.toml (uorust MIR proof frontier)");
const frontierToml = readFileSync(frontierPath, "utf8");
if (!frontierToml.includes('spec = "uorust/register/1"')) {
  fail("frontier.toml missing spec = \"uorust/register/1\"");
}
const liftedMatch = frontierToml.match(/^\s*lifted\s*=\s*(\d+)/m);
const funcsMatch = frontierToml.match(/^\s*functions\s*=\s*(\d+)/m);
if (!liftedMatch || !funcsMatch || Number(liftedMatch[1]) < 1) {
  fail("frontier.toml missing valid [lift] counts");
}

// 6. Verify requested tag matches v<version>
if (expectedTag) {
  const cleanTag = expectedTag.replace(/^refs\/tags\//, "");
  if (cleanTag !== `v${cargoVersion}`) {
    fail(`tag ${cleanTag} does not match Cargo.toml / release-ledger version v${cargoVersion}`);
  }
}

// 7. In GitHub Actions CI, confirm bootstrap.yml passed on this exact commit SHA:
const ghToken = process.env.GH_TOKEN || process.env.GITHUB_TOKEN || "";
const ghRepo = process.env.GITHUB_REPOSITORY || "";
const ghSha = flagVal("--sha") || process.env.GITHUB_SHA || "";
let bootstrapRunId = null;

if (!offline && ghToken && ghRepo && ghSha) {
  const url = `https://api.github.com/repos/${ghRepo}/actions/workflows/bootstrap.yml/runs?head_sha=${encodeURIComponent(ghSha)}&per_page=20`;
  const res = await fetch(url, {
    headers: {
      Authorization: `Bearer ${ghToken}`,
      Accept: "application/vnd.github+json",
      "X-GitHub-Api-Version": "2022-11-28",
      "User-Agent": "verdant-qualify-tag",
    },
  });
  if (!res.ok) {
    fail(`GitHub API bootstrap check returned HTTP ${res.status} for ${ghRepo}@${ghSha}`);
  }
  const data = await res.json();
  const runs = Array.isArray(data.workflow_runs) ? data.workflow_runs : [];
  const green = runs.find((r) => r.status === "completed" && r.conclusion === "success");
  if (!green) {
    fail(
      `no passing bootstrap.yml run found for ${ghRepo}@${ghSha} (checked ${runs.length} runs; Cadence::aRedMainNeverCuts)`,
    );
  }
  bootstrapRunId = green.id;
}

console.log(
  `qualify-tag: PASS — v${cargoVersion} (epoch ${ledger.releaseEpoch}, frontier ${liftedMatch[1]}/${funcsMatch[1]} lifted${bootstrapRunId ? `, bootstrap run #${bootstrapRunId} GREEN` : ", offline mode"})`,
);
