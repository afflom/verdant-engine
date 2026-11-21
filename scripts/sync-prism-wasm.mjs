#!/usr/bin/env node
/**
 * Builds & verifies `site/verdant-wasm.wasm`, `site/verdant-uorc.wasm`, `site/scene-stream.uorc`,
 * and `site/prism-bridge-meta.mjs` natively inside Verdant (`verdant operator sync-wasm`).
 *
 * Thin delegate into Verdant's lifted native Rust build orchestrator (`build_ops.rs`).
 */

import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url));
const cliArgs = process.argv.slice(2);

const res = spawnSync(
  "cargo",
  ["run", "--locked", "-q", "-p", "verdant-cli", "--", "operator", "sync-wasm", "--root", ".", ...cliArgs],
  { cwd: root, stdio: "inherit", env: process.env },
);

if (res.status !== 0) {
  process.exit(res.status ?? 1);
}
