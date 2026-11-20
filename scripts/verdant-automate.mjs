#!/usr/bin/env node
/**
 * Verdant Self-Automation & Kappa-Memoized Verification Orchestrator (`npm run automate`).
 *
 * Thin delegate into Verdant's own lifted in-Verdant automation pipeline
 * (`verdant operator automate`), so Verdant itself drives kappa-memoized Wasm builds,
 * in-process ratchet self-lifting, site & RFC 2789 Cargo Sparse Registry packaging,
 * in-process lifecycle-gate, and live browser & contract verification from within!
 *
 * Usage:
 *   node scripts/verdant-automate.mjs [--skip-rust] [--force-wasm]
 */

import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url));
const cliArgs = process.argv.slice(2);

const res = spawnSync(
  "cargo",
  ["run", "--locked", "-q", "-p", "verdant-cli", "--", "operator", "automate", "--root", ".", ...cliArgs],
  { cwd: root, stdio: "inherit", env: process.env },
);

if (res.status !== 0) {
  process.exit(res.status ?? 1);
}
