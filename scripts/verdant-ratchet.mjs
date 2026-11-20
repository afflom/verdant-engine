#!/usr/bin/env node
/**
 * Single-Command Autonomous Ratchet Orchestrator (`npm run ratchet`, `VE-55`, `VE-64`).
 *
 * Thin delegate into Verdant's own lifted in-Verdant ratchet-cycle orchestrator
 * (`verdant operator ratchet-cycle`), so Verdant itself executes xtask check-model,
 * prism-fidelity, kappa-memoized LexLean/uorust/Wasm rebuilds, in-process ratchet-lift
 * with one-way auto-tightening, and final fail-closed `prism-fidelity --check` from within!
 *
 * Usage:
 *   npm run ratchet [-- --advance-minor|--advance-major|--advance-patch] [--summary "..."] [--no-tighten] [--no-docker] [--force] [--with-gate] [--with-browser]
 */

import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url));
const cliArgs = process.argv.slice(2);

const res = spawnSync(
  "cargo",
  ["run", "--locked", "-q", "-p", "verdant-cli", "--", "operator", "ratchet-cycle", "--root", ".", ...cliArgs],
  { cwd: root, stdio: "inherit", env: process.env },
);

if (res.status !== 0) {
  process.exit(res.status ?? 1);
}
