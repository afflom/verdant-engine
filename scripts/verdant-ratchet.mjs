#!/usr/bin/env node
/**
 * Single-Command Autonomous Ratchet Orchestrator (`npm run ratchet`, `VE-55`).
 *
 * Keeps the entire Verdant ratchet self-enforcing, auto-tightening, and hands-off:
 *   1. `xtask check-model --write` (syncs `CONFORMANCE.md` with `model/ids.toml` & `authorities.toml`)
 *   2. `prism-fidelity --no-ratchet` (updates `Fidelity.lex.tex`, `Provenance.lex.tex`, `fidelity.toml`)
 *   3. Kappa-memoized locked SDK `lexlean lock && lexlean fmt && lexlean lock` (only when `app/` changed)
 *   4. Kappa-memoized `uorust` MIR proof frontier refresh (`frontier.toml`, auto-discovered if installed)
 *   5. Kappa-memoized Wasm rebuild (`scripts/sync-prism-wasm.mjs`)
 *   6. `verdant operator ratchet-lift --tighten` (synthesizes manifests, syncs `Cargo.toml`, updates
 *      `Cargo.lock` if needed, binds `frontier_rust_tree_sha256`, one-way auto-tightens `[frontier_ratchet]`
 *      & `[surface_ratchet]` ceilings in `model/parametric-ratchet.toml`, seals `immutability-seal.json`
 *      and `self-host-ratchet-manifest.toml`, updates `site/frame/frame-pin.json`)
 *   7. Final `prism-fidelity --check` proving zero magic constants, 100% self-hosting, live frontier
 *      source binding, and surface weight/line-count watermarks!
 *
 * Usage:
 *   npm run ratchet [-- --advance-minor|--advance-major|--advance-patch] [--summary "..."] [--no-tighten] [--force]
 */

import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url));
const cliArgs = process.argv.slice(2);
const force = cliArgs.includes("--force");
const noTighten = cliArgs.includes("--no-tighten");
const noDocker = cliArgs.includes("--no-docker");
const cacheDir = join(root, ".verdant-cache");
mkdirSync(cacheDir, { recursive: true });

function walkFiles(dir, suffix = null, out = []) {
  if (!existsSync(dir)) return out;
  for (const entry of readdirSync(dir).sort()) {
    if (entry.startsWith(".") || entry === "target" || entry === "node_modules") continue;
    const p = join(dir, entry);
    const st = statSync(p);
    if (st.isDirectory()) walkFiles(p, suffix, out);
    else if (!suffix || entry.endsWith(suffix)) out.push(p);
  }
  return out;
}

function hashFiles(files) {
  const h = createHash("sha256");
  for (const file of files) {
    if (!existsSync(file)) continue;
    h.update(relative(root, file).replace(/\\/g, "/"));
    h.update("\0");
    h.update(readFileSync(file));
    h.update("\0");
  }
  return h.digest("hex");
}

function run(label, cmd, args, opts = {}) {
  const t0 = Date.now();
  const res = spawnSync(cmd, args, {
    cwd: opts.cwd || root,
    stdio: opts.quiet ? ["ignore", "pipe", "pipe"] : "inherit",
    encoding: "utf8",
    env: { ...process.env, ...(opts.env || {}) },
  });
  const ms = Date.now() - t0;
  if (res.status !== 0) {
    if (opts.quiet) {
      if (res.stdout) process.stdout.write(res.stdout);
      if (res.stderr) process.stderr.write(res.stderr);
    }
    throw new Error(`${label} failed (exit ${res.status})`);
  }
  const tail = opts.quiet
    ? (res.stdout || "").trim().split(/\r?\n/).filter(Boolean).pop() || "OK"
    : "OK";
  console.log(`[PASS ${ms}ms] ${label}: ${tail}`);
  return res.stdout || "";
}

function findUorust() {
  const candidates = [
    process.env.VERDANT_UORUST_BIN,
    "/tmp/uorust-bin/uorust-x86_64-unknown-linux-gnu/uorust",
  ].filter(Boolean);
  for (const c of candidates) {
    if (existsSync(c)) return c;
  }
  const which = spawnSync("which", ["uorust"], { encoding: "utf8" });
  if (which.status === 0 && which.stdout.trim()) return which.stdout.trim();
  return null;
}

function computeRustSourceTreeSha256() {
  const roots = [
    join(root, "crates", "verdant-core", "src"),
    join(root, "crates", "verdant-tensor", "src"),
    join(root, "crates", "verdant-wasm", "src"),
    join(root, "crates", "verdant-cli", "src"),
  ];
  const files = roots.flatMap((r) => walkFiles(r, ".rs")).sort();
  return hashFiles(files);
}

const tStart = Date.now();

// 1. Model & Conformance check-model --write
run("xtask check-model --write", "cargo", ["run", "--locked", "-q", "-p", "xtask", "--", "check-model", "--write"], {
  quiet: true,
});

// 2. Refresh fidelity & provenance (no ratchet yet because self-host manifest hasn't lifted edits yet)
run(
  "prism-fidelity refresh",
  "cargo",
  [
    "run",
    "--locked",
    "-q",
    "-p",
    "prism-fidelity",
    "--",
    "--project",
    "app",
    "--repo",
    ".",
    "--register",
    "fidelity.toml",
    "--no-ratchet",
    "--cite",
    "model/provenance.toml",
    "app/src/Provenance.lex.tex",
    "--measures",
    "model/measures.toml",
    "--lean",
    "app/src/Fidelity.lex.tex",
  ],
  { quiet: true },
);

// 3. Kappa-memoized locked SDK `lexlean lock && lexlean fmt && lexlean lock`
const appFiles = [
  join(root, "app", "lexlean.toml"),
  join(root, "app", "prismpm.toml"),
  ...walkFiles(join(root, "app", "src"), ".lex.tex"),
];
const appKappa = hashFiles(appFiles);
const appCacheFile = join(cacheDir, "lexlean-app.sha256");
const prevAppKappa = existsSync(appCacheFile) ? readFileSync(appCacheFile, "utf8").trim() : "";

if (!noDocker && (force || appKappa !== prevAppKappa)) {
  const lockRaw = readFileSync(join(root, "prismpm.lock"), "utf8");
  const imgMatch = lockRaw.match(/"sdk_image"\s*:\s*"([^"]+)"/);
  if (imgMatch) {
    const uid = process.getuid ? `${process.getuid()}:${process.getgid()}` : null;
    const dArgs = ["run", "--rm"];
    if (uid) dArgs.push("-u", uid);
    dArgs.push("-v", `${root}:${root}`, "-w", join(root, "app"), imgMatch[1], "bash", "-c", "lexlean lock && lexlean fmt && lexlean lock");
    run("lexlean fmt & lock (SDK)", "docker", dArgs, { quiet: true });
    // Re-score fidelity after lexlean fmt in case formatting touched Fidelity.lex.tex
    run(
      "prism-fidelity post-fmt",
      "cargo",
      [
        "run",
        "--locked",
        "-q",
        "-p",
        "prism-fidelity",
        "--",
        "--project",
        "app",
        "--repo",
        ".",
        "--register",
        "fidelity.toml",
        "--no-ratchet",
        "--cite",
        "model/provenance.toml",
        "app/src/Provenance.lex.tex",
        "--measures",
        "model/measures.toml",
        "--lean",
        "app/src/Fidelity.lex.tex",
      ],
      { quiet: true },
    );
    writeFileSync(appCacheFile, `${hashFiles(appFiles)}\n`);
  }
} else {
  console.log(`[SKIP 0ms] lexlean fmt & lock: kappa hit (${appKappa.slice(0, 12)}…)`);
}

// 4. Kappa-memoized uorust MIR proof frontier refresh
const rustTreeSha = computeRustSourceTreeSha256();
const frontierCacheFile = join(cacheDir, "uorust-frontier.sha256");
const prevFrontierSha = existsSync(frontierCacheFile) ? readFileSync(frontierCacheFile, "utf8").trim() : "";
const uorustBin = findUorust();
if (uorustBin && (force || rustTreeSha !== prevFrontierSha || !existsSync(join(root, "frontier.toml")))) {
  const tmpOut = join(tmpdir(), `verdant-uorust-ratchet-${process.pid}`);
  run("uorust frontier refresh", uorustBin, ["project", "--out", tmpOut, "--register", "frontier.toml", "."], {
    quiet: true,
  });
  writeFileSync(frontierCacheFile, `${rustTreeSha}\n`);
} else if (!uorustBin) {
  console.log("[SKIP 0ms] uorust frontier refresh: uorust binary not in PATH (using committed frontier.toml)");
} else {
  console.log(`[SKIP 0ms] uorust frontier refresh: kappa hit (${rustTreeSha.slice(0, 12)}…)`);
}

// 5. Kappa-memoized Wasm rebuild
const wasmInputs = [
  join(root, "Cargo.toml"),
  join(root, "Cargo.lock"),
  ...walkFiles(join(root, "crates", "verdant-core", "src"), ".rs"),
  ...walkFiles(join(root, "crates", "verdant-tensor", "src"), ".rs"),
  ...walkFiles(join(root, "crates", "verdant-wasm", "src"), ".rs"),
  ...walkFiles(join(root, "catalogs")),
];
const wasmKappa = hashFiles(wasmInputs);
const wasmCacheFile = join(cacheDir, "wasm-inputs.sha256");
const prevWasmKappa = existsSync(wasmCacheFile) ? readFileSync(wasmCacheFile, "utf8").trim() : "";
const wasmPresent = existsSync(join(root, "site", "verdant-wasm.wasm")) && existsSync(join(root, "site", "verdant-uorc.wasm"));

if (force || wasmKappa !== prevWasmKappa || !wasmPresent) {
  run("sync-prism-wasm", process.execPath, ["scripts/sync-prism-wasm.mjs"], { quiet: true });
  writeFileSync(wasmCacheFile, `${wasmKappa}\n`);
} else {
  console.log(`[SKIP 0ms] sync-prism-wasm: kappa hit (${wasmKappa.slice(0, 12)}…)`);
}

// 6. Autonomous Verdant ratchet-lift (with one-way --tighten by default!)
const hasBump = cliArgs.some((a) => a === "--advance-major" || a === "--advance-minor" || a === "--advance-patch");
const liftArgs = ["run", "--locked", "-q", "-p", "verdant-cli", "--", "operator", "ratchet-lift", "--root", ".", "--out", "dist/stitch-release"];
if (noTighten || hasBump) liftArgs.push("--no-tighten");
for (let i = 0; i < cliArgs.length; i++) {
  const a = cliArgs[i];
  if (a === "--advance-major" || a === "--advance-minor" || a === "--advance-patch") {
    liftArgs.push(a);
  } else if (a === "--summary" && i + 1 < cliArgs.length) {
    liftArgs.push(a, cliArgs[++i]);
  }
}
const cargoBefore = readFileSync(join(root, "Cargo.toml"), "utf8");
run("verdant operator ratchet-lift", "cargo", liftArgs, { quiet: true });
const cargoAfter = readFileSync(join(root, "Cargo.toml"), "utf8");
if (cargoAfter !== cargoBefore || hasBump) {
  if (cargoAfter !== cargoBefore) {
    run("cargo update -w --offline (Cargo.lock sync)", "cargo", ["update", "-q", "-w", "--offline"], { quiet: true });
  }
  // Second pass without bump flags so manifests/KPIs/watermarks lock post-Cargo.lock state:
  const pass2 = ["run", "--locked", "-q", "-p", "verdant-cli", "--", "operator", "ratchet-lift", "--root", ".", "--out", "dist/stitch-release"];
  if (noTighten) pass2.push("--no-tighten");
  run("verdant operator ratchet-lift (post-sync & tighten)", "cargo", pass2, { quiet: true });
}

// 7. Final fail-closed `prism-fidelity --check` across ALL ratchet layers
run(
  "prism-fidelity --check (ALL ratchets)",
  "cargo",
  [
    "run",
    "--locked",
    "-q",
    "-p",
    "prism-fidelity",
    "--",
    "--project",
    "app",
    "--repo",
    ".",
    "--register",
    "fidelity.toml",
    "--ratchet",
    "model/parametric-ratchet.toml",
    "--cite",
    "model/provenance.toml",
    "app/src/Provenance.lex.tex",
    "--measures",
    "model/measures.toml",
    "--lean",
    "app/src/Fidelity.lex.tex",
    "--check",
  ],
  { quiet: true },
);

console.log(`verdant-ratchet: ALL RATCHETS LIFTED, TIGHTENED & VERIFIED in ${((Date.now() - tStart) / 1000).toFixed(1)}s`);
